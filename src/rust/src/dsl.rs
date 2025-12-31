//! Domain-Specific Language for ARC rules
//!
//! The DSL captures the finite grammar of ARC transformations.
//! Chollet's "novel tasks" are novel COMPOSITIONS of these primitives.
//!
//! Key insight: The DSL is small (~50 primitives).
//! Novel = novel composition, not novel primitive.

use serde::{Deserialize, Serialize};

/// High-level rule classes (archetypes)
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuleClass {
    // Navigation games
    Navigation(NavigationRule),

    // Click/toggle games
    Toggle(ToggleRule),

    // Pattern completion games
    PatternFill(PatternRule),

    // Object manipulation games
    ObjectTransform(ObjectRule),

    // Puzzle games (Sokoban-like)
    PushPuzzle(PushRule),

    // Unknown (needs discrimination)
    Unknown,
}

/// Navigation rule variants
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NavigationRule {
    /// Simple grid movement, walls block
    SimpleWalls,
    /// Movement with keys/doors
    KeyDoor,
    /// Movement with health/hazards
    HealthHazard,
    /// Movement with collectibles
    Collectibles,
    /// Wraparound edges
    Wraparound,
}

/// Toggle rule variants
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToggleRule {
    /// Click toggles single cell
    SingleCell,
    /// Click toggles 3x3 neighborhood
    Neighborhood3x3,
    /// Click toggles cross pattern (+)
    CrossPattern,
    /// Click toggles row
    Row,
    /// Click toggles column
    Column,
    /// Click affects linked cells (non-local)
    LinkedCells,
}

/// Pattern completion rule variants
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PatternRule {
    /// Fill to match template
    MatchTemplate,
    /// Mirror/reflect pattern
    Mirror,
    /// Rotate to match
    Rotate,
    /// Scale pattern
    Scale,
    /// Translate pattern
    Translate,
}

/// Object transformation rule variants
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectRule {
    /// Move objects
    Move,
    /// Rotate objects
    Rotate,
    /// Scale objects
    Scale,
    /// Recolor objects
    Recolor,
    /// Clone/duplicate objects
    Clone,
    /// Remove objects
    Remove,
}

/// Push puzzle rule variants
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PushRule {
    /// Standard Sokoban
    Sokoban,
    /// Push with sliding (ice physics)
    Sliding,
    /// Push with multiple box types
    MultiBox,
}

/// Primitive operations in the DSL
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Primitive {
    // Movement
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    // Click/selection
    Click(u8, u8),
    Select(u8, u8),
    Deselect,

    // Grid operations
    Fill(u8),           // Fill with color
    Clear,              // Clear to background
    Toggle,             // Toggle state

    // Transforms
    RotateCW,           // Rotate 90° clockwise
    RotateCCW,          // Rotate 90° counter-clockwise
    FlipH,              // Flip horizontal
    FlipV,              // Flip vertical

    // Conditionals
    IfColor(u8),        // Conditional on color
    IfEmpty,            // Conditional on empty
    IfWall,             // Conditional on wall

    // Loops
    RepeatN(u8),        // Repeat N times
    RepeatUntilWall,    // Repeat until hitting wall
    RepeatUntilColor(u8),

    // Composition
    Sequence(Vec<Primitive>),
    Parallel(Vec<Primitive>),
}

/// A complete program in the DSL
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Program {
    pub primitives: Vec<Primitive>,
    pub rule_class: RuleClass,
}

impl Program {
    pub fn new(rule_class: RuleClass) -> Self {
        Self {
            primitives: Vec::new(),
            rule_class,
        }
    }

    /// Estimated complexity (for Occam's razor scoring)
    pub fn complexity(&self) -> usize {
        self.primitives.iter().map(|p| p.complexity()).sum()
    }
}

impl Primitive {
    /// Complexity of a single primitive
    pub fn complexity(&self) -> usize {
        match self {
            // Simple operations: 1
            Primitive::MoveUp
            | Primitive::MoveDown
            | Primitive::MoveLeft
            | Primitive::MoveRight
            | Primitive::Clear
            | Primitive::Toggle
            | Primitive::Deselect => 1,

            // Parameterized: 2
            Primitive::Click(_, _)
            | Primitive::Select(_, _)
            | Primitive::Fill(_)
            | Primitive::IfColor(_)
            | Primitive::RepeatN(_)
            | Primitive::RepeatUntilColor(_) => 2,

            // Transforms: 2
            Primitive::RotateCW
            | Primitive::RotateCCW
            | Primitive::FlipH
            | Primitive::FlipV => 2,

            // Conditionals without params: 1
            Primitive::IfEmpty | Primitive::IfWall | Primitive::RepeatUntilWall => 1,

            // Compositions: sum of children + 1
            Primitive::Sequence(children) | Primitive::Parallel(children) => {
                1 + children.iter().map(|c| c.complexity()).sum::<usize>()
            }
        }
    }
}

/// Rule signature for quick matching
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuleSignature {
    /// Does the rule involve movement?
    pub has_movement: bool,
    /// Does the rule involve clicking?
    pub has_clicking: bool,
    /// Does the rule involve pattern matching?
    pub has_patterns: bool,
    /// Does the rule have health/resource management?
    pub has_resources: bool,
    /// Is the grid state deterministic?
    pub is_deterministic: bool,
    /// Can actions be undone?
    pub is_reversible: bool,
}

impl RuleSignature {
    /// Create signature from rule class
    pub fn from_rule_class(rc: &RuleClass) -> Self {
        match rc {
            RuleClass::Navigation(_) => Self {
                has_movement: true,
                has_clicking: false,
                has_patterns: false,
                has_resources: false,
                is_deterministic: true,
                is_reversible: true,
            },
            RuleClass::Toggle(_) => Self {
                has_movement: false,
                has_clicking: true,
                has_patterns: true,
                has_resources: false,
                is_deterministic: true,
                is_reversible: true,
            },
            RuleClass::PatternFill(_) => Self {
                has_movement: false,
                has_clicking: true,
                has_patterns: true,
                has_resources: false,
                is_deterministic: true,
                is_reversible: false,
            },
            RuleClass::ObjectTransform(_) => Self {
                has_movement: false,
                has_clicking: true,
                has_patterns: true,
                has_resources: false,
                is_deterministic: true,
                is_reversible: true,
            },
            RuleClass::PushPuzzle(_) => Self {
                has_movement: true,
                has_clicking: false,
                has_patterns: false,
                has_resources: false,
                is_deterministic: true,
                is_reversible: false,
            },
            RuleClass::Unknown => Self {
                has_movement: false,
                has_clicking: false,
                has_patterns: false,
                has_resources: false,
                is_deterministic: false,
                is_reversible: false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_complexity() {
        assert_eq!(Primitive::MoveUp.complexity(), 1);
        assert_eq!(Primitive::Click(0, 0).complexity(), 2);

        let seq = Primitive::Sequence(vec![
            Primitive::MoveUp,
            Primitive::MoveDown,
        ]);
        assert_eq!(seq.complexity(), 3); // 1 + 1 + 1
    }
}
