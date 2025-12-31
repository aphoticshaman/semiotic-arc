//! Memory System for Persistent Learning
//!
//! Three-tier memory architecture inspired by cognitive science:
//! 1. Working Memory: Current game context (fast, volatile)
//! 2. Episodic Memory: Past game experiences (medium, session-persistent)
//! 3. Semantic Memory: Learned abstractions (slow, cross-session)
//!
//! Self-healing through:
//! - Automatic consolidation (working → episodic → semantic)
//! - Forgetting curves (prune low-value memories)
//! - Reconstruction from fragments (graceful degradation)

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

use crate::grid::Grid;
use crate::dsl::RuleClass;
use crate::Action;

/// Working memory: current game context
/// Fast access, limited capacity, volatile
pub struct WorkingMemory {
    /// Recent observations (ring buffer)
    pub observations: VecDeque<Observation>,
    /// Current hypotheses being considered
    pub active_hypotheses: Vec<HypothesisState>,
    /// Actions taken this game
    pub action_history: Vec<ActionRecord>,
    /// Attention weights (what to focus on)
    pub attention: AttentionState,
    /// Capacity limit
    pub max_observations: usize,
}

#[derive(Clone, Debug)]
pub struct Observation {
    pub frame: Grid,
    pub timestamp: u64,  // Game tick
    pub delta_from_prev: Option<GridDelta>,
}

#[derive(Clone, Debug)]
pub struct GridDelta {
    pub changed_cells: Vec<(u8, u8, u8, u8)>,  // (x, y, old, new)
    pub color_changes: [i16; 10],
}

#[derive(Clone, Debug)]
pub struct HypothesisState {
    pub rule: RuleClass,
    pub confidence: f32,
    pub evidence_for: u32,
    pub evidence_against: u32,
    pub last_updated: u64,
}

#[derive(Clone, Debug)]
pub struct ActionRecord {
    pub action: Action,
    pub tick: u64,
    pub pre_state_hash: u64,
    pub post_state_hash: Option<u64>,
    pub outcome: ActionOutcome,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionOutcome {
    Unknown,
    Progress,      // Moved toward goal
    Neutral,       // No change
    Regression,    // Moved away from goal
    Terminal,      // Game ended
}

#[derive(Clone, Debug)]
pub struct AttentionState {
    /// Regions of interest (bounding boxes)
    pub focus_regions: Vec<(u8, u8, u8, u8)>,
    /// Colors of interest
    pub focus_colors: [f32; 10],
    /// Pattern features of interest
    pub focus_patterns: Vec<String>,
}

impl WorkingMemory {
    pub fn new() -> Self {
        Self {
            observations: VecDeque::with_capacity(64),
            active_hypotheses: Vec::with_capacity(16),
            action_history: Vec::with_capacity(256),
            attention: AttentionState {
                focus_regions: Vec::new(),
                focus_colors: [0.1; 10],
                focus_patterns: Vec::new(),
            },
            max_observations: 64,
        }
    }

    pub fn observe(&mut self, frame: Grid, tick: u64) {
        let delta = if let Some(prev) = self.observations.back() {
            Some(Self::compute_delta(&prev.frame, &frame))
        } else {
            None
        };

        self.observations.push_back(Observation {
            frame,
            timestamp: tick,
            delta_from_prev: delta,
        });

        // Maintain capacity
        while self.observations.len() > self.max_observations {
            self.observations.pop_front();
        }

        // Update attention based on changes - clone delta to avoid borrow conflict
        let delta_to_process = self.observations.back()
            .and_then(|obs| obs.delta_from_prev.clone());

        if let Some(d) = delta_to_process {
            self.update_attention(&d);
        }
    }

    pub fn record_action(&mut self, action: Action, tick: u64, pre_hash: u64) {
        self.action_history.push(ActionRecord {
            action,
            tick,
            pre_state_hash: pre_hash,
            post_state_hash: None,
            outcome: ActionOutcome::Unknown,
        });
    }

    pub fn update_action_outcome(&mut self, post_hash: u64, outcome: ActionOutcome) {
        if let Some(last) = self.action_history.last_mut() {
            last.post_state_hash = Some(post_hash);
            last.outcome = outcome;
        }
    }

    pub fn add_hypothesis(&mut self, rule: RuleClass, confidence: f32, tick: u64) {
        // Check if hypothesis already exists
        for h in &mut self.active_hypotheses {
            if h.rule == rule {
                h.confidence = (h.confidence + confidence) / 2.0;
                h.last_updated = tick;
                return;
            }
        }

        self.active_hypotheses.push(HypothesisState {
            rule,
            confidence,
            evidence_for: 0,
            evidence_against: 0,
            last_updated: tick,
        });

        // Keep top hypotheses only
        self.active_hypotheses.sort_by(|a, b| {
            b.confidence.partial_cmp(&a.confidence).unwrap()
        });
        self.active_hypotheses.truncate(16);
    }

    pub fn best_hypothesis(&self) -> Option<&HypothesisState> {
        self.active_hypotheses.first()
    }

    pub fn recent_observations(&self, n: usize) -> Vec<&Observation> {
        self.observations.iter().rev().take(n).collect()
    }

    pub fn clear(&mut self) {
        self.observations.clear();
        self.active_hypotheses.clear();
        self.action_history.clear();
    }

    fn compute_delta(prev: &Grid, curr: &Grid) -> GridDelta {
        let mut changed_cells = Vec::new();
        let mut color_changes = [0i16; 10];

        for y in 0..prev.height.max(curr.height) {
            for x in 0..prev.width.max(curr.width) {
                let old = prev.get(x, y);
                let new = curr.get(x, y);
                if old != new {
                    changed_cells.push((x, y, old, new));
                    if old < 10 {
                        color_changes[old as usize] -= 1;
                    }
                    if new < 10 {
                        color_changes[new as usize] += 1;
                    }
                }
            }
        }

        GridDelta {
            changed_cells,
            color_changes,
        }
    }

    fn update_attention(&mut self, delta: &GridDelta) {
        // Focus on changing regions
        if !delta.changed_cells.is_empty() {
            let min_x = delta.changed_cells.iter().map(|c| c.0).min().unwrap();
            let max_x = delta.changed_cells.iter().map(|c| c.0).max().unwrap();
            let min_y = delta.changed_cells.iter().map(|c| c.1).min().unwrap();
            let max_y = delta.changed_cells.iter().map(|c| c.1).max().unwrap();

            self.attention.focus_regions.push((min_x, min_y, max_x + 1, max_y + 1));
            if self.attention.focus_regions.len() > 8 {
                self.attention.focus_regions.remove(0);
            }
        }

        // Update color focus based on changes
        for (i, &change) in delta.color_changes.iter().enumerate() {
            if change != 0 {
                self.attention.focus_colors[i] =
                    (self.attention.focus_colors[i] + 0.3).min(1.0);
            } else {
                self.attention.focus_colors[i] =
                    (self.attention.focus_colors[i] - 0.05).max(0.1);
            }
        }
    }
}

impl Default for WorkingMemory {
    fn default() -> Self {
        Self::new()
    }
}

/// Episodic memory: past game experiences
/// Medium-term storage, session-persistent
pub struct EpisodicMemory {
    /// Past game episodes
    episodes: Vec<Episode>,
    /// Episode index by outcome
    by_outcome: HashMap<EpisodeOutcome, Vec<usize>>,
    /// Episode index by rule class
    by_rule: HashMap<String, Vec<usize>>,
    /// Maximum episodes to store
    max_episodes: usize,
}

#[derive(Clone, Debug)]
pub struct Episode {
    pub game_id: u64,
    pub rule_class: Option<RuleClass>,
    pub actions_taken: u32,
    pub budget_used: f32,  // Fraction of budget used
    pub outcome: EpisodeOutcome,
    pub key_moments: Vec<KeyMoment>,
    pub learned_patterns: Vec<String>,
    pub timestamp: Instant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EpisodeOutcome {
    Win,
    Loss,
    Timeout,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct KeyMoment {
    pub tick: u64,
    pub description: String,
    pub confidence_delta: f32,
    pub action: Option<Action>,
}

impl EpisodicMemory {
    pub fn new() -> Self {
        Self {
            episodes: Vec::with_capacity(1024),
            by_outcome: HashMap::new(),
            by_rule: HashMap::new(),
            max_episodes: 1024,
        }
    }

    pub fn record_episode(&mut self, episode: Episode) {
        let idx = self.episodes.len();

        // Index by outcome
        self.by_outcome
            .entry(episode.outcome)
            .or_insert_with(Vec::new)
            .push(idx);

        // Index by rule class
        if let Some(ref rule) = episode.rule_class {
            self.by_rule
                .entry(format!("{:?}", rule))
                .or_insert_with(Vec::new)
                .push(idx);
        }

        self.episodes.push(episode);

        // Prune old episodes if over capacity
        self.prune_if_needed();
    }

    pub fn similar_episodes(&self, rule: &RuleClass, n: usize) -> Vec<&Episode> {
        let key = format!("{:?}", rule);
        if let Some(indices) = self.by_rule.get(&key) {
            indices.iter()
                .rev()
                .take(n)
                .filter_map(|&i| self.episodes.get(i))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn winning_episodes(&self, n: usize) -> Vec<&Episode> {
        if let Some(indices) = self.by_outcome.get(&EpisodeOutcome::Win) {
            indices.iter()
                .rev()
                .take(n)
                .filter_map(|&i| self.episodes.get(i))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn win_rate(&self) -> f32 {
        if self.episodes.is_empty() {
            return 0.0;
        }
        let wins = self.by_outcome.get(&EpisodeOutcome::Win)
            .map(|v| v.len())
            .unwrap_or(0);
        wins as f32 / self.episodes.len() as f32
    }

    fn prune_if_needed(&mut self) {
        if self.episodes.len() <= self.max_episodes {
            return;
        }

        // Keep wins and recent episodes, prune old losses
        let cutoff = self.episodes.len() - self.max_episodes / 2;

        let mut keep = vec![false; self.episodes.len()];

        // Always keep wins
        if let Some(wins) = self.by_outcome.get(&EpisodeOutcome::Win) {
            for &idx in wins {
                keep[idx] = true;
            }
        }

        // Keep recent episodes
        for i in cutoff..self.episodes.len() {
            keep[i] = true;
        }

        // Rebuild
        let kept: Vec<Episode> = self.episodes
            .drain(..)
            .enumerate()
            .filter(|(i, _)| keep[*i])
            .map(|(_, e)| e)
            .collect();

        self.episodes = kept;

        // Rebuild indices
        self.by_outcome.clear();
        self.by_rule.clear();
        for (idx, ep) in self.episodes.iter().enumerate() {
            self.by_outcome
                .entry(ep.outcome)
                .or_insert_with(Vec::new)
                .push(idx);
            if let Some(ref rule) = ep.rule_class {
                self.by_rule
                    .entry(format!("{:?}", rule))
                    .or_insert_with(Vec::new)
                    .push(idx);
            }
        }
    }
}

impl Default for EpisodicMemory {
    fn default() -> Self {
        Self::new()
    }
}

/// Semantic memory: learned abstractions
/// Long-term storage, cross-session persistent
pub struct SemanticMemory {
    /// Rule class statistics
    rule_stats: HashMap<String, RuleStatistics>,
    /// Learned action preferences
    action_priors: HashMap<String, ActionPrior>,
    /// Pattern recognition cache
    pattern_cache: HashMap<u64, PatternMatch>,
    /// Consolidated insights
    insights: Vec<Insight>,
}

#[derive(Clone, Debug, Default)]
pub struct RuleStatistics {
    pub encounters: u32,
    pub wins: u32,
    pub avg_actions_to_solve: f32,
    pub best_first_action: Option<Action>,
    pub confidence: f32,
}

#[derive(Clone, Debug)]
pub struct ActionPrior {
    pub action: Action,
    pub success_rate: f32,
    pub sample_size: u32,
}

#[derive(Clone, Debug)]
pub struct PatternMatch {
    pub pattern_hash: u64,
    pub rule_class: RuleClass,
    pub confidence: f32,
}

#[derive(Clone, Debug)]
pub struct Insight {
    pub description: String,
    pub confidence: f32,
    pub source_episodes: Vec<u64>,
    pub validated: bool,
}

impl SemanticMemory {
    pub fn new() -> Self {
        Self {
            rule_stats: HashMap::new(),
            action_priors: HashMap::new(),
            pattern_cache: HashMap::new(),
            insights: Vec::new(),
        }
    }

    pub fn update_rule_stats(&mut self, rule: &RuleClass, won: bool, actions: u32) {
        let key = format!("{:?}", rule);
        let stats = self.rule_stats.entry(key).or_insert_with(RuleStatistics::default);

        stats.encounters += 1;
        if won {
            stats.wins += 1;
        }

        // Running average of actions to solve
        let n = stats.encounters as f32;
        stats.avg_actions_to_solve =
            (stats.avg_actions_to_solve * (n - 1.0) + actions as f32) / n;

        // Update confidence
        stats.confidence = stats.wins as f32 / stats.encounters as f32;
    }

    pub fn get_rule_stats(&self, rule: &RuleClass) -> Option<&RuleStatistics> {
        let key = format!("{:?}", rule);
        self.rule_stats.get(&key)
    }

    pub fn cache_pattern(&mut self, hash: u64, rule: RuleClass, confidence: f32) {
        self.pattern_cache.insert(hash, PatternMatch {
            pattern_hash: hash,
            rule_class: rule,
            confidence,
        });
    }

    pub fn lookup_pattern(&self, hash: u64) -> Option<&PatternMatch> {
        self.pattern_cache.get(&hash)
    }

    pub fn add_insight(&mut self, description: String, confidence: f32, sources: Vec<u64>) {
        self.insights.push(Insight {
            description,
            confidence,
            source_episodes: sources,
            validated: false,
        });
    }

    /// Consolidate episodic memories into semantic knowledge
    pub fn consolidate(&mut self, episodes: &[Episode]) {
        // Extract patterns from winning episodes
        let wins: Vec<_> = episodes.iter()
            .filter(|e| e.outcome == EpisodeOutcome::Win)
            .collect();

        if wins.len() < 3 {
            return;
        }

        // Find common patterns in winning episodes
        let mut pattern_counts: HashMap<String, u32> = HashMap::new();
        for ep in &wins {
            for pattern in &ep.learned_patterns {
                *pattern_counts.entry(pattern.clone()).or_insert(0) += 1;
            }
        }

        // Patterns that appear in >50% of wins become insights
        let threshold = wins.len() as u32 / 2;
        for (pattern, count) in pattern_counts {
            if count > threshold {
                let confidence = count as f32 / wins.len() as f32;
                self.add_insight(
                    pattern,
                    confidence,
                    wins.iter().map(|e| e.game_id).collect(),
                );
            }
        }
    }
}

impl Default for SemanticMemory {
    fn default() -> Self {
        Self::new()
    }
}

/// Unified memory system
pub struct MemorySystem {
    pub working: WorkingMemory,
    pub episodic: EpisodicMemory,
    pub semantic: SemanticMemory,
    /// Current game ID
    current_game: u64,
    /// Game counter
    game_counter: u64,
}

impl MemorySystem {
    pub fn new() -> Self {
        Self {
            working: WorkingMemory::new(),
            episodic: EpisodicMemory::new(),
            semantic: SemanticMemory::new(),
            current_game: 0,
            game_counter: 0,
        }
    }

    /// Start a new game
    pub fn new_game(&mut self) -> u64 {
        self.game_counter += 1;
        self.current_game = self.game_counter;
        self.working.clear();
        self.current_game
    }

    /// End current game and consolidate
    pub fn end_game(&mut self, outcome: EpisodeOutcome, rule: Option<RuleClass>) {
        let episode = Episode {
            game_id: self.current_game,
            rule_class: rule.clone(),
            actions_taken: self.working.action_history.len() as u32,
            budget_used: 0.0,  // TODO: track budget
            outcome,
            key_moments: Vec::new(),  // TODO: extract key moments
            learned_patterns: Vec::new(),  // TODO: extract patterns
            timestamp: Instant::now(),
        };

        // Update semantic memory
        if let Some(ref r) = rule {
            self.semantic.update_rule_stats(
                r,
                outcome == EpisodeOutcome::Win,
                episode.actions_taken,
            );
        }

        // Record episode
        self.episodic.record_episode(episode);

        // Periodic consolidation
        if self.game_counter % 10 == 0 {
            self.semantic.consolidate(&self.episodic.episodes);
        }
    }

    /// Get prior for a rule based on past experience
    pub fn get_prior(&self, rule: &RuleClass) -> f32 {
        self.semantic.get_rule_stats(rule)
            .map(|s| s.confidence)
            .unwrap_or(0.5)
    }
}

impl Default for MemorySystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working_memory() {
        let mut wm = WorkingMemory::new();
        let grid = Grid::new(5, 5);

        wm.observe(grid.clone(), 0);
        wm.observe(grid.clone(), 1);

        assert_eq!(wm.observations.len(), 2);
    }

    #[test]
    fn test_episodic_memory() {
        let mut em = EpisodicMemory::new();

        em.record_episode(Episode {
            game_id: 1,
            rule_class: None,
            actions_taken: 10,
            budget_used: 0.5,
            outcome: EpisodeOutcome::Win,
            key_moments: Vec::new(),
            learned_patterns: Vec::new(),
            timestamp: Instant::now(),
        });

        assert_eq!(em.win_rate(), 1.0);
    }

    #[test]
    fn test_memory_system() {
        let mut ms = MemorySystem::new();

        let game_id = ms.new_game();
        assert_eq!(game_id, 1);

        ms.end_game(EpisodeOutcome::Win, None);
        assert_eq!(ms.episodic.episodes.len(), 1);
    }
}
