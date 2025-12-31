//! Discrimination Tree for Rule Classification
//!
//! Layer 4 of the architecture.
//!
//! When the prior bank is uncertain, we need to probe.
//! The discrimination tree finds the optimal probe sequence
//! that maximizes information gain per action.
//!
//! Key insight: 2 probes can distinguish 4 classes.
//! 3 probes can distinguish 8 classes.
//! We don't need to try everything - just the discriminating actions.

use crate::dsl::RuleClass;
use crate::prior_bank::HypothesisDistribution;
use crate::Action;

/// A node in the discrimination tree
#[derive(Clone, Debug)]
pub struct DiscriminationNode {
    /// Probe action to take at this node
    pub probe: Action,

    /// Expected outcomes and their successor nodes
    pub branches: Vec<(Outcome, Box<DiscriminationNode>)>,

    /// If this is a leaf, the determined rule class
    pub conclusion: Option<RuleClass>,

    /// Expected information gain from this probe
    pub info_gain: f32,
}

/// Possible outcomes of a probe action
#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    /// No change observed
    NoChange,
    /// Single cell changed
    SingleCellChange,
    /// Cross pattern changed (5 cells)
    CrossChange,
    /// 3x3 neighborhood changed (9 cells)
    NeighborhoodChange,
    /// Row changed
    RowChange,
    /// Column changed
    ColumnChange,
    /// Player moved
    PlayerMoved,
    /// Player blocked
    PlayerBlocked,
    /// Any other change
    Other,
}

/// The discrimination tree engine
pub struct DiscriminationTree {
    /// Root nodes for different game archetypes
    roots: Vec<DiscriminationNode>,

    /// Precomputed optimal probe sequences
    probe_sequences: Vec<ProbeSequence>,
}

/// A sequence of probes with expected outcomes
#[derive(Clone, Debug)]
pub struct ProbeSequence {
    pub probes: Vec<Action>,
    pub distinguishes: Vec<RuleClass>,
}

impl DiscriminationTree {
    pub fn new() -> Self {
        Self {
            roots: Self::build_default_tree(),
            probe_sequences: Self::build_probe_sequences(),
        }
    }

    /// Build the default discrimination tree
    fn build_default_tree() -> Vec<DiscriminationNode> {
        use crate::dsl::{NavigationRule, ToggleRule};

        // Level 1: Movement vs Click games
        let movement_probe = DiscriminationNode {
            probe: Action::Move(crate::Direction::Up),
            branches: vec![
                // Player moved → Navigation game
                (
                    Outcome::PlayerMoved,
                    Box::new(DiscriminationNode {
                        probe: Action::Noop,
                        branches: vec![],
                        conclusion: Some(RuleClass::Navigation(NavigationRule::SimpleWalls)),
                        info_gain: 0.0,
                    }),
                ),
                // No change → Click game or blocked
                (
                    Outcome::NoChange,
                    Box::new(Self::build_click_subtree()),
                ),
            ],
            conclusion: None,
            info_gain: 1.0, // High info gain for first probe
        };

        vec![movement_probe]
    }

    /// Build subtree for click-based games
    fn build_click_subtree() -> DiscriminationNode {
        use crate::dsl::ToggleRule;

        // Probe: click at center of grid
        DiscriminationNode {
            probe: Action::Click(32, 32),
            branches: vec![
                (
                    Outcome::SingleCellChange,
                    Box::new(DiscriminationNode {
                        probe: Action::Noop,
                        branches: vec![],
                        conclusion: Some(RuleClass::Toggle(ToggleRule::SingleCell)),
                        info_gain: 0.0,
                    }),
                ),
                (
                    Outcome::CrossChange,
                    Box::new(DiscriminationNode {
                        probe: Action::Noop,
                        branches: vec![],
                        conclusion: Some(RuleClass::Toggle(ToggleRule::CrossPattern)),
                        info_gain: 0.0,
                    }),
                ),
                (
                    Outcome::NeighborhoodChange,
                    Box::new(DiscriminationNode {
                        probe: Action::Noop,
                        branches: vec![],
                        conclusion: Some(RuleClass::Toggle(ToggleRule::Neighborhood3x3)),
                        info_gain: 0.0,
                    }),
                ),
            ],
            conclusion: None,
            info_gain: 0.8,
        }
    }

    /// Build precomputed optimal probe sequences
    fn build_probe_sequences() -> Vec<ProbeSequence> {
        use crate::dsl::{NavigationRule, ToggleRule};

        vec![
            // Sequence 1: Distinguish navigation types
            ProbeSequence {
                probes: vec![
                    Action::Move(crate::Direction::Up),
                    Action::Move(crate::Direction::Down),
                ],
                distinguishes: vec![
                    RuleClass::Navigation(NavigationRule::SimpleWalls),
                    RuleClass::Navigation(NavigationRule::Wraparound),
                ],
            },
            // Sequence 2: Distinguish toggle types
            ProbeSequence {
                probes: vec![
                    Action::Click(32, 32),
                    Action::Click(32, 32), // Click same spot twice
                ],
                distinguishes: vec![
                    RuleClass::Toggle(ToggleRule::SingleCell),
                    RuleClass::Toggle(ToggleRule::CrossPattern),
                    RuleClass::Toggle(ToggleRule::Neighborhood3x3),
                ],
            },
        ]
    }

    /// Get next optimal probe given current hypothesis distribution
    pub fn next_probe(&self, dist: &HypothesisDistribution) -> Option<Action> {
        // Find the probe with highest expected information gain
        let mut best_probe = None;
        let mut best_gain = 0.0;

        for root in &self.roots {
            let gain = self.expected_info_gain(root, dist);
            if gain > best_gain {
                best_gain = gain;
                best_probe = Some(root.probe.clone());
            }
        }

        // Only probe if gain exceeds threshold
        if best_gain > 0.1 {
            best_probe
        } else {
            None
        }
    }

    /// Calculate expected information gain of a probe
    fn expected_info_gain(&self, node: &DiscriminationNode, dist: &HypothesisDistribution) -> f32 {
        let current_entropy = dist.entropy();

        // Calculate expected entropy after probe
        let mut expected_entropy = 0.0;

        for (outcome, child) in &node.branches {
            let outcome_prob = self.outcome_probability(outcome, dist);
            if outcome_prob > 0.0 {
                let posterior = self.posterior_distribution(outcome, dist);
                expected_entropy += outcome_prob * posterior.entropy();
            }
        }

        current_entropy - expected_entropy
    }

    /// Probability of an outcome given hypothesis distribution
    fn outcome_probability(&self, _outcome: &Outcome, _dist: &HypothesisDistribution) -> f32 {
        // TODO: Learn from data
        0.25 // Uniform for now
    }

    /// Posterior distribution after observing outcome
    fn posterior_distribution(
        &self,
        outcome: &Outcome,
        prior: &HypothesisDistribution,
    ) -> HypothesisDistribution {
        let mut posterior = prior.clone();

        // Update based on outcome
        use crate::dsl::{NavigationRule, ToggleRule};

        match outcome {
            Outcome::PlayerMoved => {
                // Strongly suggests navigation
                posterior.set(
                    RuleClass::Navigation(NavigationRule::SimpleWalls),
                    prior.prob(&RuleClass::Navigation(NavigationRule::SimpleWalls)) * 10.0,
                );
            }
            Outcome::SingleCellChange => {
                posterior.set(
                    RuleClass::Toggle(ToggleRule::SingleCell),
                    prior.prob(&RuleClass::Toggle(ToggleRule::SingleCell)) * 10.0,
                );
            }
            Outcome::CrossChange => {
                posterior.set(
                    RuleClass::Toggle(ToggleRule::CrossPattern),
                    prior.prob(&RuleClass::Toggle(ToggleRule::CrossPattern)) * 10.0,
                );
            }
            _ => {}
        }

        posterior.normalize();
        posterior
    }

    /// Update tree with observed probe result
    pub fn observe(&mut self, action: &Action, outcome: &Outcome) {
        // TODO: Online learning to refine tree
        let _ = (action, outcome);
    }
}

/// Classify an observed change into an Outcome
pub fn classify_change(
    before: &crate::grid::Grid,
    after: &crate::grid::Grid,
    action: &Action,
) -> Outcome {
    let diff = before.diff(after);

    match action {
        Action::Move(_) => {
            if diff > 0 {
                Outcome::PlayerMoved
            } else {
                Outcome::PlayerBlocked
            }
        }
        Action::Click(_, _) => match diff {
            0 => Outcome::NoChange,
            1 => Outcome::SingleCellChange,
            5 => Outcome::CrossChange,
            9 => Outcome::NeighborhoodChange,
            w if w == before.width as u32 => Outcome::RowChange,
            h if h == before.height as u32 => Outcome::ColumnChange,
            _ => Outcome::Other,
        },
        _ => Outcome::Other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discrimination_tree() {
        let tree = DiscriminationTree::new();
        let dist = HypothesisDistribution::uniform(&[
            RuleClass::Unknown,
            RuleClass::Navigation(crate::dsl::NavigationRule::SimpleWalls),
        ]);

        let probe = tree.next_probe(&dist);
        assert!(probe.is_some());
    }
}
