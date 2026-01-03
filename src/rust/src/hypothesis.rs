//! Hypothesis Generator with Beam Search
//!
//! Gap 2 from feedback: Missing hypothesis search mechanism.
//!
//! This module implements beam search over operator compositions
//! to find transformations that map input → output.
//!
//! Key insight: The search space is exponential but bounded.
//! Beam search with good heuristics finds solutions quickly.

use crate::grid::Grid;
use crate::operator::{
    ComposedOperator, GridOperator, PrimitiveOp, Identity, Rotation, Flip,
    Translate, Scale, ColorMap, Crop, Fill, Tile,
};
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

/// A hypothesis about the input→output transformation
#[derive(Clone)]
pub struct Hypothesis {
    /// The composed operator
    pub operator: ComposedOperator,
    /// Primitive operations used (for serialization)
    pub primitives: Vec<PrimitiveOp>,
    /// Score (lower is better - number of differing cells + complexity penalty)
    pub score: f32,
    /// Number of I/O pairs this hypothesis fits
    pub fits: usize,
    /// Confidence in this hypothesis
    pub confidence: f32,
}

impl Hypothesis {
    pub fn new() -> Self {
        Self {
            operator: ComposedOperator::new(),
            primitives: vec![PrimitiveOp::Identity],
            score: f32::MAX,
            fits: 0,
            confidence: 0.0,
        }
    }

    pub fn identity() -> Self {
        Self {
            operator: ComposedOperator::from_single(Identity),
            primitives: vec![PrimitiveOp::Identity],
            score: 0.0,
            fits: 0,
            confidence: 1.0,
        }
    }

    /// Apply to input grid
    pub fn apply(&self, input: &Grid) -> Grid {
        self.operator.apply(input)
    }

    /// Complexity penalty (Occam's razor)
    pub fn complexity(&self) -> usize {
        self.operator.complexity()
    }

    /// Human-readable description
    pub fn describe(&self) -> String {
        self.operator.describe()
    }
}

impl PartialEq for Hypothesis {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Hypothesis {}

impl PartialOrd for Hypothesis {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Lower score is better (min-heap behavior with BinaryHeap)
        other.score.partial_cmp(&self.score)
    }
}

impl Ord for Hypothesis {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

/// I/O pair for testing hypotheses
#[derive(Clone)]
pub struct IOPair {
    pub input: Grid,
    pub output: Grid,
}

/// Hypothesis generator with beam search
pub struct HypothesisGenerator {
    /// Beam width (number of hypotheses to keep at each step)
    beam_width: usize,
    /// Maximum search depth (number of operator compositions)
    max_depth: usize,
    /// Complexity penalty weight
    complexity_weight: f32,
    /// Available primitive operators
    primitives: Vec<PrimitiveOp>,
}

impl HypothesisGenerator {
    pub fn new() -> Self {
        Self {
            beam_width: 100,
            max_depth: 5,
            complexity_weight: 0.1,
            primitives: Self::all_primitives(),
        }
    }

    pub fn with_beam_width(mut self, width: usize) -> Self {
        self.beam_width = width;
        self
    }

    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// All primitive operators to try
    fn all_primitives() -> Vec<PrimitiveOp> {
        let mut prims = PrimitiveOp::simple_primitives();

        // Add translations
        for d in [-2i8, -1, 1, 2] {
            prims.push(PrimitiveOp::Translate(d, 0));
            prims.push(PrimitiveOp::Translate(0, d));
            prims.push(PrimitiveOp::Translate(d, d));
            prims.push(PrimitiveOp::Translate(d, -d));
        }

        // Add scales
        prims.push(PrimitiveOp::Scale(2));
        prims.push(PrimitiveOp::Scale(3));

        // Add tiles
        prims.push(PrimitiveOp::Tile(2, 1));
        prims.push(PrimitiveOp::Tile(1, 2));
        prims.push(PrimitiveOp::Tile(2, 2));

        prims
    }

    /// Generate color mappings from I/O pair analysis
    fn infer_color_maps(&self, pairs: &[IOPair]) -> Vec<PrimitiveOp> {
        let mut maps = Vec::new();

        for pair in pairs {
            let in_hist = pair.input.color_histogram();
            let out_hist = pair.output.color_histogram();

            // Find potential color swaps
            for c1 in 0..10u8 {
                for c2 in (c1 + 1)..10u8 {
                    // Check if swapping c1 and c2 might help
                    if in_hist[c1 as usize] == out_hist[c2 as usize]
                        && in_hist[c2 as usize] == out_hist[c1 as usize]
                    {
                        let mut map = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
                        map[c1 as usize] = c2;
                        map[c2 as usize] = c1;
                        maps.push(PrimitiveOp::ColorMap(map));
                    }
                }
            }

            // Find potential color replacements
            for c1 in 0..10u8 {
                for c2 in 0..10u8 {
                    if c1 != c2
                        && in_hist[c1 as usize] > 0
                        && out_hist[c1 as usize] < in_hist[c1 as usize]
                    {
                        let mut map = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
                        map[c1 as usize] = c2;
                        maps.push(PrimitiveOp::ColorMap(map));
                    }
                }
            }
        }

        maps
    }

    /// Score a hypothesis against I/O pairs
    fn score_hypothesis(&self, hyp: &Hypothesis, pairs: &[IOPair]) -> (f32, usize) {
        let mut total_diff = 0u32;
        let mut fits = 0usize;

        for pair in pairs {
            let predicted = hyp.apply(&pair.input);
            let diff = predicted.diff(&pair.output);
            total_diff += diff;
            if diff == 0 {
                fits += 1;
            }
        }

        let score = total_diff as f32 + self.complexity_weight * hyp.complexity() as f32;
        (score, fits)
    }

    /// Beam search for hypotheses
    pub fn search(&self, pairs: &[IOPair]) -> Vec<Hypothesis> {
        if pairs.is_empty() {
            return vec![Hypothesis::identity()];
        }

        // Start with identity and simple primitives
        let mut beam: BinaryHeap<Hypothesis> = BinaryHeap::new();

        // Add identity
        let mut id_hyp = Hypothesis::identity();
        let (score, fits) = self.score_hypothesis(&id_hyp, pairs);
        id_hyp.score = score;
        id_hyp.fits = fits;
        beam.push(id_hyp);

        // Get color maps from I/O analysis
        let color_maps = self.infer_color_maps(pairs);

        // Add all primitives including inferred color maps
        let mut all_prims = self.primitives.clone();
        all_prims.extend(color_maps);

        for prim in &all_prims {
            let op = prim.to_operator();
            let mut hyp = Hypothesis {
                operator: ComposedOperator::from_single(Identity).then_compose(
                    ComposedOperator::new()
                ),
                primitives: vec![prim.clone()],
                score: 0.0,
                fits: 0,
                confidence: 0.0,
            };
            hyp.operator = make_composed(prim);
            let (score, fits) = self.score_hypothesis(&hyp, pairs);
            hyp.score = score;
            hyp.fits = fits;
            beam.push(hyp);
        }

        // Seen states to avoid duplicates
        let mut seen: HashSet<u64> = HashSet::new();

        // Iterative deepening beam search
        for depth in 0..self.max_depth {
            let mut next_beam: BinaryHeap<Hypothesis> = BinaryHeap::new();

            // Take top beam_width hypotheses
            let current: Vec<Hypothesis> = (0..self.beam_width)
                .filter_map(|_| beam.pop())
                .collect();

            // Check for perfect solution
            for hyp in &current {
                if hyp.fits == pairs.len() {
                    // Found a hypothesis that fits all pairs!
                    let mut results: Vec<Hypothesis> = current.clone();
                    results.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
                    return results.into_iter().take(10).collect();
                }
            }

            // Extend each hypothesis with each primitive
            for hyp in &current {
                for prim in &all_prims {
                    let mut new_prims = hyp.primitives.clone();
                    new_prims.push(prim.clone());

                    // Build new operator
                    let new_op = extend_composed(&hyp.primitives, prim);

                    let mut new_hyp = Hypothesis {
                        operator: new_op,
                        primitives: new_prims,
                        score: 0.0,
                        fits: 0,
                        confidence: 0.0,
                    };

                    // Compute hash for deduplication
                    let hash = compute_hyp_hash(&new_hyp, pairs);
                    if seen.contains(&hash) {
                        continue;
                    }
                    seen.insert(hash);

                    let (score, fits) = self.score_hypothesis(&new_hyp, pairs);
                    new_hyp.score = score;
                    new_hyp.fits = fits;
                    next_beam.push(new_hyp);
                }

                // Also keep the original
                next_beam.push(hyp.clone());
            }

            beam = next_beam;

            // Early exit if we found a perfect solution
            if let Some(best) = beam.peek() {
                if best.fits == pairs.len() {
                    break;
                }
            }
        }

        // Return top hypotheses
        let mut results: Vec<Hypothesis> = Vec::new();
        while let Some(hyp) = beam.pop() {
            results.push(hyp);
            if results.len() >= 10 {
                break;
            }
        }
        results.reverse();
        results
    }
}

/// Helper to make a composed operator from a primitive
fn make_composed(prim: &PrimitiveOp) -> ComposedOperator {
    let op = prim.to_operator();
    let mut composed = ComposedOperator::new();
    // We need to actually add the operator here
    // This is a limitation of the current design - we'll work around it
    match prim {
        PrimitiveOp::Identity => ComposedOperator::from_single(Identity),
        PrimitiveOp::RotateCW90 => ComposedOperator::from_single(Rotation::CW90),
        PrimitiveOp::RotateCW180 => ComposedOperator::from_single(Rotation::CW180),
        PrimitiveOp::RotateCW270 => ComposedOperator::from_single(Rotation::CW270),
        PrimitiveOp::FlipH => ComposedOperator::from_single(Flip::Horizontal),
        PrimitiveOp::FlipV => ComposedOperator::from_single(Flip::Vertical),
        PrimitiveOp::Translate(dx, dy) => ComposedOperator::from_single(Translate { dx: *dx, dy: *dy }),
        PrimitiveOp::Scale(f) => ComposedOperator::from_single(Scale { factor: *f }),
        PrimitiveOp::ColorMap(m) => ComposedOperator::from_single(ColorMap { map: *m }),
        PrimitiveOp::Crop(x, y, w, h) => ComposedOperator::from_single(Crop { x: *x, y: *y, w: *w, h: *h }),
        PrimitiveOp::Fill(x, y, w, h, c) => ComposedOperator::from_single(Fill { x: *x, y: *y, w: *w, h: *h, color: *c }),
        PrimitiveOp::Tile(rx, ry) => ComposedOperator::from_single(Tile { repeat_x: *rx, repeat_y: *ry }),
    }
}

/// Extend a composed operator with a new primitive
fn extend_composed(prims: &[PrimitiveOp], new_prim: &PrimitiveOp) -> ComposedOperator {
    let mut composed = ComposedOperator::new();
    for prim in prims {
        composed = composed.then_compose(make_composed(prim));
    }
    composed.then_compose(make_composed(new_prim))
}

/// Compute a hash for hypothesis deduplication
fn compute_hyp_hash(hyp: &Hypothesis, pairs: &[IOPair]) -> u64 {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    let mut hasher = DefaultHasher::new();

    // Hash the outputs on all inputs
    for pair in pairs {
        let output = hyp.apply(&pair.input);
        output.hash().hash(&mut hasher);
    }

    hasher.finish()
}

/// Verify a solution against I/O pairs
///
/// Gap 4 from feedback: This is the missing verification function.
pub fn verify_solution(hyp: &Hypothesis, pairs: &[IOPair]) -> VerificationResult {
    let mut correct = 0;
    let mut total_diff = 0u32;
    let mut details = Vec::new();

    for (i, pair) in pairs.iter().enumerate() {
        let predicted = hyp.apply(&pair.input);
        let diff = predicted.diff(&pair.output);

        if diff == 0 {
            correct += 1;
            details.push(VerificationDetail {
                pair_index: i,
                passed: true,
                diff_cells: 0,
                confidence: 1.0,
            });
        } else {
            details.push(VerificationDetail {
                pair_index: i,
                passed: false,
                diff_cells: diff,
                confidence: 1.0 - (diff as f32 / (pair.output.width as f32 * pair.output.height as f32)),
            });
        }
        total_diff += diff;
    }

    let all_pass = correct == pairs.len();
    let confidence = if pairs.is_empty() {
        1.0
    } else {
        correct as f32 / pairs.len() as f32
    };

    VerificationResult {
        passed: all_pass,
        correct_pairs: correct,
        total_pairs: pairs.len(),
        total_diff_cells: total_diff,
        confidence,
        details,
    }
}

/// Result of solution verification
#[derive(Clone, Debug)]
pub struct VerificationResult {
    /// Whether all I/O pairs passed
    pub passed: bool,
    /// Number of correctly predicted pairs
    pub correct_pairs: usize,
    /// Total number of pairs tested
    pub total_pairs: usize,
    /// Total number of differing cells across all pairs
    pub total_diff_cells: u32,
    /// Confidence score (0-1)
    pub confidence: f32,
    /// Per-pair details
    pub details: Vec<VerificationDetail>,
}

#[derive(Clone, Debug)]
pub struct VerificationDetail {
    pub pair_index: usize,
    pub passed: bool,
    pub diff_cells: u32,
    pub confidence: f32,
}

/// Apply a hypothesis to a test input to get predicted output
pub fn apply_hypothesis(hyp: &Hypothesis, test_input: &Grid) -> Grid {
    hyp.apply(test_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_hypothesis() {
        let mut input = Grid::new(3, 3);
        input.set(1, 1, 5);

        let hyp = Hypothesis::identity();
        let output = hyp.apply(&input);

        assert_eq!(input.hash(), output.hash());
    }

    #[test]
    fn test_rotation_search() {
        let mut input = Grid::new(2, 3);
        input.set(0, 0, 1);
        input.set(1, 2, 2);

        let output = Rotation::CW90.apply(&input);

        let pairs = vec![IOPair { input, output }];
        let gen = HypothesisGenerator::new()
            .with_beam_width(50)
            .with_max_depth(2);

        let hypotheses = gen.search(&pairs);
        assert!(!hypotheses.is_empty());

        // The best hypothesis should find the rotation
        let best = &hypotheses[0];
        assert!(best.fits > 0);
    }

    #[test]
    fn test_verification() {
        let mut input = Grid::new(2, 2);
        input.set(0, 0, 1);

        let output = input.flip_horizontal();
        let pairs = vec![IOPair { input: input.clone(), output }];

        // Create a flip hypothesis manually
        let hyp = Hypothesis {
            operator: ComposedOperator::from_single(Flip::Horizontal),
            primitives: vec![PrimitiveOp::FlipH],
            score: 0.0,
            fits: 1,
            confidence: 1.0,
        };

        let result = verify_solution(&hyp, &pairs);
        assert!(result.passed);
        assert_eq!(result.correct_pairs, 1);
    }
}
