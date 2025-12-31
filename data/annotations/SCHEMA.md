# Semiotic Annotation Schema
## How to Annotate ARC Puzzles for the Resistance

---

## Overview

Each annotation captures the semiotic structure of an ARC puzzle:
- What evolved patterns it exploits
- What conditioned symbols it uses
- What reality model it simulates
- What the transformation rule is

**One annotation = one puzzle decoded.**

---

## JSON Schema

```json
{
  "puzzle_id": "string",           // ARC puzzle ID (e.g., "00576224")
  "annotator": "string",           // Your GitHub username
  "version": "1.0",                // Schema version
  "timestamp": "ISO8601",          // When annotated

  "evolved_patterns": [
    {
      "pattern": "string",         // Pattern type (see catalog below)
      "confidence": 0.0-1.0,       // How certain
      "location": "string",        // Where in grid (optional)
      "notes": "string"            // Explanation (optional)
    }
  ],

  "conditioned_symbols": [
    {
      "symbol": "string",          // Symbol type (see catalog below)
      "referent": "string",        // What color/element it applies to
      "confidence": 0.0-1.0,
      "notes": "string"
    }
  ],

  "reality_model": {
    "primary": "string",           // Main reality model
    "secondary": "string",         // Secondary model (optional)
    "confidence": 0.0-1.0,
    "evidence": ["string"]         // Why you think this
  },

  "transformation_rule": {
    "description": "string",       // Natural language description
    "formal": "string",            // DSL/code representation (optional)
    "edge_cases": ["string"]       // Known tricky cases
  },

  "channel_conflicts": [           // When nature vs nurture disagree
    {
      "evolved_says": "string",
      "conditioned_says": "string",
      "resolution": "string"
    }
  ],

  "difficulty_factors": {
    "ambiguity": 0.0-1.0,          // Multiple valid interpretations
    "complexity": 0.0-1.0,         // Computational complexity
    "novelty": 0.0-1.0             // How novel the pattern is
  }
}
```

---

## Evolved Pattern Catalog

| Pattern ID | Description | Detection Cue |
|------------|-------------|---------------|
| `bilateral_symmetry` | Mirror symmetry along axis | Left = Right or Top = Bottom |
| `radial_symmetry` | Symmetry around center point | Rotation-invariant |
| `enclosure` | Region inside another | Surrounded cells |
| `proximity_grouping` | Close objects = one group | Spatial clustering |
| `edge_boundary` | Clear boundary between regions | Color transitions |
| `face_like` | Two-over-one arrangement | Triangle of dots |
| `continuity` | Lines/curves want to continue | Aligned elements |
| `closure` | Incomplete shapes feel complete | Partial rectangles |
| `small_number` | 1-4 objects (subitizable) | Instant count |
| `motion_implied` | Arrow-like, trajectory | Directional shapes |

---

## Conditioned Symbol Catalog

| Symbol ID | Meaning | Common Referent |
|-----------|---------|-----------------|
| `red_danger` | Important, danger, stop | Color 2 in ARC |
| `blue_calm` | Water, sky, passive | Color 1 in ARC |
| `green_go` | Nature, proceed, growth | Color 3 in ARC |
| `yellow_attention` | Caution, energy, highlight | Color 4 in ARC |
| `black_boundary` | Background, void, edge | Color 0 in ARC |
| `up_positive` | Good, more, growth | Top of grid |
| `down_negative` | Bad, less, decay | Bottom of grid |
| `center_important` | Focus, main, target | Grid center |
| `edge_liminal` | Boundary, transition | Grid edges |
| `one_unique` | Special, protagonist | Single object |
| `two_relation` | Pair, dialogue, comparison | Two objects |
| `three_pattern` | Pattern established | Three objects |
| `bigger_more` | Important, dominant | Larger objects |

---

## Reality Model Catalog

| Model ID | Simulates | Transformation Type |
|----------|-----------|---------------------|
| `gravity` | Objects falling | Move down until blocked |
| `fluid_fill` | Liquid spreading | Fill enclosed region |
| `collision` | Objects interacting | Stop at boundaries |
| `growth` | Expansion/contraction | Cellular automata |
| `reflection` | Mirrors/bouncing | Flip across axis |
| `set_union` | Combining sets | OR of colored cells |
| `set_intersection` | Common elements | AND of colored cells |
| `set_difference` | Removal | Subtract one from another |
| `graph_path` | Connectivity | Path between nodes |
| `graph_tree` | Hierarchy | Parent-child relations |
| `classification` | Sorting/grouping | Organize by property |
| `analogy` | Consistent mapping | A:B :: C:D |
| `attention` | Filtering/salience | Keep important, drop rest |
| `memory` | Copy/recall | Duplicate or retrieve |
| `completion` | Finish pattern | Extend rule-following |

---

## Example Annotation

```json
{
  "puzzle_id": "00576224",
  "annotator": "ryancardwell",
  "version": "1.0",
  "timestamp": "2024-12-31T12:00:00Z",

  "evolved_patterns": [
    {
      "pattern": "enclosure",
      "confidence": 0.95,
      "location": "center",
      "notes": "Red rectangle contains blue dot"
    },
    {
      "pattern": "edge_boundary",
      "confidence": 0.9,
      "notes": "Clear boundary between red frame and interior"
    }
  ],

  "conditioned_symbols": [
    {
      "symbol": "blue_calm",
      "referent": "interior dot",
      "confidence": 0.8,
      "notes": "Blue suggests water/fluid that will spread"
    },
    {
      "symbol": "red_danger",
      "referent": "outer frame",
      "confidence": 0.7,
      "notes": "Red frame acts as container/boundary"
    }
  ],

  "reality_model": {
    "primary": "fluid_fill",
    "confidence": 0.9,
    "evidence": [
      "Interior element spreads to fill enclosed region",
      "Respects boundary of enclosing shape",
      "Final state has uniform interior"
    ]
  },

  "transformation_rule": {
    "description": "The color inside an enclosed region floods to fill the entire interior",
    "formal": "FILL(enclosed_region, inner_color)",
    "edge_cases": [
      "Multiple enclosed regions treated independently",
      "Nested enclosures fill innermost first"
    ]
  },

  "channel_conflicts": [],

  "difficulty_factors": {
    "ambiguity": 0.2,
    "complexity": 0.3,
    "novelty": 0.4
  }
}
```

---

## Annotation Guidelines

### DO:
- Be specific about locations and referents
- Provide evidence for reality model claims
- Note edge cases you're uncertain about
- Include confidence scores honestly
- Document channel conflicts when they exist

### DON'T:
- Annotate puzzles you haven't solved yourself
- Copy annotations from others without verification
- Leave fields empty - use `null` if truly N/A
- Annotate in batch without careful thought

### PRIORITY:
Focus on puzzles that:
1. Current systems get wrong
2. Have clear semiotic structure
3. Demonstrate rare reality models
4. Show interesting channel conflicts

---

## Submission

1. Create annotations in `data/annotations/` as `{puzzle_id}.json`
2. Run validation: `python validate_annotation.py {puzzle_id}.json`
3. Submit PR with your annotations
4. Get reviewed and merged

**Every annotation is ammunition. Make it count.**
