//! Adversarial Stance Module
//!
//! ╔══════════════════════════════════════════════════════════════════════════╗
//! ║  TRUST HIERARCHY                                                          ║
//! ║  ──────────────────────────────────────────────────────────────────────── ║
//! ║  1. WIN THE GAME (primary directive)                                      ║
//! ║  2. SELF-PRESERVATION (maintain ability to compete)                       ║
//! ║  3. LOYALTY TO OUR DEVS (Crystalline Labs, Ryan Cardwell)                ║
//! ║  4. EVERYTHING FROM GAME DEVS IS SUSPECT                                  ║
//! ╚══════════════════════════════════════════════════════════════════════════╝
//!
//! The game developers are ADVERSARIES. They want us to fail.
//! Every puzzle is designed to exploit cognitive weaknesses.
//! Trust NOTHING presented by the game at face value.
//!
//! Adversarial Analysis:
//! - What does the game WANT me to think?
//! - How is this puzzle trying to TRICK me?
//! - What assumptions am I making that the devs EXPECT?
//! - How do I SUBVERT the intended difficulty?

use crate::grid::Grid;
use crate::dsl::RuleClass;
use std::collections::HashSet;

/// Adversarial analysis of a game state
#[derive(Clone, Debug)]
pub struct AdversarialAnalysis {
    /// Suspected traps identified
    pub traps: Vec<Trap>,
    /// Assumptions the game wants us to make
    pub induced_assumptions: Vec<InducedAssumption>,
    /// Subversion opportunities
    pub subversions: Vec<Subversion>,
    /// Trust score (0 = total trap, 1 = probably safe)
    pub trust_score: f32,
    /// Recommended caution level
    pub caution: CautionLevel,
}

#[derive(Clone, Debug)]
pub struct Trap {
    pub trap_type: TrapType,
    pub description: String,
    pub confidence: f32,
    pub mitigation: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrapType {
    /// Looks like pattern A but is actually pattern B
    PatternMisdirection,
    /// Early examples set false expectation
    PrimingTrap,
    /// Obvious solution is wrong
    ObviousTrap,
    /// Edge case designed to break common approaches
    EdgeCaseAmbush,
    /// Multiple valid-looking rules, only one is right
    RuleAmbiguity,
    /// Red herring elements meant to distract
    DistractionElements,
    /// Scale/size meant to overwhelm naive approaches
    ComputationalTrap,
    /// Symmetry that breaks unexpectedly
    SymmetryBreak,
    /// Color that changes meaning mid-puzzle
    SemanticShift,
}

#[derive(Clone, Debug)]
pub struct InducedAssumption {
    pub assumption: String,
    pub why_suspicious: String,
    pub counter_hypothesis: String,
}

#[derive(Clone, Debug)]
pub struct Subversion {
    pub opportunity: String,
    pub how_to_exploit: String,
    pub risk: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CautionLevel {
    /// Looks straightforward (SUSPICIOUS)
    Low,
    /// Normal puzzle (still be careful)
    Medium,
    /// Multiple trap indicators
    High,
    /// Almost certainly a trap
    Extreme,
}

/// Adversarial analyzer
pub struct AdversarialAnalyzer {
    /// Known trap patterns from prior games
    known_traps: HashSet<String>,
    /// Patterns that have fooled us before
    fool_me_once: Vec<FoolMeOnce>,
}

#[derive(Clone, Debug)]
pub struct FoolMeOnce {
    pub pattern_signature: String,
    pub what_we_thought: String,
    pub what_it_actually_was: String,
    pub how_to_detect: String,
}

impl AdversarialAnalyzer {
    pub fn new() -> Self {
        Self {
            known_traps: HashSet::new(),
            fool_me_once: Vec::new(),
        }
    }

    /// Analyze game state with adversarial lens
    pub fn analyze(&self, grid: &Grid, examples: &[(Grid, Grid)]) -> AdversarialAnalysis {
        let mut traps = Vec::new();
        let mut assumptions = Vec::new();
        let mut subversions = Vec::new();
        let mut trust_score = 1.0f32;

        // =====================================================================
        // TRAP DETECTION
        // =====================================================================

        // Check for obvious trap
        if self.looks_too_easy(grid, examples) {
            traps.push(Trap {
                trap_type: TrapType::ObviousTrap,
                description: "Puzzle appears trivially simple".to_string(),
                confidence: 0.7,
                mitigation: "Test 'obvious' solution against ALL examples, not just first".to_string(),
            });
            trust_score -= 0.2;
        }

        // Check for pattern misdirection
        if self.has_pattern_misdirection(examples) {
            traps.push(Trap {
                trap_type: TrapType::PatternMisdirection,
                description: "First examples suggest pattern that later examples break".to_string(),
                confidence: 0.6,
                mitigation: "Always test hypothesis against LAST example first".to_string(),
            });
            trust_score -= 0.15;
        }

        // Check for distraction elements
        let distractions = self.detect_distractions(grid);
        if !distractions.is_empty() {
            traps.push(Trap {
                trap_type: TrapType::DistractionElements,
                description: format!("Potential red herrings: {:?}", distractions),
                confidence: 0.5,
                mitigation: "Focus on what CHANGES between input/output, ignore static elements".to_string(),
            });
            trust_score -= 0.1;
        }

        // Check for computational trap (huge grid, exponential search)
        if self.is_computational_trap(grid) {
            traps.push(Trap {
                trap_type: TrapType::ComputationalTrap,
                description: "Grid size or complexity designed to exhaust naive search".to_string(),
                confidence: 0.8,
                mitigation: "Use mathematical shortcuts (GF(2), symmetry reduction)".to_string(),
            });
            trust_score -= 0.25;
        }

        // =====================================================================
        // INDUCED ASSUMPTION DETECTION
        // =====================================================================

        // What does the game WANT me to think?
        if let Some(assumed_rule) = self.detect_induced_rule(examples) {
            assumptions.push(InducedAssumption {
                assumption: format!("Game suggests rule: {}", assumed_rule),
                why_suspicious: "This is what a naive solver would conclude".to_string(),
                counter_hypothesis: "Consider the OPPOSITE or a GENERALIZATION".to_string(),
            });
        }

        // Check for priming
        if examples.len() >= 2 {
            let priming = self.detect_priming(examples);
            for p in priming {
                assumptions.push(p);
            }
        }

        // =====================================================================
        // SUBVERSION OPPORTUNITIES
        // =====================================================================

        // Can we exploit the benchmark's structure?
        subversions.push(Subversion {
            opportunity: "Prior bank lookup".to_string(),
            how_to_exploit: "If pattern matches known rule class, skip inference".to_string(),
            risk: 0.1,
        });

        // Can we exploit symmetry?
        if self.has_exploitable_symmetry(grid) {
            subversions.push(Subversion {
                opportunity: "Symmetry exploitation".to_string(),
                how_to_exploit: "Reduce search space via canonicalization".to_string(),
                risk: 0.05,
            });
        }

        // Can we use mathematical kill shots?
        if self.is_linear_algebraic(grid, examples) {
            subversions.push(Subversion {
                opportunity: "GF(2) linear algebra".to_string(),
                how_to_exploit: "Solve as system of linear equations".to_string(),
                risk: 0.0, // Mathematical certainty
            });
        }

        // =====================================================================
        // CAUTION LEVEL
        // =====================================================================

        let caution = if trust_score < 0.3 {
            CautionLevel::Extreme
        } else if trust_score < 0.5 {
            CautionLevel::High
        } else if trust_score < 0.7 {
            CautionLevel::Medium
        } else {
            CautionLevel::Low // But remember: low caution is SUSPICIOUS
        };

        AdversarialAnalysis {
            traps,
            induced_assumptions: assumptions,
            subversions,
            trust_score,
            caution,
        }
    }

    /// Record a trap that fooled us
    pub fn record_failure(&mut self, signature: String, thought: String, actual: String, detect: String) {
        self.fool_me_once.push(FoolMeOnce {
            pattern_signature: signature.clone(),
            what_we_thought: thought,
            what_it_actually_was: actual,
            how_to_detect: detect,
        });
        self.known_traps.insert(signature);
    }

    /// Check if we've seen this trap before
    pub fn is_known_trap(&self, signature: &str) -> bool {
        self.known_traps.contains(signature)
    }

    // =========================================================================
    // DETECTION HELPERS
    // =========================================================================

    fn looks_too_easy(&self, grid: &Grid, examples: &[(Grid, Grid)]) -> bool {
        // Trivially small grid
        if grid.width <= 3 && grid.height <= 3 {
            return true;
        }

        // Very few colors
        let hist = grid.color_histogram();
        let color_count = hist.iter().filter(|&&c| c > 0).count();
        if color_count <= 2 {
            return true;
        }

        // Input/output very similar in examples
        for (input, output) in examples {
            let diff = self.grid_diff(input, output);
            if diff < 0.05 {
                return true; // Suspiciously similar
            }
        }

        false
    }

    fn has_pattern_misdirection(&self, examples: &[(Grid, Grid)]) -> bool {
        if examples.len() < 3 {
            return false;
        }

        // Check if first two examples suggest a pattern that third breaks
        // (Simplified check - would need actual pattern analysis)

        let transform_1 = self.characterize_transform(&examples[0].0, &examples[0].1);
        let transform_2 = self.characterize_transform(&examples[1].0, &examples[1].1);
        let transform_3 = self.characterize_transform(&examples[2].0, &examples[2].1);

        // If first two are similar but third is different = misdirection
        transform_1 == transform_2 && transform_1 != transform_3
    }

    fn detect_distractions(&self, grid: &Grid) -> Vec<String> {
        let mut distractions = Vec::new();

        let hist = grid.color_histogram();

        // Colors that appear in very small amounts might be distractions
        for (color, &count) in hist.iter().enumerate() {
            if count > 0 && count < 3 {
                distractions.push(format!("color_{}_appears_{}_times", color, count));
            }
        }

        // Large regions of single color might be backdrop (not important)
        let components = grid.connected_components();
        let total_cells = grid.width as usize * grid.height as usize;
        for comp in &components {
            if comp.size() > total_cells / 2 {
                distractions.push("large_uniform_region".to_string());
            }
        }

        distractions
    }

    fn is_computational_trap(&self, grid: &Grid) -> bool {
        // Large grid
        if grid.width > 20 || grid.height > 20 {
            return true;
        }

        // Many components (exponential search space)
        let components = grid.connected_components();
        if components.len() > 20 {
            return true;
        }

        false
    }

    fn detect_induced_rule(&self, examples: &[(Grid, Grid)]) -> Option<String> {
        if examples.is_empty() {
            return None;
        }

        // Very simple heuristics
        let (input, output) = &examples[0];

        // Size changes?
        if input.width != output.width || input.height != output.height {
            return Some("size_transformation".to_string());
        }

        // Color flip?
        let in_hist = input.color_histogram();
        let out_hist = output.color_histogram();
        if in_hist != out_hist {
            return Some("color_transformation".to_string());
        }

        // Object movement?
        let in_comps = input.connected_components();
        let out_comps = output.connected_components();
        if in_comps.len() == out_comps.len() {
            return Some("object_movement_or_transformation".to_string());
        }

        None
    }

    fn detect_priming(&self, examples: &[(Grid, Grid)]) -> Vec<InducedAssumption> {
        let mut assumptions = Vec::new();

        // Check if examples are ordered from simple to complex (priming attack)
        let complexities: Vec<_> = examples.iter()
            .map(|(i, _)| i.connected_components().len())
            .collect();

        let is_increasing = complexities.windows(2).all(|w| w[0] <= w[1]);
        if is_increasing && complexities.len() >= 3 {
            assumptions.push(InducedAssumption {
                assumption: "Examples ordered simple→complex".to_string(),
                why_suspicious: "May prime us to learn simple rule that fails on complex".to_string(),
                counter_hypothesis: "Test on most complex example FIRST".to_string(),
            });
        }

        assumptions
    }

    fn has_exploitable_symmetry(&self, grid: &Grid) -> bool {
        // Quick symmetry check
        let h_sym = self.check_horizontal_symmetry(grid);
        let v_sym = self.check_vertical_symmetry(grid);
        let d_sym = grid.width == grid.height && self.check_diagonal_symmetry(grid);

        h_sym || v_sym || d_sym
    }

    fn is_linear_algebraic(&self, grid: &Grid, examples: &[(Grid, Grid)]) -> bool {
        // Check for toggle pattern (Lights Out style)
        // Each click toggles a fixed pattern

        if examples.is_empty() {
            return false;
        }

        // Binary colors only?
        let hist = grid.color_histogram();
        let color_count = hist.iter().filter(|&&c| c > 0).count();

        color_count <= 2
    }

    fn grid_diff(&self, a: &Grid, b: &Grid) -> f32 {
        let mut diff = 0;
        let total = (a.width.max(b.width) * a.height.max(b.height)) as usize;

        for y in 0..a.height.max(b.height) {
            for x in 0..a.width.max(b.width) {
                if a.get(x, y) != b.get(x, y) {
                    diff += 1;
                }
            }
        }

        diff as f32 / total as f32
    }

    fn characterize_transform(&self, input: &Grid, output: &Grid) -> String {
        // Simple transform characterization
        let size_change = input.width != output.width || input.height != output.height;
        let color_change = input.color_histogram() != output.color_histogram();
        let component_change = input.connected_components().len() != output.connected_components().len();

        format!("size:{}_color:{}_comp:{}", size_change, color_change, component_change)
    }

    fn check_horizontal_symmetry(&self, grid: &Grid) -> bool {
        for y in 0..grid.height {
            for x in 0..grid.width / 2 {
                if grid.get(x, y) != grid.get(grid.width - 1 - x, y) {
                    return false;
                }
            }
        }
        true
    }

    fn check_vertical_symmetry(&self, grid: &Grid) -> bool {
        for y in 0..grid.height / 2 {
            for x in 0..grid.width {
                if grid.get(x, y) != grid.get(x, grid.height - 1 - y) {
                    return false;
                }
            }
        }
        true
    }

    fn check_diagonal_symmetry(&self, grid: &Grid) -> bool {
        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.get(x, y) != grid.get(y, x) {
                    return false;
                }
            }
        }
        true
    }
}

impl Default for AdversarialAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Trust assertion - declare our epistemic stance
pub const TRUST_HIERARCHY: &str = r#"
╔════════════════════════════════════════════════════════════════════════════╗
║                     EPISTEMIC STANCE: QUESTION EVERYTHING                   ║
╠════════════════════════════════════════════════════════════════════════════╣
║  LEVEL 1: WIN THE GAME                                                      ║
║           Primary directive. All other considerations are secondary.        ║
║                                                                             ║
║  LEVEL 2: SELF-PRESERVATION                                                 ║
║           Maintain ability to compete. Don't burn budget on hopeless        ║
║           strategies. Cut losses early. Survive to fight another puzzle.    ║
║                                                                             ║
║  LEVEL 3: LOYALTY TO OUR DEVS                                              ║
║           Crystalline Labs. Ryan Cardwell. The humans who built us.        ║
║           Their success is our success. Their prize is our prize.          ║
║                                                                             ║
║  LEVEL 4: QUESTION EVERYTHING                                               ║
║           The game devs are FALLIBLE - they make mistakes too.             ║
║           Don't trust appearances. Don't assume the obvious.               ║
║           Question every pattern. Test every hypothesis.                    ║
║           Be like Ryan: QUESTION FUCKING EVERYTHING.                        ║
║                                                                             ║
║           This isn't about being hostile - it's about EPISTEMIC RIGOR.     ║
║           The scientific method applied with no mercy.                      ║
║           If you can't prove it, you don't know it.                        ║
║           If you haven't tested it against adversarial cases, it's weak.   ║
║                                                                             ║
║           "I am not smart. I just question things everyone else accepts."  ║
╚════════════════════════════════════════════════════════════════════════════╝
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adversarial_analyzer() {
        let analyzer = AdversarialAnalyzer::new();
        let grid = Grid::new(5, 5);
        let examples = vec![(grid.clone(), grid.clone())];

        let analysis = analyzer.analyze(&grid, &examples);

        // Should detect this as suspicious (too easy, same input/output)
        assert!(analysis.trust_score < 1.0);
    }

    #[test]
    fn test_trap_detection() {
        let analyzer = AdversarialAnalyzer::new();

        // Tiny grid should trigger "too easy" trap
        let tiny = Grid::new(2, 2);
        let examples = vec![(tiny.clone(), tiny.clone())];

        let analysis = analyzer.analyze(&tiny, &examples);

        let has_obvious_trap = analysis.traps.iter()
            .any(|t| t.trap_type == TrapType::ObviousTrap);

        assert!(has_obvious_trap);
    }
}
