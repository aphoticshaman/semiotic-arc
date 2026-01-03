//! Semiotic Reasoning Engine for Abstract Visual Tasks
//!
//! A constraint-first architecture for ARC-AGI and related benchmarks,
//! based on the dual-channel model of human visual cognition.
//!
//! ## Core Thesis
//!
//! Human visual reasoning exploits two parallel systems:
//! 1. **Evolved patterns** - Phylogenetically hardwired pattern detectors
//!    (edges, symmetry, enclosure, proximity, face detection)
//! 2. **Conditioned symbols** - Ontogenetically learned associations
//!    (color meanings, positional semantics, quantity symbolism)
//!
//! Furthermore, visual puzzles function as **compressed reality models**,
//! simulating physical, mathematical, or social phenomena.
//!
//! ## Architecture
//!
//! 1. Symmetry reduction (canonicalization)
//! 2. Prior bank lookup (amortized inference)
//! 3. Discrimination tree (hypothesis space partitioning)
//! 4. World simulation (forward model)
//! 5. Viability-preserving planning
//! 6. GF(2) linear algebra (specialized solver for toggle puzzles)
//! 7. Adaptive probing (information-theoretic action selection)
//! 8. Arena allocation (memory-efficient search)
//! 9. SIMD vectorization (parallel grid operations)
//! 10. Cognitive self-monitoring (distortion detection and correction)
//! 11. Three-tier memory (working/episodic/semantic)
//! 12. Critical analysis (epistemic rigor)
//! 13. Semiotic analysis (evolved + conditioned + reality models)
//!
//! ## References
//!
//! - Chollet, F. (2019). On the Measure of Intelligence. arXiv:1911.01547
//! - Peirce, C.S. (1931-58). Collected Papers of Charles Sanders Peirce
//! - Gibson, J.J. (1979). The Ecological Approach to Visual Perception
//! - Spelke, E.S. (2007). Core Knowledge. Developmental Science, 10(1)

#![allow(dead_code)]
#![allow(unused_variables)]

// Core modules
pub mod grid;
pub mod symmetry;
pub mod dsl;

// Inference modules
pub mod prior_bank;
pub mod discrimination;

// Planning modules
pub mod simulator;
pub mod planner;

// Optimization modules
pub mod simd;      // SIMD-accelerated grid operations
pub mod arena;     // Arena allocation for search
pub mod gf2;       // GF(2) linear algebra for toggle puzzles
pub mod probe;     // Adaptive probing / information-theoretic selection
pub mod phase;     // Phase transition detection

// Cognitive architecture
pub mod memory;                 // Three-tier memory system
pub mod state;                  // Behavioral state machine
pub mod cognitive_regulation;   // Self-monitoring and correction
pub mod critical_analysis;      // Epistemic rigor

// Specialized solvers
pub mod sokoban;    // IDA* solver with deadlock detection

// Semiotic analysis (core contribution)
pub mod semiotics;  // Evolved patterns + Conditioned symbols + Reality models

// Operator algebra (Gap 1 fix)
pub mod operator;   // GridOperator trait, compose(), invert()

// Hypothesis search (Gap 2 fix)
pub mod hypothesis; // Beam search over operator compositions

// Grid DSL grammar (Gap 3 fix)
pub mod grammar;    // Formal grammar for grid operations

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use grid::Grid;
use prior_bank::PriorBank;
use discrimination::DiscriminationTree;
use simulator::WorldSimulator;
use planner::Planner;
use memory::{MemorySystem, EpisodeOutcome, ActionOutcome};
use state::{StateManager, OperatingMode, BehaviorStateMachine, BehaviorState};
use cognitive_regulation::{SelfHealer, HealingReport, CorrectionAction, apply_corrections, should_pause};
use critical_analysis::{AdversarialAnalyzer, AdversarialAnalysis, CautionLevel};

/// The unified semiotic reasoning engine.
///
/// Implements a 13-layer cognitive architecture that explicitly models
/// both evolved and conditioned aspects of human visual reasoning.
pub struct SemioticEngine {
    // Core inference
    prior_bank: PriorBank,
    discrimination_tree: DiscriminationTree,
    simulator: WorldSimulator,
    planner: Planner,

    // Cognitive architecture
    memory: MemorySystem,
    state_manager: StateManager,
    self_monitor: SelfHealer,
    critical_analyzer: AdversarialAnalyzer,
    behavior: BehaviorStateMachine,

    // Budget tracking
    action_budget: u32,
    actions_taken: u32,
    tick: u64,

    // State
    current_hypothesis: Option<dsl::RuleClass>,
    confidence: f32,

    // Training examples for current task
    examples: Vec<(Grid, Grid)>,

    // Analysis results
    last_regulation_report: Option<HealingReport>,
    last_critical_analysis: Option<AdversarialAnalysis>,
}

impl SemioticEngine {
    pub fn new() -> Self {
        Self {
            prior_bank: PriorBank::new(),
            discrimination_tree: DiscriminationTree::new(),
            simulator: WorldSimulator::new(),
            planner: Planner::new(),
            memory: MemorySystem::new(),
            state_manager: StateManager::new(),
            self_monitor: SelfHealer::new(),
            critical_analyzer: AdversarialAnalyzer::new(),
            behavior: BehaviorStateMachine::new(),
            action_budget: 100,
            actions_taken: 0,
            tick: 0,
            current_hypothesis: None,
            confidence: 0.0,
            examples: Vec::new(),
            last_regulation_report: None,
            last_critical_analysis: None,
        }
    }

    /// Main decision function: observe frame, decide action.
    ///
    /// Implements the full 13-layer cognitive pipeline:
    /// 1. Observation recording
    /// 2. Critical analysis (epistemic check)
    /// 3. Symmetry reduction
    /// 4. Compression to abstraction
    /// 5. Prior bank lookup
    /// 6. Self-monitoring check
    /// 7. State-driven exploration
    /// 8. Discrimination probing
    /// 9. Hypothesis selection
    /// 10. Planning
    /// 11. Execution
    pub fn decide(&mut self, frame: &Grid) -> Action {
        self.tick += 1;

        // Layer 1: Observe and record
        self.memory.working.observe(frame.clone(), self.tick);

        // Layer 2: Critical analysis
        let critical_analysis = self.critical_analyzer.analyze(frame, &self.examples);
        self.last_critical_analysis = Some(critical_analysis.clone());

        let caution_modifier = match critical_analysis.caution {
            CautionLevel::Low => 1.0,
            CautionLevel::Medium => 0.9,
            CautionLevel::High => 0.7,
            CautionLevel::Extreme => 0.5,
        };

        // Layer 3: Symmetry reduction
        let canonical = symmetry::canonicalize(frame);

        // Layer 4: Compression
        let abstraction = self.compress(&canonical);

        // Layer 5: Prior bank lookup
        let hypothesis_dist = self.prior_bank.lookup(&abstraction);

        if let Some(ref hyp) = self.current_hypothesis {
            let prior_boost = self.memory.get_prior(hyp);
            self.confidence = hypothesis_dist.confidence() * caution_modifier * (0.5 + 0.5 * prior_boost);
        } else {
            self.confidence = hypothesis_dist.confidence() * caution_modifier;
        }

        // Layer 6: Self-monitoring
        let agent_state = self.state_manager.current().clone();
        let regulation_report = self.self_monitor.analyze(&agent_state, &self.memory.working);
        self.last_regulation_report = Some(regulation_report.clone());

        if !regulation_report.corrections.is_empty() {
            let mut mutable_state = agent_state.clone();
            apply_corrections(&mut mutable_state, &mut self.memory.working, &regulation_report.corrections);
            self.confidence = mutable_state.confidence;
        }

        if should_pause(&regulation_report.corrections) && self.actions_taken > 0 {
            return Action::Noop;
        }

        if regulation_report.trigger_recovery {
            self.state_manager.trigger_recovery();
            self.behavior.transition(BehaviorState::HandleError);
            self.current_hypothesis = None;
            self.confidence = 0.5;
        }

        // Layer 7: State-driven exploration
        let exploration_rate = self.state_manager.exploration_rate();
        let should_explore = rand_float() < exploration_rate;

        // Layer 8: Discrimination probing
        if hypothesis_dist.entropy() > CONFIDENCE_THRESHOLD || should_explore {
            self.behavior.transition(BehaviorState::Probe);
            if let Some(probe) = self.discrimination_tree.next_probe(&hypothesis_dist) {
                self.record_action(&probe);
                return probe;
            }
        }

        // Layer 9: Hypothesis selection
        self.behavior.transition(BehaviorState::Hypothesize);
        let best_hypothesis = hypothesis_dist.argmax();
        self.current_hypothesis = Some(best_hypothesis.clone());
        self.memory.working.add_hypothesis(best_hypothesis.clone(), self.confidence, self.tick);

        // Layer 10: Planning
        self.behavior.transition(BehaviorState::Execute);
        let plan = self.planner.plan(
            frame,
            &best_hypothesis,
            &self.simulator,
            self.action_budget - self.actions_taken,
        );

        // Layer 11: Execution
        if let Some(action) = plan.first() {
            self.record_action(action);
            action.clone()
        } else {
            Action::Noop
        }
    }

    /// Provide training examples for current task.
    pub fn set_examples(&mut self, examples: Vec<(Grid, Grid)>) {
        self.examples = examples;
    }

    /// Report outcome of last action (for learning).
    pub fn report_outcome(&mut self, succeeded: bool) {
        let outcome = if succeeded {
            ActionOutcome::Progress
        } else {
            ActionOutcome::Regression
        };

        self.memory.working.update_action_outcome(0, outcome);
        self.state_manager.update(
            self.confidence,
            true,
            Some(succeeded),
            &Grid::new(1, 1),
        );
    }

    /// End the current task.
    pub fn end_task(&mut self, won: bool) {
        let outcome = if won {
            EpisodeOutcome::Win
        } else {
            EpisodeOutcome::Loss
        };

        self.memory.end_game(outcome, self.current_hypothesis.clone());
        self.behavior.transition(BehaviorState::Done);

        if !won {
            if let Some(ref analysis) = self.last_critical_analysis {
                for trap in &analysis.traps {
                    self.critical_analyzer.record_failure(
                        format!("{:?}", trap.trap_type),
                        "Detection failure".to_string(),
                        trap.description.clone(),
                        trap.mitigation.clone(),
                    );
                }
            }
        }
    }

    /// Reset for new task.
    pub fn reset(&mut self, action_budget: u32) {
        self.action_budget = action_budget;
        self.actions_taken = 0;
        self.tick = 0;
        self.current_hypothesis = None;
        self.confidence = 0.0;
        self.examples.clear();
        self.last_regulation_report = None;
        self.last_critical_analysis = None;

        self.planner.reset();
        self.state_manager.reset(action_budget);
        self.self_monitor.reset();
        self.behavior.reset();
        self.memory.new_game();
    }

    /// Get current confidence level.
    pub fn get_confidence(&self) -> f32 {
        self.confidence
    }

    /// Get current hypothesis.
    pub fn get_hypothesis(&self) -> Option<&dsl::RuleClass> {
        self.current_hypothesis.as_ref()
    }

    /// Get last self-monitoring report.
    pub fn get_regulation_report(&self) -> Option<&HealingReport> {
        self.last_regulation_report.as_ref()
    }

    /// Get last critical analysis.
    pub fn get_critical_analysis(&self) -> Option<&AdversarialAnalysis> {
        self.last_critical_analysis.as_ref()
    }

    /// Get win rate across all tasks.
    pub fn get_win_rate(&self) -> f32 {
        self.memory.episodic.win_rate()
    }

    fn record_action(&mut self, action: &Action) {
        self.actions_taken += 1;
        self.memory.working.record_action(action.clone(), self.tick, 0);
        self.state_manager.update(self.confidence, true, None, &Grid::new(1, 1));
    }

    fn compress(&self, grid: &Grid) -> Abstraction {
        Abstraction {
            color_histogram: grid.color_histogram(),
            object_count: grid.connected_components().len() as u8,
            has_symmetry: symmetry::detect_symmetries(grid),
            bounding_box: grid.bounding_box(),
        }
    }
}

fn rand_float() -> f32 {
    0.5 // Placeholder - would use proper RNG
}

impl Default for SemioticEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Action that can be taken during task solving.
#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Move(Direction),
    Click(u8, u8),
    Special(u8),
    Noop,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Compressed representation of a grid.
#[derive(Clone, Debug)]
pub struct Abstraction {
    pub color_histogram: [u16; 10],
    pub object_count: u8,
    pub has_symmetry: symmetry::SymmetryFlags,
    pub bounding_box: (u8, u8, u8, u8),
}

const CONFIDENCE_THRESHOLD: f32 = 0.7;

// WASM bindings
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct WasmEngine {
    inner: SemioticEngine,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WasmEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self {
            inner: SemioticEngine::new(),
        }
    }

    #[wasm_bindgen]
    pub fn decide(&mut self, frame_json: &str) -> String {
        let frame: Grid = serde_json::from_str(frame_json).unwrap_or_default();
        let action = self.inner.decide(&frame);
        serde_json::to_string(&action).unwrap_or_default()
    }

    #[wasm_bindgen]
    pub fn reset(&mut self, budget: u32) {
        self.inner.reset(budget);
    }

    #[wasm_bindgen]
    pub fn set_examples(&mut self, examples_json: &str) {
        if let Ok(examples) = serde_json::from_str::<Vec<(Grid, Grid)>>(examples_json) {
            self.inner.set_examples(examples);
        }
    }

    #[wasm_bindgen]
    pub fn get_confidence(&self) -> f32 {
        self.inner.get_confidence()
    }

    #[wasm_bindgen]
    pub fn get_win_rate(&self) -> f32 {
        self.inner.get_win_rate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = SemioticEngine::new();
        assert_eq!(engine.actions_taken, 0);
    }
}
