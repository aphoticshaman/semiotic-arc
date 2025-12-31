//! Viability-Preserving Planner
//!
//! Layer 5-6 of the architecture.
//!
//! Key insight: Preserve optionality.
//! Don't just find A path - find a path that keeps escape routes open.
//! Health/resources are "optionality tokens" - spend wisely.

use crate::dsl::RuleClass;
use crate::grid::Grid;
use crate::simulator::{SimResult, WorldSimulator};
use crate::Action;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

/// A* planner with viability tracking
pub struct Planner {
    /// Maximum nodes to expand
    pub max_expansions: usize,

    /// Weight for viability in scoring
    pub viability_weight: f32,

    /// Statistics
    pub nodes_expanded: u64,
    pub plans_generated: u64,
}

impl Planner {
    pub fn new() -> Self {
        Self {
            max_expansions: 10000,
            viability_weight: 0.3,
            nodes_expanded: 0,
            plans_generated: 0,
        }
    }

    pub fn reset(&mut self) {
        self.nodes_expanded = 0;
        self.plans_generated = 0;
    }

    /// Plan from current state to goal
    pub fn plan(
        &mut self,
        start: &Grid,
        rule: &RuleClass,
        simulator: &WorldSimulator,
        budget: u32,
    ) -> Vec<Action> {
        // For now, use simple BFS/A* depending on game type
        match rule {
            RuleClass::Navigation(_) => self.plan_navigation(start, rule, simulator, budget),
            RuleClass::Toggle(_) => self.plan_toggle(start, rule, simulator, budget),
            _ => self.plan_generic(start, rule, simulator, budget),
        }
    }

    /// Plan for navigation games (A* with heuristic)
    fn plan_navigation(
        &mut self,
        start: &Grid,
        rule: &RuleClass,
        simulator: &WorldSimulator,
        budget: u32,
    ) -> Vec<Action> {
        let mut open = BinaryHeap::new();
        let mut closed: HashSet<u64> = HashSet::new();
        let mut came_from: HashMap<u64, (u64, Action)> = HashMap::new();
        let mut g_score: HashMap<u64, f32> = HashMap::new();

        let start_hash = start.hash();
        g_score.insert(start_hash, 0.0);

        open.push(SearchNode {
            state: start.clone(),
            g: 0.0,
            h: 0.0, // TODO: Add goal heuristic
            f: 0.0,
        });

        let actions = [
            Action::Move(crate::Direction::Up),
            Action::Move(crate::Direction::Down),
            Action::Move(crate::Direction::Left),
            Action::Move(crate::Direction::Right),
        ];

        let mut expansions = 0;
        let mut best_state = start.clone();
        let mut best_score = f32::MIN;

        while let Some(current) = open.pop() {
            let current_hash = current.state.hash();

            if closed.contains(&current_hash) {
                continue;
            }
            closed.insert(current_hash);

            expansions += 1;
            if expansions > self.max_expansions {
                break;
            }

            // Track best state seen (for incomplete searches)
            let score = self.evaluate_state(&current.state, rule);
            if score > best_score {
                best_score = score;
                best_state = current.state.clone();
            }

            // Expand neighbors
            for action in &actions {
                let mut sim = WorldSimulator::new();
                sim.set_rule(rule.clone());

                if let SimResult::Success { new_state, .. } = sim.simulate(&current.state, action) {
                    let new_hash = new_state.hash();

                    if closed.contains(&new_hash) {
                        continue;
                    }

                    let new_g = current.g + 1.0;
                    let existing_g = *g_score.get(&new_hash).unwrap_or(&f32::MAX);

                    if new_g < existing_g {
                        g_score.insert(new_hash, new_g);
                        came_from.insert(new_hash, (current_hash, action.clone()));

                        let h = self.navigation_heuristic(&new_state);
                        open.push(SearchNode {
                            state: new_state,
                            g: new_g,
                            h,
                            f: new_g + h,
                        });
                    }
                }
            }
        }

        self.nodes_expanded += expansions as u64;
        self.plans_generated += 1;

        // Reconstruct path to best state
        self.reconstruct_path(&came_from, best_state.hash(), start.hash())
    }

    /// Plan for toggle games (BFS with pattern matching)
    fn plan_toggle(
        &mut self,
        start: &Grid,
        rule: &RuleClass,
        simulator: &WorldSimulator,
        budget: u32,
    ) -> Vec<Action> {
        // For toggle games, try clicking each non-zero cell
        // Score by how much it improves the state

        let mut best_action = None;
        let mut best_score = f32::MIN;

        for y in 0..start.height {
            for x in 0..start.width {
                let action = Action::Click(x, y);
                let mut sim = WorldSimulator::new();
                sim.set_rule(rule.clone());

                if let SimResult::Success { new_state, .. } = sim.simulate(start, &action) {
                    let score = self.evaluate_state(&new_state, rule);
                    if score > best_score {
                        best_score = score;
                        best_action = Some(action);
                    }
                }
            }
        }

        best_action.into_iter().collect()
    }

    /// Generic planning (random search with evaluation)
    fn plan_generic(
        &mut self,
        start: &Grid,
        rule: &RuleClass,
        _simulator: &WorldSimulator,
        _budget: u32,
    ) -> Vec<Action> {
        // Fallback: no-op
        vec![]
    }

    /// Reconstruct path from came_from map
    fn reconstruct_path(
        &self,
        came_from: &HashMap<u64, (u64, Action)>,
        goal_hash: u64,
        start_hash: u64,
    ) -> Vec<Action> {
        let mut path = Vec::new();
        let mut current = goal_hash;

        while current != start_hash {
            if let Some((prev, action)) = came_from.get(&current) {
                path.push(action.clone());
                current = *prev;
            } else {
                break;
            }
        }

        path.reverse();
        path
    }

    /// Evaluate a state (higher = better)
    fn evaluate_state(&self, state: &Grid, rule: &RuleClass) -> f32 {
        // Generic evaluation based on state features
        let hist = state.color_histogram();
        let components = state.connected_components();

        // Fewer active cells = closer to goal (often)
        let active_cells: u16 = hist[1..].iter().sum();

        // More connected = more structured
        let connectivity = components.len() as f32;

        -(active_cells as f32) + connectivity * 0.1
    }

    /// Heuristic for navigation games
    fn navigation_heuristic(&self, state: &Grid) -> f32 {
        // TODO: Add goal position and compute Manhattan distance
        0.0
    }
}

/// Node in A* search
#[derive(Clone)]
struct SearchNode {
    state: Grid,
    g: f32, // Cost so far
    h: f32, // Heuristic to goal
    f: f32, // Total = g + h
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl Eq for SearchNode {}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap: lower f is better
        other.f.partial_cmp(&self.f).unwrap_or(Ordering::Equal)
    }
}

/// Viability tracker for resource-constrained games
#[derive(Clone, Debug)]
pub struct ViabilityState {
    /// Remaining health/resources
    pub resources: HashMap<String, f32>,

    /// Estimated actions to goal
    pub estimated_cost: f32,

    /// Is this state viable (can still win)?
    pub is_viable: bool,
}

impl ViabilityState {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            estimated_cost: 0.0,
            is_viable: true,
        }
    }

    /// Update after an action
    pub fn update(&mut self, cost: f32, reward: f32) {
        self.estimated_cost -= 1.0;

        // Update resource (e.g., health)
        if let Some(health) = self.resources.get_mut("health") {
            *health += reward - cost;
            if *health <= 0.0 {
                self.is_viable = false;
            }
        }
    }

    /// Calculate viability score (higher = more options)
    pub fn viability_score(&self) -> f32 {
        if !self.is_viable {
            return 0.0;
        }

        let resource_score: f32 = self.resources.values().sum::<f32>();
        resource_score / (self.estimated_cost + 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::NavigationRule;

    #[test]
    fn test_planner_creation() {
        let planner = Planner::new();
        assert_eq!(planner.nodes_expanded, 0);
    }

    #[test]
    fn test_navigation_planning() {
        let mut planner = Planner::new();
        let mut grid = Grid::new(5, 5);
        grid.set(0, 0, 1); // Player

        let sim = WorldSimulator::new();
        let plan = planner.plan(
            &grid,
            &RuleClass::Navigation(NavigationRule::SimpleWalls),
            &sim,
            10,
        );

        // Should generate some plan
        assert!(planner.nodes_expanded > 0);
    }
}
