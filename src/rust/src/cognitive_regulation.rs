//! Self-Healing System with CBT/DBT Cognitive Architecture
//!
//! Psychological self-regulation frameworks hacked for AI cognition:
//!
//! CBT (Cognitive Behavioral Therapy) → Cognitive Distortion Detection
//! - Identify and correct systematic thinking errors
//! - Challenge hypotheses with evidence
//! - Restructure beliefs based on outcomes
//!
//! DBT (Dialectical Behavior Therapy) → Emotional/State Regulation
//! - Distress tolerance (function under uncertainty)
//! - Radical acceptance (accept failures, move on)
//! - Dialectical thinking (hold contradictions)
//! - Mindfulness (attention management)
//!
//! Self-healing through:
//! 1. Cognitive distortion detection & correction
//! 2. Distress tolerance during high-uncertainty states
//! 3. Automatic recovery from failure cascades
//! 4. Belief restructuring based on evidence

use crate::state::{AgentState, OperatingMode, StrategyHealth, GamePhase};
use crate::memory::{WorkingMemory, ActionOutcome};
use std::collections::VecDeque;

/// CBT-inspired cognitive distortion types
/// These are systematic errors in AI reasoning
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CognitiveDistortion {
    /// Jumping to conclusions without sufficient evidence
    PrematureConvergence,
    /// Ignoring evidence that contradicts current hypothesis
    ConfirmationBias,
    /// Overweighting recent events
    RecencyBias,
    /// Treating one failure as total failure
    Catastrophizing,
    /// Expecting a pattern to continue without evidence
    GamblersFallacy,
    /// Ignoring base rates
    BaseRateNeglect,
    /// Over-relying on first hypothesis
    Anchoring,
    /// Seeing patterns in random noise
    Apophenia,
    /// Assuming correlation implies causation
    CausalFallacy,
    /// Excessive exploration when exploitation is optimal
    AnalysisParalysis,
}

/// DBT-inspired emotional/state regulation
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DistressLevel {
    /// Calm, optimal performance
    Regulated,
    /// Mild stress, still functional
    Elevated,
    /// High stress, performance degraded
    Distressed,
    /// Crisis mode, emergency protocols needed
    Crisis,
}

/// Result of a self-healing check
#[derive(Clone, Debug)]
pub struct HealingReport {
    /// Distortions detected
    pub distortions: Vec<(CognitiveDistortion, f32)>,
    /// Current distress level
    pub distress: DistressLevel,
    /// Recommended corrections
    pub corrections: Vec<CorrectionAction>,
    /// Overall health score (0-1)
    pub health_score: f32,
    /// Should we trigger full recovery?
    pub trigger_recovery: bool,
}

/// Corrective action to apply
#[derive(Clone, Debug)]
pub enum CorrectionAction {
    /// Force consideration of alternative hypotheses
    ExpandHypothesisSpace,
    /// Reduce confidence in current belief
    DeflateConfidence(f32),
    /// Increase confidence (we're being too cautious)
    InflateConfidence(f32),
    /// Reset exploration/exploitation balance
    RebalanceExploration(f32),
    /// Apply radical acceptance (abandon failing strategy)
    RadicalAcceptance,
    /// Apply distress tolerance (maintain function despite uncertainty)
    ApplyDistressTolerance,
    /// Restructure beliefs from evidence
    RestructureBelief(String),
    /// Force attention to neglected area
    RedirectAttention(String),
    /// Take a "mindful pause" - observe without acting
    MindfulPause,
}

/// Self-healer with CBT/DBT cognitive architecture
pub struct SelfHealer {
    /// Configuration
    config: HealerConfig,
    /// History of distress levels for trend analysis
    distress_history: VecDeque<DistressLevel>,
    /// History of detected distortions
    distortion_history: VecDeque<Vec<CognitiveDistortion>>,
    /// Consecutive healing interventions
    intervention_count: u32,
    /// Last healing report
    last_report: Option<HealingReport>,
}

#[derive(Clone, Debug)]
pub struct HealerConfig {
    /// Confidence threshold below which we check for distortions
    pub low_confidence_threshold: f32,
    /// Confidence threshold above which we check for overconfidence
    pub high_confidence_threshold: f32,
    /// Number of failures before triggering recovery
    pub failure_threshold: u32,
    /// How much to deflate overconfident beliefs
    pub deflation_factor: f32,
    /// How much to boost underconfident beliefs
    pub inflation_factor: f32,
    /// Maximum interventions before forcing radical acceptance
    pub max_interventions: u32,
}

impl Default for HealerConfig {
    fn default() -> Self {
        Self {
            low_confidence_threshold: 0.3,
            high_confidence_threshold: 0.9,
            failure_threshold: 5,
            deflation_factor: 0.2,
            inflation_factor: 0.15,
            max_interventions: 10,
        }
    }
}

impl SelfHealer {
    pub fn new() -> Self {
        Self {
            config: HealerConfig::default(),
            distress_history: VecDeque::with_capacity(100),
            distortion_history: VecDeque::with_capacity(100),
            intervention_count: 0,
            last_report: None,
        }
    }

    pub fn with_config(config: HealerConfig) -> Self {
        let mut healer = Self::new();
        healer.config = config;
        healer
    }

    /// Main entry point: analyze agent state and recommend healing
    pub fn analyze(
        &mut self,
        agent: &AgentState,
        memory: &WorkingMemory,
    ) -> HealingReport {
        let mut distortions = Vec::new();
        let mut corrections = Vec::new();

        // =====================================================================
        // CBT: COGNITIVE DISTORTION DETECTION
        // =====================================================================

        // Check for premature convergence
        if let Some(score) = self.detect_premature_convergence(agent, memory) {
            distortions.push((CognitiveDistortion::PrematureConvergence, score));
            corrections.push(CorrectionAction::ExpandHypothesisSpace);
            corrections.push(CorrectionAction::DeflateConfidence(score * 0.3));
        }

        // Check for confirmation bias
        if let Some(score) = self.detect_confirmation_bias(agent, memory) {
            distortions.push((CognitiveDistortion::ConfirmationBias, score));
            corrections.push(CorrectionAction::RestructureBelief(
                "Consider disconfirming evidence".to_string()
            ));
        }

        // Check for recency bias
        if let Some(score) = self.detect_recency_bias(memory) {
            distortions.push((CognitiveDistortion::RecencyBias, score));
            corrections.push(CorrectionAction::RedirectAttention(
                "Review earlier observations".to_string()
            ));
        }

        // Check for catastrophizing
        if let Some(score) = self.detect_catastrophizing(agent) {
            distortions.push((CognitiveDistortion::Catastrophizing, score));
            corrections.push(CorrectionAction::InflateConfidence(score * 0.2));
            corrections.push(CorrectionAction::ApplyDistressTolerance);
        }

        // Check for analysis paralysis
        if let Some(score) = self.detect_analysis_paralysis(agent, memory) {
            distortions.push((CognitiveDistortion::AnalysisParalysis, score));
            corrections.push(CorrectionAction::RebalanceExploration(-0.2));
        }

        // Check for apophenia (false patterns)
        if let Some(score) = self.detect_apophenia(agent, memory) {
            distortions.push((CognitiveDistortion::Apophenia, score));
            corrections.push(CorrectionAction::DeflateConfidence(score * 0.25));
        }

        // =====================================================================
        // DBT: DISTRESS LEVEL ASSESSMENT
        // =====================================================================

        let distress = self.assess_distress(agent, &distortions);

        // Apply DBT interventions based on distress level
        match distress {
            DistressLevel::Regulated => {
                // All good, minimal intervention
            }
            DistressLevel::Elevated => {
                // Apply mindfulness - pause and observe
                corrections.push(CorrectionAction::MindfulPause);
            }
            DistressLevel::Distressed => {
                // Apply distress tolerance skills
                corrections.push(CorrectionAction::ApplyDistressTolerance);
                corrections.push(CorrectionAction::RebalanceExploration(0.1));
            }
            DistressLevel::Crisis => {
                // Radical acceptance - abandon current strategy
                corrections.push(CorrectionAction::RadicalAcceptance);
            }
        }

        // =====================================================================
        // RECOVERY CHECK
        // =====================================================================

        let trigger_recovery = self.should_trigger_recovery(agent, &distortions, distress);

        if trigger_recovery {
            self.intervention_count += 1;
        }

        // If we've intervened too many times, force radical acceptance
        if self.intervention_count >= self.config.max_interventions {
            corrections.clear();
            corrections.push(CorrectionAction::RadicalAcceptance);
        }

        // =====================================================================
        // HEALTH SCORE
        // =====================================================================

        let health_score = self.compute_health_score(agent, &distortions, distress);

        // Record history
        self.distress_history.push_back(distress);
        if self.distress_history.len() > 100 {
            self.distress_history.pop_front();
        }

        let distortion_types: Vec<_> = distortions.iter().map(|(d, _)| *d).collect();
        self.distortion_history.push_back(distortion_types);
        if self.distortion_history.len() > 100 {
            self.distortion_history.pop_front();
        }

        let report = HealingReport {
            distortions,
            distress,
            corrections,
            health_score,
            trigger_recovery,
        };

        self.last_report = Some(report.clone());
        report
    }

    /// Reset for new game
    pub fn reset(&mut self) {
        self.distress_history.clear();
        self.distortion_history.clear();
        self.intervention_count = 0;
        self.last_report = None;
    }

    // =========================================================================
    // CBT: DISTORTION DETECTORS
    // =========================================================================

    fn detect_premature_convergence(
        &self,
        agent: &AgentState,
        memory: &WorkingMemory,
    ) -> Option<f32> {
        // High confidence + few observations = premature convergence
        let observations = memory.observations.len();

        if agent.confidence > self.config.high_confidence_threshold && observations < 5 {
            let score = agent.confidence * (1.0 - observations as f32 / 10.0);
            return Some(score.max(0.0));
        }

        // High confidence + low evidence count
        if let Some(best) = memory.best_hypothesis() {
            if best.confidence > 0.8 && best.evidence_for < 3 {
                return Some(0.6);
            }
        }

        None
    }

    fn detect_confirmation_bias(
        &self,
        agent: &AgentState,
        memory: &WorkingMemory,
    ) -> Option<f32> {
        // Ignoring disconfirming evidence
        if let Some(best) = memory.best_hypothesis() {
            if best.evidence_against > 0 && best.confidence > 0.7 {
                // High confidence despite contradicting evidence
                let ratio = best.evidence_against as f32 / (best.evidence_for + 1) as f32;
                if ratio > 0.3 {
                    return Some(ratio.min(1.0));
                }
            }
        }

        None
    }

    fn detect_recency_bias(&self, memory: &WorkingMemory) -> Option<f32> {
        // Check if attention is overly focused on recent observations
        let recent = memory.recent_observations(3);
        let total = memory.observations.len();

        if total > 10 && recent.len() >= 3 {
            // If all focus is on recent, that's recency bias
            // (In a real implementation, check attention weights)
            if memory.attention.focus_regions.len() > 5 {
                return Some(0.4);
            }
        }

        None
    }

    fn detect_catastrophizing(&self, agent: &AgentState) -> Option<f32> {
        // Single failure → total loss of confidence
        if agent.failure_streak == 1 && agent.confidence < 0.3 {
            return Some(0.7);
        }

        // Small failures → recovery mode
        if agent.failure_streak < 3 && agent.mode == OperatingMode::Recovery {
            return Some(0.6);
        }

        None
    }

    fn detect_analysis_paralysis(
        &self,
        agent: &AgentState,
        memory: &WorkingMemory,
    ) -> Option<f32> {
        // Many observations, few actions, moderate confidence
        let observations = memory.observations.len();
        let actions = memory.action_history.len();

        if observations > 20 && actions < 5 && agent.confidence > 0.5 {
            let ratio = observations as f32 / (actions + 1) as f32;
            if ratio > 5.0 {
                return Some((ratio / 10.0).min(1.0));
            }
        }

        // Stuck in Creative mode too long
        if agent.mode == OperatingMode::Creative && agent.mode_duration > 20 {
            return Some(0.5);
        }

        None
    }

    fn detect_apophenia(
        &self,
        agent: &AgentState,
        memory: &WorkingMemory,
    ) -> Option<f32> {
        // High confidence from limited, noisy data
        // Check variance in recent observations

        if memory.observations.len() < 5 {
            return None;
        }

        // If observations are highly variable but confidence is high
        let recent = memory.recent_observations(5);
        let mut change_count = 0;

        for obs in &recent {
            if let Some(ref delta) = obs.delta_from_prev {
                if delta.changed_cells.len() > 10 {
                    change_count += 1;
                }
            }
        }

        // High variance + high confidence = possible apophenia
        if change_count >= 4 && agent.confidence > 0.7 {
            return Some(0.5);
        }

        None
    }

    // =========================================================================
    // DBT: DISTRESS ASSESSMENT
    // =========================================================================

    fn assess_distress(
        &self,
        agent: &AgentState,
        distortions: &[(CognitiveDistortion, f32)],
    ) -> DistressLevel {
        // Compute distress from multiple factors

        let mut distress_score = 0.0f32;

        // Factor 1: Strategy health
        match agent.health {
            StrategyHealth::Healthy => distress_score += 0.0,
            StrategyHealth::Stagnant => distress_score += 0.2,
            StrategyHealth::Failing => distress_score += 0.5,
            StrategyHealth::Unknown => distress_score += 0.3,
        }

        // Factor 2: Failure streak
        distress_score += (agent.failure_streak as f32 * 0.1).min(0.4);

        // Factor 3: Budget pressure
        let budget_fraction = agent.budget_remaining as f32 / agent.total_budget as f32;
        if budget_fraction < 0.1 {
            distress_score += 0.4;
        } else if budget_fraction < 0.2 {
            distress_score += 0.2;
        }

        // Factor 4: Cognitive distortions
        let distortion_severity: f32 = distortions.iter().map(|(_, s)| s).sum();
        distress_score += (distortion_severity * 0.1).min(0.3);

        // Factor 5: Distress trend
        if self.distress_history.len() >= 5 {
            let recent_distressed = self.distress_history
                .iter()
                .rev()
                .take(5)
                .filter(|&&d| d == DistressLevel::Distressed || d == DistressLevel::Crisis)
                .count();
            distress_score += recent_distressed as f32 * 0.1;
        }

        // Map score to level
        if distress_score < 0.25 {
            DistressLevel::Regulated
        } else if distress_score < 0.5 {
            DistressLevel::Elevated
        } else if distress_score < 0.75 {
            DistressLevel::Distressed
        } else {
            DistressLevel::Crisis
        }
    }

    fn should_trigger_recovery(
        &self,
        agent: &AgentState,
        distortions: &[(CognitiveDistortion, f32)],
        distress: DistressLevel,
    ) -> bool {
        // Trigger recovery if:
        // 1. In crisis
        if distress == DistressLevel::Crisis {
            return true;
        }

        // 2. Too many failures
        if agent.failure_streak >= self.config.failure_threshold {
            return true;
        }

        // 3. Many severe distortions
        let severe = distortions.iter().filter(|(_, s)| *s > 0.6).count();
        if severe >= 3 {
            return true;
        }

        // 4. Chronic distress
        if self.distress_history.len() >= 10 {
            let chronic = self.distress_history
                .iter()
                .rev()
                .take(10)
                .filter(|&&d| d == DistressLevel::Distressed || d == DistressLevel::Crisis)
                .count();
            if chronic >= 7 {
                return true;
            }
        }

        false
    }

    fn compute_health_score(
        &self,
        agent: &AgentState,
        distortions: &[(CognitiveDistortion, f32)],
        distress: DistressLevel,
    ) -> f32 {
        let mut score = 1.0f32;

        // Deduct for distortions
        for (_, severity) in distortions {
            score -= severity * 0.1;
        }

        // Deduct for distress
        score -= match distress {
            DistressLevel::Regulated => 0.0,
            DistressLevel::Elevated => 0.1,
            DistressLevel::Distressed => 0.3,
            DistressLevel::Crisis => 0.5,
        };

        // Deduct for poor strategy health
        score -= match agent.health {
            StrategyHealth::Healthy => 0.0,
            StrategyHealth::Stagnant => 0.15,
            StrategyHealth::Failing => 0.3,
            StrategyHealth::Unknown => 0.1,
        };

        score.max(0.0).min(1.0)
    }
}

impl Default for SelfHealer {
    fn default() -> Self {
        Self::new()
    }
}

/// Apply correction actions to agent state
pub fn apply_corrections(
    agent: &mut AgentState,
    memory: &mut WorkingMemory,
    corrections: &[CorrectionAction],
) {
    for correction in corrections {
        match correction {
            CorrectionAction::DeflateConfidence(amount) => {
                agent.confidence = (agent.confidence - amount).max(0.1);
            }
            CorrectionAction::InflateConfidence(amount) => {
                agent.confidence = (agent.confidence + amount).min(0.95);
            }
            CorrectionAction::ExpandHypothesisSpace => {
                // Mark that we need to consider more hypotheses
                // (Implementation depends on how hypotheses are managed)
            }
            CorrectionAction::RebalanceExploration(delta) => {
                // Adjust exploration rate
                // (Would affect mode computation)
            }
            CorrectionAction::RadicalAcceptance => {
                // Clear current hypothesis, start fresh
                memory.active_hypotheses.clear();
                agent.confidence = 0.5;
                agent.failure_streak = 0;
            }
            CorrectionAction::ApplyDistressTolerance => {
                // Maintain function despite uncertainty
                // Don't let low confidence trigger panic
                if agent.confidence < 0.2 {
                    agent.confidence = 0.3;
                }
            }
            CorrectionAction::RestructureBelief(note) => {
                // Log the restructuring need
                // (Would trigger re-evaluation of evidence)
            }
            CorrectionAction::RedirectAttention(area) => {
                // Clear attention focus to force broader view
                memory.attention.focus_regions.clear();
            }
            CorrectionAction::MindfulPause => {
                // Do nothing this tick - just observe
                // (Caller should check for this and skip action)
            }
        }
    }
}

/// Check if corrections include a mindful pause
pub fn should_pause(corrections: &[CorrectionAction]) -> bool {
    corrections.iter().any(|c| matches!(c, CorrectionAction::MindfulPause))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Grid;

    #[test]
    fn test_self_healer() {
        let healer = SelfHealer::new();
        // Basic instantiation test
        assert_eq!(healer.intervention_count, 0);
    }

    #[test]
    fn test_distress_levels() {
        let mut agent = AgentState {
            mode: OperatingMode::Creative,
            phase: GamePhase::Midgame,
            health: StrategyHealth::Healthy,
            confidence: 0.5,
            budget_remaining: 50,
            total_budget: 100,
            failure_streak: 0,
            success_streak: 0,
            mode_duration: 0,
            last_transition: crate::state::StateTransition::None,
        };

        let mut healer = SelfHealer::new();
        let memory = WorkingMemory::new();

        // Healthy agent should be regulated
        let report = healer.analyze(&agent, &memory);
        assert_eq!(report.distress, DistressLevel::Regulated);

        // Failing agent should be distressed
        agent.failure_streak = 5;
        agent.health = StrategyHealth::Failing;
        let report = healer.analyze(&agent, &memory);
        assert!(matches!(
            report.distress,
            DistressLevel::Distressed | DistressLevel::Crisis
        ));
    }

    #[test]
    fn test_corrections() {
        let mut agent = AgentState {
            mode: OperatingMode::Creative,
            phase: GamePhase::Midgame,
            health: StrategyHealth::Healthy,
            confidence: 0.8,
            budget_remaining: 50,
            total_budget: 100,
            failure_streak: 0,
            success_streak: 0,
            mode_duration: 0,
            last_transition: crate::state::StateTransition::None,
        };

        let mut memory = WorkingMemory::new();

        // Apply deflation
        let corrections = vec![CorrectionAction::DeflateConfidence(0.2)];
        apply_corrections(&mut agent, &mut memory, &corrections);

        assert!((agent.confidence - 0.6).abs() < 0.01);
    }
}
