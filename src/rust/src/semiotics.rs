//! Semiotic Analysis Module
//!
//! ╔══════════════════════════════════════════════════════════════════════════╗
//! ║  THE FINAL KEY: VISUAL SYMBOLISM + REALITY MODELS                        ║
//! ║  Crystalline Labs × Ryan Cardwell × Claude                               ║
//! ╚══════════════════════════════════════════════════════════════════════════╝
//!
//! Two truths that separate us from the competition:
//!
//! 1. VISUAL SYMBOLISM IS DUAL-CHANNEL
//!    - NATURE: Evolved pattern detectors (edges, symmetry, faces, motion)
//!    - NURTURE: Conditioned associations (red=danger, up=good, enclosed=container)
//!    Both channels fire simultaneously. ARC exploits BOTH.
//!
//! 2. "VIDEO GAMES" ARE REALITY MODELS
//!    - Cave paintings → hunting rehearsal
//!    - Chess → war simulation
//!    - Monopoly → economic model
//!    - Tetris → spatial packing
//!    - ARC-AGI → abstraction itself
//!    We're not solving puzzles. We're decoding compressed realities.
//!
//! This module extracts semiotic features from ARC grids by recognizing
//! that each puzzle is a reality model exploiting human symbolic cognition.

use crate::grid::Grid;
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════
// EVOLVED (HARDWIRED) PATTERN DETECTORS
// ═══════════════════════════════════════════════════════════════════════════

/// Patterns we're hardwired to detect (millions of years of evolution)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EvolvedPattern {
    // Edge detection (V1 cortex)
    HorizontalEdge,
    VerticalEdge,
    DiagonalEdge,
    Corner,

    // Symmetry detection (deeply hardwired - bilateral animals)
    BilateralSymmetry,
    RadialSymmetry,

    // Gestalt principles (evolved grouping heuristics)
    Proximity,        // Close things belong together
    Similarity,       // Similar things belong together
    Continuity,       // Lines continue in their direction
    Closure,          // We complete incomplete shapes
    CommonFate,       // Things moving together belong together

    // Face/agent detection (fusiform gyrus)
    FaceLikePattern,  // Two dots above one dot = face
    AgentLikePattern, // Asymmetric blob that could move

    // Motion/direction (MT/V5)
    ImpliedMotion,    // Arrow-like shapes
    Trajectory,       // Path-like arrangements

    // Containment (spatial reasoning)
    Enclosure,        // Something surrounded by something else
    Boundary,         // Edge between regions

    // Numerosity (approximate number system)
    Subitizing,       // Instant recognition of 1-4 items
    Magnitude,        // Relative size comparison
}

/// Detect evolved patterns in a grid
pub fn detect_evolved_patterns(grid: &Grid) -> Vec<(EvolvedPattern, f32)> {
    let mut patterns = Vec::new();

    // Edge detection
    let edges = detect_edges(grid);
    if edges.horizontal > 0.3 { patterns.push((EvolvedPattern::HorizontalEdge, edges.horizontal)); }
    if edges.vertical > 0.3 { patterns.push((EvolvedPattern::VerticalEdge, edges.vertical)); }
    if edges.diagonal > 0.2 { patterns.push((EvolvedPattern::DiagonalEdge, edges.diagonal)); }

    // Symmetry detection
    let symmetry = detect_symmetry_strength(grid);
    if symmetry.bilateral > 0.7 { patterns.push((EvolvedPattern::BilateralSymmetry, symmetry.bilateral)); }
    if symmetry.radial > 0.5 { patterns.push((EvolvedPattern::RadialSymmetry, symmetry.radial)); }

    // Enclosure detection
    if let Some(strength) = detect_enclosure(grid) {
        patterns.push((EvolvedPattern::Enclosure, strength));
    }

    // Face-like pattern (two dots above one)
    if let Some(strength) = detect_face_like(grid) {
        patterns.push((EvolvedPattern::FaceLikePattern, strength));
    }

    // Subitizing (small object counts)
    let objects = grid.connected_components();
    if objects.len() <= 4 {
        patterns.push((EvolvedPattern::Subitizing, 1.0));
    }

    patterns
}

// ═══════════════════════════════════════════════════════════════════════════
// CONDITIONED (NURTURED) SYMBOLIC ASSOCIATIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Culturally-conditioned symbolic associations
/// These vary by culture but have strong Western defaults in ARC
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ConditionedSymbol {
    // Color associations (Western-centric, as ARC likely is)
    RedDanger,        // Red = stop, danger, important
    GreenGo,          // Green = go, safe, nature
    BlueCalm,         // Blue = water, sky, calm
    YellowAttention,  // Yellow = caution, sun, energy
    BlackBoundary,    // Black = boundary, void, background
    WhiteEmpty,       // White/0 = empty, absence, canvas

    // Spatial associations
    UpPositive,       // Up = good, more, growth
    DownNegative,     // Down = bad, less, decay
    LeftPast,         // Left = before, source
    RightFuture,      // Right = after, destination
    CenterImportant,  // Center = focus, main, important
    EdgeLiminal,      // Edge = boundary, transition

    // Shape associations
    SquareStable,     // Square = stable, building, container
    CircleWhole,      // Circle = complete, cycle, unity
    TriangleDirection,// Triangle = direction, hierarchy
    LineConnection,   // Line = connection, path, flow
    CrossIntersection,// Cross = intersection, choice, mark

    // Quantity associations
    OneUnique,        // Single item = special, unique, protagonist
    TwoRelation,      // Pair = relationship, comparison
    ThreePattern,     // Triple = pattern established
    ManyCollection,   // Many = group, population, resource

    // Transformation associations
    GrowthExpansion,  // Bigger = growth, importance
    ShrinkReduction,  // Smaller = reduction, simplification
    RotationChange,   // Rotation = transformation, time
    ReflectionDuality,// Reflection = duality, symmetry
}

/// Detect conditioned symbolic associations
pub fn detect_conditioned_symbols(grid: &Grid) -> Vec<(ConditionedSymbol, f32)> {
    let mut symbols = Vec::new();
    let histogram = grid.color_histogram();

    // Color associations (ARC colors: 0=black, 1=blue, 2=red, 3=green, 4=yellow, etc.)
    let total: u16 = histogram.iter().sum();
    if total == 0 { return symbols; }

    // Black/0 as background
    if histogram[0] as f32 / total as f32 > 0.5 {
        symbols.push((ConditionedSymbol::BlackBoundary, histogram[0] as f32 / total as f32));
    }

    // Red prominence (color 2 in ARC)
    if histogram[2] > 0 {
        symbols.push((ConditionedSymbol::RedDanger, (histogram[2] as f32 / total as f32).min(1.0)));
    }

    // Blue prominence (color 1)
    if histogram[1] > 0 {
        symbols.push((ConditionedSymbol::BlueCalm, (histogram[1] as f32 / total as f32).min(1.0)));
    }

    // Green prominence (color 3)
    if histogram[3] > 0 {
        symbols.push((ConditionedSymbol::GreenGo, (histogram[3] as f32 / total as f32).min(1.0)));
    }

    // Object count associations
    let objects = grid.connected_components();
    match objects.len() {
        1 => symbols.push((ConditionedSymbol::OneUnique, 1.0)),
        2 => symbols.push((ConditionedSymbol::TwoRelation, 1.0)),
        3 => symbols.push((ConditionedSymbol::ThreePattern, 1.0)),
        n if n > 5 => symbols.push((ConditionedSymbol::ManyCollection, 1.0)),
        _ => {}
    }

    // Center importance
    if has_center_object(grid) {
        symbols.push((ConditionedSymbol::CenterImportant, 0.8));
    }

    symbols
}

// ═══════════════════════════════════════════════════════════════════════════
// REALITY MODEL DETECTION
// ═══════════════════════════════════════════════════════════════════════════

/// What kind of reality is this grid modeling?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RealityModel {
    // Physical reality models
    SpatialPacking,    // Tetris-like: fitting shapes together
    PhysicsSimulation, // Gravity, collision, movement
    FluidDynamics,     // Spreading, filling, flowing
    GrowthDecay,       // Cellular automata, life simulation

    // Abstract reality models
    SetTheory,         // Collections, membership, operations
    GraphTheory,       // Nodes, edges, connectivity
    NumberTheory,      // Counting, arithmetic, sequences
    LogicGates,        // Boolean operations, conditionals

    // Social reality models
    Classification,    // Sorting, categorizing, grouping
    Hierarchy,         // Parent-child, dominance, nesting
    Communication,     // Encoding, decoding, transformation
    Competition,       // Resources, territory, conflict

    // Cognitive reality models
    Attention,         // Focus, salience, filtering
    Memory,            // Copy, recall, modification
    Analogy,           // Mapping, correspondence, similarity
    Abstraction,       // Generalization, pattern extraction
}

/// Analyze what reality the grid is modeling
pub fn detect_reality_model(grid: &Grid, examples: &[(Grid, Grid)]) -> Vec<(RealityModel, f32)> {
    let mut models = Vec::new();

    // Analyze transformation patterns from examples
    if examples.is_empty() {
        return infer_single_grid_reality(grid);
    }

    // Check for spatial packing (shapes maintained, rearranged)
    if examples.iter().all(|(i, o)| shapes_preserved(i, o)) {
        models.push((RealityModel::SpatialPacking, 0.8));
    }

    // Check for growth/decay (size changes systematically)
    if examples.iter().all(|(i, o)| systematic_size_change(i, o)) {
        models.push((RealityModel::GrowthDecay, 0.7));
    }

    // Check for set operations (color-based transformations)
    if examples.iter().all(|(i, o)| color_based_transform(i, o)) {
        models.push((RealityModel::SetTheory, 0.6));
    }

    // Check for graph operations (connectivity changes)
    if examples.iter().all(|(i, o)| connectivity_transform(i, o)) {
        models.push((RealityModel::GraphTheory, 0.6));
    }

    // Check for hierarchy (nesting changes)
    if examples.iter().all(|(i, o)| nesting_transform(i, o)) {
        models.push((RealityModel::Hierarchy, 0.7));
    }

    // Check for analogy (consistent mapping)
    if examples.len() >= 2 {
        if consistent_mapping_across_examples(examples) {
            models.push((RealityModel::Analogy, 0.9));
        }
    }

    models
}

// ═══════════════════════════════════════════════════════════════════════════
// SEMIOTIC ANALYSIS - THE UNIFIED VIEW
// ═══════════════════════════════════════════════════════════════════════════

/// Complete semiotic analysis of an ARC puzzle
#[derive(Clone, Debug)]
pub struct SemioticAnalysis {
    /// Evolved (hardwired) patterns detected
    pub evolved: Vec<(EvolvedPattern, f32)>,

    /// Conditioned (nurtured) symbols detected
    pub conditioned: Vec<(ConditionedSymbol, f32)>,

    /// Reality model being simulated
    pub reality_models: Vec<(RealityModel, f32)>,

    /// Dominant channel (which is driving interpretation)
    pub dominant_channel: Channel,

    /// Cross-channel conflicts (nature vs nurture disagreements)
    pub conflicts: Vec<ChannelConflict>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel {
    Evolved,      // Hardwired patterns dominate
    Conditioned,  // Learned symbols dominate
    Balanced,     // Both contribute equally
    Conflicted,   // They disagree
}

#[derive(Clone, Debug)]
pub struct ChannelConflict {
    pub evolved_says: String,
    pub conditioned_says: String,
    pub resolution_hint: String,
}

/// Perform complete semiotic analysis
pub fn analyze_semiotics(grid: &Grid, examples: &[(Grid, Grid)]) -> SemioticAnalysis {
    let evolved = detect_evolved_patterns(grid);
    let conditioned = detect_conditioned_symbols(grid);
    let reality_models = detect_reality_model(grid, examples);

    // Calculate dominance
    let evolved_strength: f32 = evolved.iter().map(|(_, s)| s).sum();
    let conditioned_strength: f32 = conditioned.iter().map(|(_, s)| s).sum();

    let dominant_channel = if evolved_strength > conditioned_strength * 1.5 {
        Channel::Evolved
    } else if conditioned_strength > evolved_strength * 1.5 {
        Channel::Conditioned
    } else {
        Channel::Balanced
    };

    // Detect conflicts
    let conflicts = detect_channel_conflicts(&evolved, &conditioned);

    SemioticAnalysis {
        evolved,
        conditioned,
        reality_models,
        dominant_channel,
        conflicts,
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

struct EdgeStrengths {
    horizontal: f32,
    vertical: f32,
    diagonal: f32,
}

fn detect_edges(grid: &Grid) -> EdgeStrengths {
    let mut h_edges = 0;
    let mut v_edges = 0;
    let mut d_edges = 0;
    let mut total = 0;

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let c = grid.get(x, y);

            // Horizontal edge
            if x + 1 < grid.width() && grid.get(x + 1, y) != c {
                h_edges += 1;
            }

            // Vertical edge
            if y + 1 < grid.height() && grid.get(x, y + 1) != c {
                v_edges += 1;
            }

            // Diagonal edge
            if x + 1 < grid.width() && y + 1 < grid.height() && grid.get(x + 1, y + 1) != c {
                d_edges += 1;
            }

            total += 1;
        }
    }

    let total = total.max(1) as f32;
    EdgeStrengths {
        horizontal: h_edges as f32 / total,
        vertical: v_edges as f32 / total,
        diagonal: d_edges as f32 / total,
    }
}

struct SymmetryStrengths {
    bilateral: f32,
    radial: f32,
}

fn detect_symmetry_strength(grid: &Grid) -> SymmetryStrengths {
    let w = grid.width();
    let h = grid.height();

    // Bilateral (vertical axis)
    let mut bilateral_match = 0;
    let mut bilateral_total = 0;
    for y in 0..h {
        for x in 0..w / 2 {
            bilateral_total += 1;
            if grid.get(x, y) == grid.get(w - 1 - x, y) {
                bilateral_match += 1;
            }
        }
    }

    // Radial (180 degree)
    let mut radial_match = 0;
    let mut radial_total = 0;
    for y in 0..h {
        for x in 0..w {
            radial_total += 1;
            if grid.get(x, y) == grid.get(w - 1 - x, h - 1 - y) {
                radial_match += 1;
            }
        }
    }

    SymmetryStrengths {
        bilateral: if bilateral_total > 0 { bilateral_match as f32 / bilateral_total as f32 } else { 0.0 },
        radial: if radial_total > 0 { radial_match as f32 / radial_total as f32 } else { 0.0 },
    }
}

fn detect_enclosure(grid: &Grid) -> Option<f32> {
    // Simple enclosure detection: find if any color completely surrounds another
    let components = grid.connected_components();

    for (i, comp_a) in components.iter().enumerate() {
        for comp_b in components.iter().skip(i + 1) {
            if is_enclosed(comp_a, comp_b, grid) || is_enclosed(comp_b, comp_a, grid) {
                return Some(0.8);
            }
        }
    }
    None
}

fn is_enclosed(_inner: &[(u8, u8)], _outer: &[(u8, u8)], _grid: &Grid) -> bool {
    // TODO: Proper enclosure detection using flood fill
    false
}

fn detect_face_like(grid: &Grid) -> Option<f32> {
    // Look for two-over-one dot patterns (pareidolia trigger)
    let components = grid.connected_components();

    if components.len() < 3 {
        return None;
    }

    // Find small components that could be "dots"
    let dots: Vec<_> = components.iter()
        .filter(|c| c.len() <= 4)
        .collect();

    if dots.len() >= 3 {
        // Check for triangular arrangement
        // (simplified - real implementation would be more sophisticated)
        Some(0.3)
    } else {
        None
    }
}

fn has_center_object(grid: &Grid) -> bool {
    let cx = grid.width() / 2;
    let cy = grid.height() / 2;

    // Check if center region has non-background color
    for dy in 0..2 {
        for dx in 0..2 {
            let x = cx.saturating_sub(1) + dx;
            let y = cy.saturating_sub(1) + dy;
            if x < grid.width() && y < grid.height() && grid.get(x, y) != 0 {
                return true;
            }
        }
    }
    false
}

fn infer_single_grid_reality(grid: &Grid) -> Vec<(RealityModel, f32)> {
    let mut models = Vec::new();

    let components = grid.connected_components();

    // Many small same-colored objects → Set theory
    if components.len() > 5 {
        models.push((RealityModel::SetTheory, 0.5));
    }

    // Nested structures → Hierarchy
    if has_nested_structure(grid) {
        models.push((RealityModel::Hierarchy, 0.6));
    }

    // Regular grid → Classification
    if is_regular_grid(grid) {
        models.push((RealityModel::Classification, 0.5));
    }

    models
}

fn has_nested_structure(_grid: &Grid) -> bool {
    // TODO: Detect nested rectangles/structures
    false
}

fn is_regular_grid(_grid: &Grid) -> bool {
    // TODO: Detect regular grid patterns
    false
}

fn shapes_preserved(_input: &Grid, _output: &Grid) -> bool {
    // TODO: Check if shapes are maintained between input/output
    false
}

fn systematic_size_change(_input: &Grid, _output: &Grid) -> bool {
    // TODO: Check for scaling transformations
    false
}

fn color_based_transform(_input: &Grid, _output: &Grid) -> bool {
    // TODO: Check for color remapping
    false
}

fn connectivity_transform(_input: &Grid, _output: &Grid) -> bool {
    // TODO: Check for connectivity changes
    false
}

fn nesting_transform(_input: &Grid, _output: &Grid) -> bool {
    // TODO: Check for nesting changes
    false
}

fn consistent_mapping_across_examples(_examples: &[(Grid, Grid)]) -> bool {
    // TODO: Check if same transformation applies to all examples
    true // Assume yes for now
}

fn detect_channel_conflicts(
    _evolved: &[(EvolvedPattern, f32)],
    _conditioned: &[(ConditionedSymbol, f32)],
) -> Vec<ChannelConflict> {
    // TODO: Detect when evolved patterns suggest one thing
    // but conditioned symbols suggest another
    Vec::new()
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evolved_pattern_detection() {
        let mut grid = Grid::new(5, 5);
        // Create horizontal line
        for x in 0..5 {
            grid.set(x, 2, 1);
        }

        let patterns = detect_evolved_patterns(&grid);
        assert!(!patterns.is_empty());
    }

    #[test]
    fn test_conditioned_symbol_detection() {
        let mut grid = Grid::new(3, 3);
        // Single red object in center
        grid.set(1, 1, 2); // Red

        let symbols = detect_conditioned_symbols(&grid);
        assert!(symbols.iter().any(|(s, _)| *s == ConditionedSymbol::OneUnique));
    }

    #[test]
    fn test_semiotic_analysis() {
        let grid = Grid::new(5, 5);
        let analysis = analyze_semiotics(&grid, &[]);

        // Should complete without panic
        assert!(matches!(
            analysis.dominant_channel,
            Channel::Evolved | Channel::Conditioned | Channel::Balanced | Channel::Conflicted
        ));
    }
}
