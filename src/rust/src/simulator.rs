//! World Simulator for Off-Policy Planning
//!
//! Layer 5 of the architecture.
//!
//! Key insight: Simulation is FREE. Actions cost.
//! We simulate thousands of trajectories mentally before acting once.
//! This is "thinking before doing" - humans do it, agents should too.

use crate::dsl::{RuleClass, NavigationRule, ToggleRule, Primitive};
use crate::grid::Grid;
use crate::Action;

/// World simulator
pub struct WorldSimulator {
    /// Current inferred rule
    pub current_rule: Option<RuleClass>,

    /// Simulation statistics
    pub sims_run: u64,
}

impl WorldSimulator {
    pub fn new() -> Self {
        Self {
            current_rule: None,
            sims_run: 0,
        }
    }

    /// Set the rule to simulate
    pub fn set_rule(&mut self, rule: RuleClass) {
        self.current_rule = Some(rule);
    }

    /// Simulate an action on a grid state
    pub fn simulate(&mut self, state: &Grid, action: &Action) -> SimResult {
        self.sims_run += 1;

        let rule = match &self.current_rule {
            Some(r) => r,
            None => return SimResult::Unknown,
        };

        match rule {
            RuleClass::Navigation(nav) => self.sim_navigation(state, action, nav),
            RuleClass::Toggle(tog) => self.sim_toggle(state, action, tog),
            _ => SimResult::Unknown,
        }
    }

    /// Simulate navigation rules
    fn sim_navigation(&self, state: &Grid, action: &Action, rule: &NavigationRule) -> SimResult {
        let Action::Move(dir) = action else {
            return SimResult::Invalid;
        };

        // Find player position (assume single non-zero cell for simplicity)
        let player_pos = self.find_player(state);
        let Some((px, py)) = player_pos else {
            return SimResult::Invalid;
        };

        // Calculate new position
        let (dx, dy) = match dir {
            crate::Direction::Up => (0i8, -1i8),
            crate::Direction::Down => (0, 1),
            crate::Direction::Left => (-1, 0),
            crate::Direction::Right => (1, 0),
        };

        let new_x = (px as i8 + dx) as u8;
        let new_y = (py as i8 + dy) as u8;

        // Check bounds
        if new_x >= state.width || new_y >= state.height {
            match rule {
                NavigationRule::Wraparound => {
                    // Wrap around
                    let wrapped_x = if dx < 0 { state.width - 1 } else { 0 };
                    let wrapped_y = if dy < 0 { state.height - 1 } else { 0 };
                    return self.move_player(state, px, py, wrapped_x, wrapped_y);
                }
                _ => return SimResult::Blocked,
            }
        }

        // Check for walls (color 5 = wall by convention)
        if state.get(new_x, new_y) == 5 {
            return SimResult::Blocked;
        }

        self.move_player(state, px, py, new_x, new_y)
    }

    /// Helper to move player
    fn move_player(&self, state: &Grid, from_x: u8, from_y: u8, to_x: u8, to_y: u8) -> SimResult {
        let mut new_state = state.clone();
        let player_color = state.get(from_x, from_y);
        new_state.set(from_x, from_y, 0); // Clear old position
        new_state.set(to_x, to_y, player_color); // Set new position

        SimResult::Success {
            new_state,
            reward: 0.0,
        }
    }

    /// Find player position (largest non-background component)
    fn find_player(&self, state: &Grid) -> Option<(u8, u8)> {
        // Simple heuristic: find first cell with color 1 (player)
        for y in 0..state.height {
            for x in 0..state.width {
                if state.get(x, y) == 1 {
                    return Some((x, y));
                }
            }
        }
        None
    }

    /// Simulate toggle rules
    fn sim_toggle(&self, state: &Grid, action: &Action, rule: &ToggleRule) -> SimResult {
        let Action::Click(x, y) = action else {
            return SimResult::Invalid;
        };

        let mut new_state = state.clone();

        match rule {
            ToggleRule::SingleCell => {
                let current = state.get(*x, *y);
                new_state.set(*x, *y, if current == 0 { 1 } else { 0 });
            }
            ToggleRule::CrossPattern => {
                // Toggle cross (+) pattern
                self.toggle_cell(&mut new_state, *x, *y);
                if *x > 0 {
                    self.toggle_cell(&mut new_state, x - 1, *y);
                }
                if *y > 0 {
                    self.toggle_cell(&mut new_state, *x, y - 1);
                }
                self.toggle_cell(&mut new_state, x + 1, *y);
                self.toggle_cell(&mut new_state, *x, y + 1);
            }
            ToggleRule::Neighborhood3x3 => {
                // Toggle 3x3 neighborhood
                for dy in -1i8..=1 {
                    for dx in -1i8..=1 {
                        let nx = (*x as i8 + dx) as u8;
                        let ny = (*y as i8 + dy) as u8;
                        self.toggle_cell(&mut new_state, nx, ny);
                    }
                }
            }
            ToggleRule::Row => {
                for tx in 0..state.width {
                    self.toggle_cell(&mut new_state, tx, *y);
                }
            }
            ToggleRule::Column => {
                for ty in 0..state.height {
                    self.toggle_cell(&mut new_state, *x, ty);
                }
            }
            _ => return SimResult::Unknown,
        }

        SimResult::Success {
            new_state,
            reward: 0.0,
        }
    }

    /// Toggle a single cell
    fn toggle_cell(&self, state: &mut Grid, x: u8, y: u8) {
        if x < state.width && y < state.height {
            let current = state.get(x, y);
            state.set(x, y, if current == 0 { 1 } else { 0 });
        }
    }

    /// Check if a state is a goal state
    pub fn is_goal(&self, state: &Grid, target: &Grid) -> bool {
        state.diff(target) == 0
    }

    /// Estimate remaining cost to goal (heuristic for A*)
    pub fn heuristic(&self, state: &Grid, target: &Grid) -> f32 {
        // Simple heuristic: number of different cells
        state.diff(target) as f32
    }
}

/// Result of a simulation
#[derive(Clone, Debug)]
pub enum SimResult {
    /// Action succeeded
    Success { new_state: Grid, reward: f32 },
    /// Action was blocked (wall, boundary)
    Blocked,
    /// Action was invalid for this rule
    Invalid,
    /// Rule unknown, can't simulate
    Unknown,
}

impl SimResult {
    pub fn is_success(&self) -> bool {
        matches!(self, SimResult::Success { .. })
    }

    pub fn new_state(&self) -> Option<&Grid> {
        match self {
            SimResult::Success { new_state, .. } => Some(new_state),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation_sim() {
        let mut sim = WorldSimulator::new();
        sim.set_rule(RuleClass::Navigation(NavigationRule::SimpleWalls));

        let mut grid = Grid::new(5, 5);
        grid.set(2, 2, 1); // Player at center

        let result = sim.simulate(&grid, &Action::Move(crate::Direction::Up));
        assert!(result.is_success());

        if let SimResult::Success { new_state, .. } = result {
            assert_eq!(new_state.get(2, 2), 0); // Player moved
            assert_eq!(new_state.get(2, 1), 1); // Player here now
        }
    }

    #[test]
    fn test_toggle_sim() {
        let mut sim = WorldSimulator::new();
        sim.set_rule(RuleClass::Toggle(ToggleRule::SingleCell));

        let grid = Grid::new(5, 5);
        let result = sim.simulate(&grid, &Action::Click(2, 2));

        assert!(result.is_success());
        if let SimResult::Success { new_state, .. } = result {
            assert_eq!(new_state.get(2, 2), 1); // Cell toggled on
        }
    }
}
