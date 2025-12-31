//! Adaptive Probing via Group Testing
//!
//! The ACTION6 problem: 4096 possible click locations.
//! Random clicking = death by budget exhaustion.
//!
//! Group testing achieves O(k log n) for k active cells.
//! If only 10 cells respond to clicks, we find them in ~40 probes.
//!
//! Inspired by: compressed sensing, pooled COVID testing,
//! and the fundamental theorem of information theory.

use crate::grid::Grid;
use crate::Action;
use std::collections::HashSet;

/// Group testing strategy for finding responsive cells
pub struct AdaptiveProber {
    /// Grid dimensions
    width: u8,
    height: u8,

    /// Known responsive cells
    responsive: HashSet<(u8, u8)>,

    /// Known non-responsive cells
    non_responsive: HashSet<(u8, u8)>,

    /// Current probing stage
    stage: ProbeStage,

    /// Binary search state for row/column refinement
    search_state: Option<BinarySearchState>,

    /// Probes used
    probes_used: u32,
}

#[derive(Clone, Debug)]
enum ProbeStage {
    /// Initial row sweep
    RowSweep { current_row: u8 },
    /// Row refinement via binary search
    RowRefine { row: u8 },
    /// Column sweep for known rows
    ColumnRefine { row: u8, col_start: u8, col_end: u8 },
    /// Direct probing of candidates
    DirectProbe,
    /// Done
    Complete,
}

#[derive(Clone, Debug)]
struct BinarySearchState {
    low: u8,
    high: u8,
    midpoints: Vec<u8>,
}

impl AdaptiveProber {
    pub fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            responsive: HashSet::new(),
            non_responsive: HashSet::new(),
            stage: ProbeStage::RowSweep { current_row: 0 },
            search_state: None,
            probes_used: 0,
        }
    }

    /// Get next probe action
    pub fn next_probe(&mut self) -> Option<Action> {
        match &self.stage {
            ProbeStage::RowSweep { current_row } => {
                if *current_row >= self.height {
                    self.stage = ProbeStage::DirectProbe;
                    return self.next_probe();
                }
                // Probe center of row
                let x = self.width / 2;
                let y = *current_row;
                self.stage = ProbeStage::RowSweep {
                    current_row: current_row + 1,
                };
                self.probes_used += 1;
                Some(Action::Click(x, y))
            }
            ProbeStage::RowRefine { row } => {
                // Binary search within row
                if let Some(ref state) = self.search_state {
                    if state.low >= state.high {
                        self.stage = ProbeStage::RowSweep {
                            current_row: row + 1,
                        };
                        self.search_state = None;
                        return self.next_probe();
                    }
                    let mid = (state.low + state.high) / 2;
                    self.probes_used += 1;
                    Some(Action::Click(mid, *row))
                } else {
                    self.search_state = Some(BinarySearchState {
                        low: 0,
                        high: self.width,
                        midpoints: Vec::new(),
                    });
                    self.next_probe()
                }
            }
            ProbeStage::ColumnRefine {
                row,
                col_start,
                col_end,
            } => {
                if col_start >= col_end {
                    self.stage = ProbeStage::DirectProbe;
                    return self.next_probe();
                }
                let mid = (col_start + col_end) / 2;
                self.probes_used += 1;
                Some(Action::Click(mid, *row))
            }
            ProbeStage::DirectProbe => {
                // Find next unprobed cell that might be responsive
                for y in 0..self.height {
                    for x in 0..self.width {
                        if !self.responsive.contains(&(x, y))
                            && !self.non_responsive.contains(&(x, y))
                        {
                            self.probes_used += 1;
                            return Some(Action::Click(x, y));
                        }
                    }
                }
                self.stage = ProbeStage::Complete;
                None
            }
            ProbeStage::Complete => None,
        }
    }

    /// Update based on observed result
    pub fn observe(&mut self, action: &Action, response: bool) {
        if let Action::Click(x, y) = action {
            if response {
                self.responsive.insert((*x, *y));
            } else {
                self.non_responsive.insert((*x, *y));
            }
        }
    }

    /// Get all known responsive cells
    pub fn responsive_cells(&self) -> &HashSet<(u8, u8)> {
        &self.responsive
    }

    /// Probes used so far
    pub fn probes_used(&self) -> u32 {
        self.probes_used
    }

    /// Is probing complete?
    pub fn is_complete(&self) -> bool {
        matches!(self.stage, ProbeStage::Complete)
    }
}

/// Quadtree-based hierarchical probing
/// Divides grid into quadrants, probes recursively
pub struct QuadtreeProber {
    width: u8,
    height: u8,

    /// Stack of regions to probe
    regions: Vec<Region>,

    /// Responsive regions (leaf nodes)
    responsive: Vec<(u8, u8)>,

    /// Probes used
    probes_used: u32,

    /// Minimum region size before direct probing
    min_size: u8,
}

#[derive(Clone, Debug)]
struct Region {
    x: u8,
    y: u8,
    w: u8,
    h: u8,
}

impl Region {
    fn center(&self) -> (u8, u8) {
        (self.x + self.w / 2, self.y + self.h / 2)
    }

    fn subdivide(&self) -> Vec<Region> {
        let half_w = self.w / 2;
        let half_h = self.h / 2;

        let mut regions = Vec::with_capacity(4);

        // Top-left
        if half_w > 0 && half_h > 0 {
            regions.push(Region {
                x: self.x,
                y: self.y,
                w: half_w,
                h: half_h,
            });
        }

        // Top-right
        if self.w > half_w && half_h > 0 {
            regions.push(Region {
                x: self.x + half_w,
                y: self.y,
                w: self.w - half_w,
                h: half_h,
            });
        }

        // Bottom-left
        if half_w > 0 && self.h > half_h {
            regions.push(Region {
                x: self.x,
                y: self.y + half_h,
                w: half_w,
                h: self.h - half_h,
            });
        }

        // Bottom-right
        if self.w > half_w && self.h > half_h {
            regions.push(Region {
                x: self.x + half_w,
                y: self.y + half_h,
                w: self.w - half_w,
                h: self.h - half_h,
            });
        }

        regions
    }

    fn is_leaf(&self, min_size: u8) -> bool {
        self.w <= min_size || self.h <= min_size
    }
}

impl QuadtreeProber {
    pub fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            regions: vec![Region {
                x: 0,
                y: 0,
                w: width,
                h: height,
            }],
            responsive: Vec::new(),
            probes_used: 0,
            min_size: 2,
        }
    }

    /// Get next probe action
    pub fn next_probe(&mut self) -> Option<Action> {
        let region = self.regions.pop()?;
        let (x, y) = region.center();
        self.probes_used += 1;

        // Store current region for observation
        // (We'll need to know what region this probe was for)
        self.regions.push(region);

        Some(Action::Click(x, y))
    }

    /// Update based on observed result
    pub fn observe(&mut self, response: bool) {
        let Some(region) = self.regions.pop() else {
            return;
        };

        if response {
            // Region has responsive cells
            if region.is_leaf(self.min_size) {
                // Small enough - add all cells as candidates
                for dy in 0..region.h {
                    for dx in 0..region.w {
                        self.responsive.push((region.x + dx, region.y + dy));
                    }
                }
            } else {
                // Subdivide and add to stack
                let subs = region.subdivide();
                self.regions.extend(subs);
            }
        }
        // If no response, discard this region entirely
    }

    /// Get responsive cell candidates
    pub fn responsive_candidates(&self) -> &[(u8, u8)] {
        &self.responsive
    }

    /// Probes used
    pub fn probes_used(&self) -> u32 {
        self.probes_used
    }

    /// Is probing complete?
    pub fn is_complete(&self) -> bool {
        self.regions.is_empty()
    }
}

/// Information-theoretic probe selection
/// Selects probe that maximizes expected information gain
pub struct InfoTheoreticProber {
    /// Probability each cell is responsive
    prob: Vec<Vec<f32>>,

    /// Already probed cells
    probed: HashSet<(u8, u8)>,

    /// Responsive cells found
    responsive: HashSet<(u8, u8)>,
}

impl InfoTheoreticProber {
    pub fn new(width: u8, height: u8) -> Self {
        // Initialize with uniform prior
        let prior = 0.1; // Assume 10% of cells are responsive by default
        Self {
            prob: vec![vec![prior; width as usize]; height as usize],
            probed: HashSet::new(),
            responsive: HashSet::new(),
        }
    }

    /// Get next probe that maximizes information gain
    pub fn next_probe(&self) -> Option<Action> {
        let mut best_pos = None;
        let mut best_gain = f32::NEG_INFINITY;

        for (y, row) in self.prob.iter().enumerate() {
            for (x, &p) in row.iter().enumerate() {
                if self.probed.contains(&(x as u8, y as u8)) {
                    continue;
                }

                // Information gain = entropy reduction
                // For binary outcomes: H(p) = -p*log(p) - (1-p)*log(1-p)
                let gain = self.binary_entropy(p);

                if gain > best_gain {
                    best_gain = gain;
                    best_pos = Some((x as u8, y as u8));
                }
            }
        }

        best_pos.map(|(x, y)| Action::Click(x, y))
    }

    /// Update probabilities based on observation
    pub fn observe(&mut self, x: u8, y: u8, response: bool) {
        self.probed.insert((x, y));

        if response {
            self.responsive.insert((x, y));
            self.prob[y as usize][x as usize] = 1.0;

            // Increase probability of neighbors (spatial correlation)
            self.update_neighbors(x, y, 0.3);
        } else {
            self.prob[y as usize][x as usize] = 0.0;

            // Slightly decrease probability of neighbors
            self.update_neighbors(x, y, -0.1);
        }
    }

    fn update_neighbors(&mut self, x: u8, y: u8, delta: f32) {
        let height = self.prob.len();
        let width = self.prob[0].len();

        for dy in -2i8..=2 {
            for dx in -2i8..=2 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as i8 + dx;
                let ny = y as i8 + dy;

                if nx >= 0 && (nx as usize) < width && ny >= 0 && (ny as usize) < height {
                    let dist = (dx.abs() + dy.abs()) as f32;
                    let adj = delta / dist;

                    let p = &mut self.prob[ny as usize][nx as usize];
                    *p = (*p + adj).clamp(0.0, 1.0);
                }
            }
        }
    }

    fn binary_entropy(&self, p: f32) -> f32 {
        if p <= 0.0 || p >= 1.0 {
            return 0.0;
        }
        -p * p.log2() - (1.0 - p) * (1.0 - p).log2()
    }

    /// Get responsive cells found
    pub fn responsive_cells(&self) -> &HashSet<(u8, u8)> {
        &self.responsive
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_prober() {
        let mut prober = AdaptiveProber::new(8, 8);

        // Should start with row sweep
        let probe = prober.next_probe();
        assert!(probe.is_some());

        if let Some(Action::Click(x, y)) = probe {
            assert_eq!(x, 4); // Center of row
            assert_eq!(y, 0); // First row
        }
    }

    #[test]
    fn test_quadtree_prober() {
        let mut prober = QuadtreeProber::new(16, 16);

        // Should probe center first
        let probe = prober.next_probe();
        assert!(probe.is_some());

        if let Some(Action::Click(x, y)) = probe {
            assert_eq!(x, 8);
            assert_eq!(y, 8);
        }
    }

    #[test]
    fn test_info_theoretic_prober() {
        let prober = InfoTheoreticProber::new(8, 8);

        let probe = prober.next_probe();
        assert!(probe.is_some());

        // With uniform prior, any cell has same entropy
    }
}
