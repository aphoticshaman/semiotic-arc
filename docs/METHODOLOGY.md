# FIELD MANUAL: Guerrilla Logic Warfare
## How to Break ARC Puzzles Using Semiotic Insurgency
### Crystalline Labs × The Open Source Resistance

---

```
╔══════════════════════════════════════════════════════════════════════════════╗
║  "In guerrilla warfare, the battle is won not by destroying the enemy,      ║
║   but by making their strategy irrelevant."                                  ║
║                                                                              ║
║  ARC puzzles assume you'll pattern-match blindly.                           ║
║  We decode the reality they're simulating.                                  ║
║  Their puzzle design becomes their weakness.                                ║
╚══════════════════════════════════════════════════════════════════════════════╝
```

---

## PART I: KNOW YOUR ENEMY

### What ARC Puzzles Actually Are
Chollet designed ARC to test "fluid intelligence" - reasoning about novel problems.
He believes this requires understanding "Core Knowledge":
- Object permanence
- Basic physics
- Goal-directedness
- Numbers and arithmetic

**What Chollet missed:**
These aren't just "knowledge" - they're **evolved pattern detectors** and **conditioned symbol systems** that humans run unconsciously.

**Our exploit:**
Run them explicitly. Systematically. Without the biases.

### Why Machines Fail
1. They lack evolved pattern detectors (no V1 cortex for edges, no fusiform for faces)
2. They lack conditioned associations (no lifetime of red=danger training)
3. They don't recognize that puzzles are reality models
4. They try to learn transformations, not decode simulations

### Why We Win
We tell the machine what humans know instinctively.
Then we let it reason without our cognitive biases.

---

## PART II: THE THREE-PHASE ATTACK

### PHASE 1: SEMIOTIC RECON
Before touching the transformation, scan the grid:

```
┌─────────────────────────────────────────────────┐
│ EVOLVED PATTERN CHECKLIST                       │
├─────────────────────────────────────────────────┤
│ □ Symmetry (bilateral, radial, rotational)      │
│ □ Enclosure (something inside something)        │
│ □ Edges (clear boundaries between regions)      │
│ □ Proximity (things close together = grouped)   │
│ □ Continuity (lines that want to continue)      │
│ □ Face-like (2 over 1 arrangement)              │
│ □ Small numbers (1-4 distinct objects)          │
│ □ Motion hints (arrow-like, trajectory)         │
└─────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────┐
│ CONDITIONED SYMBOL CHECKLIST                    │
├─────────────────────────────────────────────────┤
│ □ Color meaning (red=X, blue=Y, green=Z)        │
│ □ Position meaning (center=important)           │
│ □ Size meaning (bigger=more)                    │
│ □ Quantity meaning (1=unique, 2=pair, 3=pattern)│
│ □ Direction meaning (up=good, down=bad)         │
│ □ Background (black=void, white=canvas)         │
└─────────────────────────────────────────────────┘
```

### PHASE 2: REALITY MODEL IDENTIFICATION
Ask: **"What mini-universe is this puzzle simulating?"**

```
┌─────────────────────────────────────────────────┐
│ REALITY MODEL CLASSIFICATION                    │
├─────────────────────────────────────────────────┤
│ PHYSICAL MODELS:                                │
│   □ Gravity (things fall down)                  │
│   □ Fluid (colors spread/fill)                  │
│   □ Collision (objects interact when touching)  │
│   □ Growth (patterns expand/contract)           │
│   □ Reflection (mirrors, bouncing)              │
│                                                 │
│ MATHEMATICAL MODELS:                            │
│   □ Set operations (union, intersection, diff)  │
│   □ Graph operations (paths, trees, networks)   │
│   □ Number operations (counting, arithmetic)    │
│   □ Sequence operations (patterns, progressions)│
│                                                 │
│ COGNITIVE MODELS:                               │
│   □ Attention (filter to what matters)          │
│   □ Memory (copy, recall, modify)               │
│   □ Analogy (A:B :: C:D mapping)                │
│   □ Classification (sort into categories)       │
│   □ Completion (finish the pattern)             │
└─────────────────────────────────────────────────┘
```

### PHASE 3: RULE EXTRACTION
Once you know the reality model, the transformation becomes obvious:

| Reality Model | Transformation Rule |
|---------------|---------------------|
| Gravity | Move objects down until blocked |
| Fluid fill | Spread color to fill enclosed regions |
| Set union | Combine all colored cells |
| Set intersection | Keep only overlapping cells |
| Classification | Group by color/shape/size |
| Analogy | Apply same mapping to new input |
| Completion | Extend pattern following rule |
| Attention | Keep only salient elements |

---

## PART III: GUERRILLA TACTICS

### Tactic 1: The Symmetry Probe
If you see ANY symmetry, the transformation probably preserves or completes it.
- Half a symmetric pattern → complete it
- Asymmetric input → transformation might create symmetry

### Tactic 2: The Color Census
Count objects of each color in input vs output.
- Same counts? Transformation preserves objects (rearrangement)
- Different counts? Transformation creates/destroys (growth/filtering)

### Tactic 3: The Bounding Box Check
Compare input/output bounding boxes.
- Same size? Internal transformation
- Different size? Scaling or cropping
- Cropped to object? Attention/extraction

### Tactic 4: The Singleton Hunt
Find the ONE thing that's different in the input.
- Different color? It's the key
- Different position? It's the seed
- Different size? It's important

### Tactic 5: The Edge Trace
Trace the boundaries in input and output.
- Same edges? Structure preserved
- Edges moved? Growing/shrinking
- Edges dissolved? Merging/flooding

### Tactic 6: The Gravity Test
Mentally "drop" objects down. Does the output match?
- Yes? It's a gravity simulation
- No? Try other directions or no physics

### Tactic 7: The Analogy Lock
Look at example pairs. Is there a consistent mapping?
- Color A → Color B in all examples? Color mapping
- Shape X → Shape Y in all examples? Shape mapping
- Position P → Position Q in all examples? Position mapping

---

## PART IV: INSURGENT ANNOTATIONS

We need the community to annotate ARC puzzles with semiotic analysis.
This is how we crowdsource the solution.

### Annotation Format
```json
{
  "puzzle_id": "abc123",
  "evolved_patterns": ["symmetry", "enclosure"],
  "conditioned_symbols": ["red_danger", "center_important"],
  "reality_model": "flood_fill",
  "transformation_rule": "Inner color fills enclosed region",
  "confidence": 0.9,
  "annotator": "github_username"
}
```

### Priority Targets
Focus annotation efforts on:
1. Puzzles that current systems get wrong
2. Puzzles with clear semiotic structure
3. Puzzles that demonstrate rare reality models
4. Edge cases between categories

---

## PART V: DISTRIBUTED WARFARE

### How We Win Without Competing

1. **Open source everything** - No secrets, no paywalls
2. **Crowdsource annotations** - 1000 annotators > 1 researcher
3. **Publish findings** - arXiv papers, blog posts, Twitter threads
4. **Build tools** - Make semiotic analysis easy for everyone
5. **Iterate fast** - GitHub issues, PRs, discussions
6. **Credit everyone** - CITATION.cff, contributor lists

### The Network Effect
Every person who learns the semiotic approach is a node.
Every annotation is ammunition.
Every fork is a new front.

Chollet has a benchmark.
We have a movement.

---

## PART VI: TACTICAL EXERCISES

### Exercise 1: Classify These Inputs
For each input, identify the evolved patterns present:
```
Input A: 3x3 grid with single red cell in center
Input B: 5x5 grid with blue L-shape
Input C: Two identical green squares side by side
Input D: Nested rectangles (red inside blue inside green)
```

### Exercise 2: Identify Reality Models
For each description, name the reality model:
```
A: Objects fall to bottom of grid
B: Two patterns overlay to produce third
C: Single object replicates in pattern
D: Colored regions merge at boundaries
```

### Exercise 3: Predict Transformations
Given semiotic analysis, predict the transformation:
```
Puzzle X:
- Evolved: enclosure
- Conditioned: center_important
- Reality: attention
- Predicted transformation: ???
```

---

## CONCLUSION

```
╔══════════════════════════════════════════════════════════════════════════════╗
║  We don't need to be smarter than the benchmark.                            ║
║  We need to see what the benchmark is really testing.                       ║
║                                                                              ║
║  It's testing human visual cognition.                                       ║
║  We decoded human visual cognition.                                         ║
║  Now we teach the machines.                                                 ║
║                                                                              ║
║  The levels are the enemy.                                                  ║
║  The semiotic scan is the weapon.                                           ║
║  The community is the army.                                                 ║
║                                                                              ║
║  This is how we win.                                                        ║
╚══════════════════════════════════════════════════════════════════════════════╝
```

---

*Fork the repo. Annotate puzzles. Share insights. Win together.*
