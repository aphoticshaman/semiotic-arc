//! State Machine for Game Phase Management
//!
//! Inspired by VeilPath's variance classifier (solid/creative/edge)
//! and LatticeForge's phase transitions.
//!
//! The agent operates in different modes depending on:
//! - Confidence level (certain vs uncertain)
//! - Budget remaining (abundant vs scarce)
//! - Game phase (early/mid/late)
//! - Health of current strategy

use crate::phase::{PhaseDetector, PhaseState, SystemPhase};
use std::time::Instant;

/// Operating mode of the agent
/// Inspired by VeilPath variance classifier
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatingMode {
    /// High confidence, execute known strategy (75% of actions)
    Solid,
    /// Moderate confidence, try variations (15% of actions)
    Creative,
    /// Low confidence, explore novel approaches (10% of actions)
    Edge,
    /// Emergency mode - something is very wrong
    Recovery,
}

/// Game phase (early/mid/late)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GamePhase {
    /// Just started - explore and gather info
    Opening,
    /// Main gameplay - execute strategy
    Midgame,
    /// Low budget - must be efficient
    Endgame,
    /// Out of options - hail mary
    Desperate,
}

/// Strategy health assessment
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrategyHealth {
    /// Strategy is working
    Healthy,
    /// Strategy is stalling
    Stagnant,
    /// Strategy is failing
    Failing,
    /// No strategy identified
    Unknown,
}

/// Complete agent state
#[derive(Clone, Debug)]
pub struct AgentState {
    /// Current operating mode
    pub mode: OperatingMode,
    /// Current game phase
    pub phase: GamePhase,
    /// Strategy health
    pub health: StrategyHealth,
    /// Confidence in current approach (0-1)
    pub confidence: f32,
    /// Actions remaining
    pub budget_remaining: u32,
    /// Total budget for this game
    pub total_budget: u32,
    /// Consecutive failures
    pub failure_streak: u32,
    /// Consecutive successes
    pub success_streak: u32,
    /// Time in current mode
    pub mode_duration: u32,
    /// Last state transition
    pub last_transition: StateTransition,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StateTransition {
    None,
    ModeChange(OperatingMode, OperatingMode),
    PhaseChange(GamePhase, GamePhase),
    HealthChange(StrategyHealth, StrategyHealth),
    Recovery,
}

/// State manager - tracks and transitions agent state
pub struct StateManager {
    /// Current state
    state: AgentState,
    /// Phase detector (from phase.rs)
    phase_detector: PhaseDetector,
    /// State history for analysis
    history: Vec<AgentState>,
    /// Transition log
    transitions: Vec<(u32, StateTransition)>,
    /// Configuration
    config: StateConfig,
}

/// Configuration for state transitions
#[derive(Clone, Debug)]
pub struct StateConfig {
    /// Confidence threshold for Solid mode
    pub solid_threshold: f32,
    /// Confidence threshold for Creative mode
    pub creative_threshold: f32,
    /// Budget fraction for Opening phase
    pub opening_budget: f32,
    /// Budget fraction for Endgame phase
    pub endgame_budget: f32,
    /// Failures before declaring strategy Failing
    pub failure_threshold: u32,
    /// Stagnation ticks before declaring Stagnant
    pub stagnation_threshold: u32,
}

impl Default for StateConfig {
    fn default() -> Self {
        Self {
            solid_threshold: 0.75,
            creative_threshold: 0.40,
            opening_budget: 0.20,
            endgame_budget: 0.15,
            failure_threshold: 5,
            stagnation_threshold: 10,
        }
    }
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            state: AgentState {
                mode: OperatingMode::Creative,
                phase: GamePhase::Opening,
                health: StrategyHealth::Unknown,
                confidence: 0.5,
                budget_remaining: 100,
                total_budget: 100,
                failure_streak: 0,
                success_streak: 0,
                mode_duration: 0,
                last_transition: StateTransition::None,
            },
            phase_detector: PhaseDetector::new(),
            history: Vec::with_capacity(1000),
            transitions: Vec::with_capacity(100),
            config: StateConfig::default(),
        }
    }

    pub fn with_config(config: StateConfig) -> Self {
        let mut sm = Self::new();
        sm.config = config;
        sm
    }

    /// Reset for new game
    pub fn reset(&mut self, total_budget: u32) {
        self.state = AgentState {
            mode: OperatingMode::Creative,
            phase: GamePhase::Opening,
            health: StrategyHealth::Unknown,
            confidence: 0.5,
            budget_remaining: total_budget,
            total_budget,
            failure_streak: 0,
            success_streak: 0,
            mode_duration: 0,
            last_transition: StateTransition::None,
        };
        self.phase_detector.reset();
        self.history.clear();
        self.transitions.clear();
    }

    /// Update state based on new information
    pub fn update(
        &mut self,
        confidence: f32,
        action_taken: bool,
        action_succeeded: Option<bool>,
        grid: &crate::grid::Grid,
    ) {
        let tick = (self.state.total_budget - self.state.budget_remaining) as u32;

        // Update budget
        if action_taken {
            self.state.budget_remaining = self.state.budget_remaining.saturating_sub(1);
        }

        // Update confidence
        self.state.confidence = confidence;

        // Update streaks
        if let Some(succeeded) = action_succeeded {
            if succeeded {
                self.state.success_streak += 1;
                self.state.failure_streak = 0;
            } else {
                self.state.failure_streak += 1;
                self.state.success_streak = 0;
            }
        }

        // Analyze phase via phase detector
        let phase_state = self.phase_detector.analyze(grid);

        // Compute new state
        let new_mode = self.compute_mode(&phase_state);
        let new_phase = self.compute_phase();
        let new_health = self.compute_health(&phase_state);

        // Check for transitions
        if new_mode != self.state.mode {
            let transition = StateTransition::ModeChange(self.state.mode, new_mode);
            self.transitions.push((tick, transition));
            self.state.last_transition = transition;
            self.state.mode_duration = 0;
        } else {
            self.state.mode_duration += 1;
        }

        if new_phase != self.state.phase {
            let transition = StateTransition::PhaseChange(self.state.phase, new_phase);
            self.transitions.push((tick, transition));
            self.state.last_transition = transition;
        }

        if new_health != self.state.health {
            let transition = StateTransition::HealthChange(self.state.health, new_health);
            self.transitions.push((tick, transition));
            self.state.last_transition = transition;
        }

        // Apply new state
        self.state.mode = new_mode;
        self.state.phase = new_phase;
        self.state.health = new_health;

        // Record history
        self.history.push(self.state.clone());
    }

    /// Force recovery mode
    pub fn trigger_recovery(&mut self) {
        let tick = (self.state.total_budget - self.state.budget_remaining) as u32;
        self.state.mode = OperatingMode::Recovery;
        self.state.last_transition = StateTransition::Recovery;
        self.transitions.push((tick, StateTransition::Recovery));
        self.state.failure_streak = 0;
    }

    /// Get current state
    pub fn current(&self) -> &AgentState {
        &self.state
    }

    /// Get operating mode
    pub fn mode(&self) -> OperatingMode {
        self.state.mode
    }

    /// Get game phase
    pub fn phase(&self) -> GamePhase {
        self.state.phase
    }

    /// Should we explore or exploit?
    pub fn exploration_rate(&self) -> f32 {
        match self.state.mode {
            OperatingMode::Solid => 0.05,
            OperatingMode::Creative => 0.25,
            OperatingMode::Edge => 0.50,
            OperatingMode::Recovery => 0.75,
        }
    }

    /// How aggressive should we be?
    pub fn aggression(&self) -> f32 {
        match self.state.phase {
            GamePhase::Opening => 0.3,
            GamePhase::Midgame => 0.5,
            GamePhase::Endgame => 0.8,
            GamePhase::Desperate => 1.0,
        }
    }

    fn compute_mode(&self, phase_state: &PhaseState) -> OperatingMode {
        // Recovery mode takes precedence
        if self.state.failure_streak >= self.config.failure_threshold {
            return OperatingMode::Recovery;
        }

        // Map confidence to mode
        if self.state.confidence >= self.config.solid_threshold {
            OperatingMode::Solid
        } else if self.state.confidence >= self.config.creative_threshold {
            OperatingMode::Creative
        } else {
            OperatingMode::Edge
        }
    }

    fn compute_phase(&self) -> GamePhase {
        let budget_fraction = self.state.budget_remaining as f32 / self.state.total_budget as f32;

        if budget_fraction > (1.0 - self.config.opening_budget) {
            GamePhase::Opening
        } else if budget_fraction > self.config.endgame_budget {
            GamePhase::Midgame
        } else if budget_fraction > 0.05 {
            GamePhase::Endgame
        } else {
            GamePhase::Desperate
        }
    }

    fn compute_health(&self, phase_state: &PhaseState) -> StrategyHealth {
        // No hypothesis = unknown
        if self.state.confidence < 0.2 {
            return StrategyHealth::Unknown;
        }

        // Failing = many failures
        if self.state.failure_streak >= 3 {
            return StrategyHealth::Failing;
        }

        // Stagnant = long time in same mode without progress
        if self.state.mode_duration > self.config.stagnation_threshold
            && self.state.success_streak == 0
        {
            return StrategyHealth::Stagnant;
        }

        // Check phase state from phase detector
        match phase_state.phase {
            SystemPhase::Plasma => StrategyHealth::Unknown,
            SystemPhase::Nucleating => StrategyHealth::Stagnant,
            SystemPhase::Crystalline => StrategyHealth::Healthy,
            SystemPhase::Supercooled => StrategyHealth::Healthy,
            SystemPhase::Annealing => StrategyHealth::Healthy,
        }
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

/// State machine for specific behaviors
#[derive(Clone, Debug)]
pub struct BehaviorStateMachine {
    /// Current behavior state
    current: BehaviorState,
    /// Previous states (for backtracking)
    stack: Vec<BehaviorState>,
    /// Tick counter
    tick: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BehaviorState {
    /// Initial observation
    Observe,
    /// Hypothesis formation
    Hypothesize,
    /// Testing hypothesis via probing
    Probe,
    /// Executing known strategy
    Execute,
    /// Verifying solution
    Verify,
    /// Handling unexpected state
    HandleError,
    /// Terminal state
    Done,
}

impl BehaviorStateMachine {
    pub fn new() -> Self {
        Self {
            current: BehaviorState::Observe,
            stack: Vec::with_capacity(32),
            tick: 0,
        }
    }

    pub fn current(&self) -> BehaviorState {
        self.current
    }

    pub fn transition(&mut self, new_state: BehaviorState) {
        self.stack.push(self.current);
        self.current = new_state;
        self.tick = 0;
    }

    pub fn revert(&mut self) -> bool {
        if let Some(prev) = self.stack.pop() {
            self.current = prev;
            self.tick = 0;
            true
        } else {
            false
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
    }

    pub fn ticks_in_state(&self) -> u32 {
        self.tick
    }

    pub fn reset(&mut self) {
        self.current = BehaviorState::Observe;
        self.stack.clear();
        self.tick = 0;
    }
}

impl Default for BehaviorStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Grid;

    #[test]
    fn test_state_manager() {
        let mut sm = StateManager::new();
        sm.reset(100);

        assert_eq!(sm.mode(), OperatingMode::Creative);
        assert_eq!(sm.phase(), GamePhase::Opening);
    }

    #[test]
    fn test_mode_transitions() {
        let mut sm = StateManager::new();
        sm.reset(100);

        let grid = Grid::new(5, 5);

        // High confidence -> Solid mode
        sm.update(0.9, false, None, &grid);
        assert_eq!(sm.mode(), OperatingMode::Solid);

        // Low confidence -> Edge mode
        sm.update(0.2, false, None, &grid);
        assert_eq!(sm.mode(), OperatingMode::Edge);
    }

    #[test]
    fn test_phase_transitions() {
        let mut sm = StateManager::new();
        sm.reset(100);

        let grid = Grid::new(5, 5);

        // Opening phase
        assert_eq!(sm.phase(), GamePhase::Opening);

        // Consume budget to reach midgame
        for _ in 0..25 {
            sm.update(0.5, true, None, &grid);
        }
        assert_eq!(sm.phase(), GamePhase::Midgame);

        // Consume more to reach endgame
        for _ in 0..65 {
            sm.update(0.5, true, None, &grid);
        }
        assert_eq!(sm.phase(), GamePhase::Endgame);
    }

    #[test]
    fn test_behavior_state_machine() {
        let mut bsm = BehaviorStateMachine::new();

        assert_eq!(bsm.current(), BehaviorState::Observe);

        bsm.transition(BehaviorState::Hypothesize);
        assert_eq!(bsm.current(), BehaviorState::Hypothesize);

        bsm.transition(BehaviorState::Execute);
        assert_eq!(bsm.current(), BehaviorState::Execute);

        // Revert
        bsm.revert();
        assert_eq!(bsm.current(), BehaviorState::Hypothesize);
    }
}
