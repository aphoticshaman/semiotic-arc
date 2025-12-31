//! Phase Transition Detection for ARC Games
//!
//! Ported from LatticeForge nucleation-packages
//!
//! Key insight: Games exhibit phase transitions.
//! - CRYSTALLINE: Stable, predictable (we've identified the rule)
//! - SUPERCOOLED: Appears stable but one action could break it
//! - NUCLEATING: Active transition, rule is shifting
//! - PLASMA: Chaotic, unpredictable (novel game type?)
//! - ANNEALING: Settling into new pattern
//!
//! Detecting phase = knowing when to explore vs exploit

use crate::grid::Grid;
use std::collections::VecDeque;

/// Phase states (from Landau theory)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystemPhase {
    /// Stable equilibrium - rule is known, low entropy
    Crystalline,
    /// Metastable - appears stable but susceptible to perturbation
    Supercooled,
    /// Active transition - rule may be changing
    Nucleating,
    /// High energy chaotic state - novel situation
    Plasma,
    /// Post-transition settling - new rule forming
    Annealing,
}

/// Phase state with parameters
#[derive(Clone, Debug)]
pub struct PhaseState {
    pub phase: SystemPhase,
    pub confidence: f32,
    /// System "energy" - higher = more volatile
    pub temperature: f32,
    /// 0 = disordered, 1 = highly ordered
    pub order_parameter: f32,
    /// How close to phase transition
    pub critical_exponent: f32,
    /// Potential cascade triggers
    pub nucleation_sites: usize,
}

/// Phase transition detector
pub struct PhaseDetector {
    /// Proprietary constants from LatticeForge
    critical_temperature: f32,
    nucleation_threshold: f32,

    /// History of states for trajectory analysis
    history: VecDeque<PhaseState>,

    /// History of grid observations
    grid_history: VecDeque<GridSnapshot>,

    /// Maximum history length
    max_history: usize,
}

/// Snapshot of grid for history tracking
#[derive(Clone)]
struct GridSnapshot {
    color_hist: [u16; 10],
    component_count: usize,
    total_active: u16,
    centroid: (f32, f32),
}

impl PhaseDetector {
    pub fn new() -> Self {
        Self {
            // Tuned constants from LatticeForge backtesting
            critical_temperature: 0.7632,
            nucleation_threshold: 0.4219,
            history: VecDeque::with_capacity(100),
            grid_history: VecDeque::with_capacity(50),
            max_history: 50,
        }
    }

    /// Analyze current phase from grid state
    pub fn analyze(&mut self, grid: &Grid) -> PhaseState {
        // Create snapshot
        let snapshot = self.create_snapshot(grid);

        // Calculate parameters
        let temperature = self.calculate_temperature(&snapshot);
        let order_parameter = self.calculate_order_parameter(&snapshot);
        let critical_exponent = self.calculate_critical_exponent(temperature, order_parameter);
        let nucleation_sites = self.detect_nucleation_sites(grid);

        // Classify phase
        let phase = self.classify_phase(
            temperature,
            order_parameter,
            critical_exponent,
            nucleation_sites,
        );

        // Calculate confidence
        let confidence = self.calculate_confidence(temperature, order_parameter, critical_exponent);

        let state = PhaseState {
            phase,
            confidence,
            temperature,
            order_parameter,
            critical_exponent,
            nucleation_sites,
        };

        // Update history
        self.history.push_back(state.clone());
        if self.history.len() > self.max_history {
            self.history.pop_front();
        }

        self.grid_history.push_back(snapshot);
        if self.grid_history.len() > self.max_history {
            self.grid_history.pop_front();
        }

        state
    }

    /// Check if phase transition is imminent
    pub fn transition_imminent(&self) -> bool {
        if let Some(current) = self.history.back() {
            current.critical_exponent < 0.15 && current.nucleation_sites > 2
        } else {
            false
        }
    }

    /// Get trajectory trend
    pub fn trajectory(&self) -> Option<(f32, f32)> {
        if self.history.len() < 5 {
            return None;
        }

        let recent: Vec<_> = self.history.iter().rev().take(5).collect();
        let temp_delta = recent[0].temperature - recent[4].temperature;
        let order_delta = recent[0].order_parameter - recent[4].order_parameter;

        Some((temp_delta, order_delta))
    }

    fn create_snapshot(&self, grid: &Grid) -> GridSnapshot {
        let color_hist = grid.color_histogram();
        let components = grid.connected_components();
        let total_active: u16 = color_hist[1..].iter().sum();

        // Calculate centroid of active cells
        let mut cx = 0.0f32;
        let mut cy = 0.0f32;
        let mut count = 0;

        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.get(x, y) != 0 {
                    cx += x as f32;
                    cy += y as f32;
                    count += 1;
                }
            }
        }

        let centroid = if count > 0 {
            (cx / count as f32, cy / count as f32)
        } else {
            (grid.width as f32 / 2.0, grid.height as f32 / 2.0)
        };

        GridSnapshot {
            color_hist,
            component_count: components.len(),
            total_active,
            centroid,
        }
    }

    fn calculate_temperature(&self, snapshot: &GridSnapshot) -> f32 {
        if self.grid_history.is_empty() {
            return 0.5;
        }

        // Temperature = variance in recent snapshots
        let recent: Vec<_> = self.grid_history.iter().rev().take(10).collect();

        // Variance in active cell count
        let mean_active: f32 = recent.iter().map(|s| s.total_active as f32).sum::<f32>()
            / recent.len() as f32;
        let var_active: f32 = recent
            .iter()
            .map(|s| (s.total_active as f32 - mean_active).powi(2))
            .sum::<f32>()
            / recent.len() as f32;

        // Variance in component count
        let mean_comp: f32 = recent.iter().map(|s| s.component_count as f32).sum::<f32>()
            / recent.len() as f32;
        let var_comp: f32 = recent
            .iter()
            .map(|s| (s.component_count as f32 - mean_comp).powi(2))
            .sum::<f32>()
            / recent.len() as f32;

        // Variance in centroid position
        let mean_cx: f32 = recent.iter().map(|s| s.centroid.0).sum::<f32>() / recent.len() as f32;
        let mean_cy: f32 = recent.iter().map(|s| s.centroid.1).sum::<f32>() / recent.len() as f32;
        let var_centroid: f32 = recent
            .iter()
            .map(|s| (s.centroid.0 - mean_cx).powi(2) + (s.centroid.1 - mean_cy).powi(2))
            .sum::<f32>()
            / recent.len() as f32;

        // Combine into temperature (normalized)
        let raw_temp = (var_active / 100.0 + var_comp / 10.0 + var_centroid / 100.0) / 3.0;
        raw_temp.min(1.0).max(0.0)
    }

    fn calculate_order_parameter(&self, snapshot: &GridSnapshot) -> f32 {
        // Order = how structured is the current state?

        // Entropy of color distribution
        let total = snapshot.total_active as f32 + snapshot.color_hist[0] as f32;
        if total == 0.0 {
            return 0.5;
        }

        let mut entropy = 0.0f32;
        for &count in snapshot.color_hist.iter() {
            if count > 0 {
                let p = count as f32 / total;
                entropy -= p * p.log2();
            }
        }

        // Normalize entropy (max entropy for 10 colors is log2(10) ≈ 3.32)
        let normalized_entropy = entropy / 3.32;

        // Order is inverse of entropy
        let order = 1.0 - normalized_entropy;

        // Factor in component structure
        let comp_factor = if snapshot.component_count == 0 {
            0.0
        } else if snapshot.component_count <= 3 {
            1.0 // Few components = organized
        } else {
            1.0 / (snapshot.component_count as f32 / 3.0)
        };

        (order * 0.7 + comp_factor * 0.3).min(1.0).max(0.0)
    }

    fn calculate_critical_exponent(&self, temperature: f32, order_parameter: f32) -> f32 {
        // Distance from critical point in phase space
        let temp_distance = (temperature - self.critical_temperature).abs();
        let order_distance = (order_parameter - 0.5).abs();

        // Critical exponent = distance from critical point
        let exponent = (temp_distance.powi(2) + order_distance.powi(2)).sqrt();

        // Normalize to [0, 1]
        (exponent / 1.414).min(1.0)
    }

    fn detect_nucleation_sites(&self, grid: &Grid) -> usize {
        // Nucleation sites = regions of high local correlation
        // that could trigger cascade effects

        if self.grid_history.len() < 5 {
            return 0;
        }

        let mut sites = 0;
        let components = grid.connected_components();

        // Each component is a potential nucleation site
        for comp in components.iter() {
            // Check if this component has been changing
            let size = comp.size();
            if size >= 3 && size <= 20 {
                sites += 1;
            }
        }

        sites
    }

    fn classify_phase(
        &self,
        temperature: f32,
        order_parameter: f32,
        critical_exponent: f32,
        nucleation_sites: usize,
    ) -> SystemPhase {
        // Near critical point with nucleation = active transition
        if critical_exponent < 0.1 && nucleation_sites > 2 {
            return SystemPhase::Nucleating;
        }

        // High temperature, low order = plasma (chaotic)
        if temperature > 0.8 && order_parameter < 0.3 {
            return SystemPhase::Plasma;
        }

        // Low temperature, high order = crystalline (stable)
        if temperature < 0.3 && order_parameter > 0.7 {
            return SystemPhase::Crystalline;
        }

        // Moderate temperature, high order = supercooled
        if temperature < 0.5 && order_parameter > 0.5 && nucleation_sites > 0 {
            return SystemPhase::Supercooled;
        }

        // Check for annealing (post-transition settling)
        if let Some((temp_delta, order_delta)) = self.trajectory() {
            if temp_delta < -0.1 && order_delta > 0.1 {
                return SystemPhase::Annealing;
            }
        }

        // Default
        SystemPhase::Supercooled
    }

    fn calculate_confidence(
        &self,
        temperature: f32,
        order_parameter: f32,
        critical_exponent: f32,
    ) -> f32 {
        // Confidence is higher when far from critical point
        let base = critical_exponent;

        // Extreme values increase confidence
        let extremity = (temperature - 0.5).abs() / 0.5 + (order_parameter - 0.5).abs() / 0.5;

        (base * 0.6 + extremity * 0.2 + 0.2).min(1.0)
    }

    /// Reset detector state
    pub fn reset(&mut self) {
        self.history.clear();
        self.grid_history.clear();
    }
}

impl Default for PhaseDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Integrated Information (Φ) approximation for anomaly detection
pub struct PhiCalculator {
    /// History of observations per "dimension"
    dimension_history: Vec<VecDeque<f32>>,
    /// Dimension names
    dimension_names: Vec<String>,
}

impl PhiCalculator {
    pub fn new(dimensions: Vec<String>) -> Self {
        let n = dimensions.len();
        Self {
            dimension_history: vec![VecDeque::with_capacity(100); n],
            dimension_names: dimensions,
        }
    }

    /// Update with new observation
    pub fn observe(&mut self, values: &[f32]) {
        for (i, &v) in values.iter().enumerate() {
            if i < self.dimension_history.len() {
                let hist = &mut self.dimension_history[i];
                hist.push_back(v);
                if hist.len() > 100 {
                    hist.pop_front();
                }
            }
        }
    }

    /// Calculate Φ (integrated information)
    /// Higher Φ = more "unified" system state (potential anomaly)
    pub fn calculate_phi(&self) -> f32 {
        if self.dimension_history.len() < 2 {
            return 0.0;
        }

        // Calculate mutual information between all pairs
        let mut total_mi = 0.0f32;
        let mut pairs = 0;

        for i in 0..self.dimension_history.len() {
            for j in (i + 1)..self.dimension_history.len() {
                let mi = self.mutual_information(i, j);
                total_mi += mi;
                pairs += 1;
            }
        }

        if pairs > 0 {
            total_mi / pairs as f32
        } else {
            0.0
        }
    }

    fn mutual_information(&self, i: usize, j: usize) -> f32 {
        let hist_i = &self.dimension_history[i];
        let hist_j = &self.dimension_history[j];

        let n = hist_i.len().min(hist_j.len());
        if n < 10 {
            return 0.0;
        }

        // Approximate MI using correlation
        // MI ≈ -0.5 * log(1 - r²) for Gaussian variables
        let corr = self.correlation(hist_i, hist_j, n);
        let r2 = corr.powi(2).min(0.99);

        if r2 > 0.0 {
            -0.5 * (1.0 - r2).ln()
        } else {
            0.0
        }
    }

    fn correlation(&self, x: &VecDeque<f32>, y: &VecDeque<f32>, n: usize) -> f32 {
        let mean_x: f32 = x.iter().take(n).sum::<f32>() / n as f32;
        let mean_y: f32 = y.iter().take(n).sum::<f32>() / n as f32;

        let mut num = 0.0f32;
        let mut den_x = 0.0f32;
        let mut den_y = 0.0f32;

        for i in 0..n {
            let dx = x[i] - mean_x;
            let dy = y[i] - mean_y;
            num += dx * dy;
            den_x += dx * dx;
            den_y += dy * dy;
        }

        let den = (den_x * den_y).sqrt();
        if den > 0.0 {
            num / den
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_detector() {
        let mut detector = PhaseDetector::new();
        let grid = Grid::new(10, 10);

        let state = detector.analyze(&grid);
        assert!(state.confidence >= 0.0 && state.confidence <= 1.0);
    }

    #[test]
    fn test_phi_calculator() {
        let mut phi = PhiCalculator::new(vec!["x".to_string(), "y".to_string()]);

        // Correlated observations should have high phi
        for i in 0..50 {
            let v = i as f32;
            phi.observe(&[v, v * 0.9 + 0.1]);
        }

        let phi_value = phi.calculate_phi();
        assert!(phi_value > 0.0);
    }
}
