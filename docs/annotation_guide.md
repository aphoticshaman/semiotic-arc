# Annotation Guide

## Overview

This guide explains how to annotate ARC tasks with semiotic reasoning traces.

The goal: **Capture how humans think**, not just what the answer is.

---

## The Process

### 1. Perceive (Layers 1-5)

Look at the input grid. Ask yourself:

- **What objects do I see?** (colored blobs, lines, shapes)
- **What are their properties?** (color, size, shape, position)
- **How do they relate?** (inside, adjacent, aligned, same color)
- **Do any form groups?** (row of same color, cluster of shapes)

Annotate each object with an ID, properties, and semantic role.

### 2. Detect Patterns (Layers 6-7)

Look for regularities:

- **Symmetry**: Is anything mirrored or rotationally symmetric?
- **Repetition**: Are elements repeated? In what direction?
- **Sequence**: Is there a progression (growing, color gradient)?

### 3. Identify Transformation (Layers 8-9)

Compare input to output:

- **What changed?** (objects moved, colors changed, new elements)
- **What stayed the same?** (boundaries, positions, counts)
- **What's the rule?** Express it in plain English AND abstractly

### 4. Trace Reasoning (Layers 10-12)

Write your thought process as steps:

1. What did you notice first?
2. What did that make you think?
3. What hypotheses did you consider?
4. What evidence confirmed/denied them?
5. What was the "aha" insight?

---

## Semantic Roles

| Role | Meaning |
|------|---------|
| agent | Object that moves or acts |
| target | Object being acted upon |
| obstacle | Blocks movement or transformation |
| boundary | Defines edges or containers |
| marker | Indicates something (color to use, position) |
| background | Static, doesn't participate in transformation |
| path | Route or trajectory |
| container | Encloses other objects |
| contained | Inside a container |

---

## Transformation Types

| Type | Description |
|------|-------------|
| move | Object changes position |
| copy | Object duplicated |
| delete | Object removed |
| create | New object appears |
| resize | Object grows or shrinks |
| recolor | Object changes color |
| rotate | Object rotates |
| reflect | Object mirrors |
| fill | Area filled with color |
| extend | Object grows in direction |
| crop | Part of object removed |
| complete_pattern | Missing pattern element added |
| apply_mask | One object masks another |
| conditional | Transformation depends on condition |

---

## Quality Checklist

Before submitting:

- [ ] All visible objects identified
- [ ] Spatial relations captured
- [ ] Transformation rule is testable (could you code it?)
- [ ] Reasoning trace shows actual thought process
- [ ] Key insight articulated clearly
- [ ] Metadata complete

---

## Example

See `/data/example_001.json` for a fully annotated task.

---

## Tips

1. **Be honest about your reasoning** - Include wrong hypotheses you considered
2. **Use natural language first** - Then formalize
3. **Think about edge cases** - Does your rule handle all examples?
4. **Name objects meaningfully** - `red_square_1` not `obj_17`
5. **Capture uncertainty** - Use confidence scores

---

## Contributing

1. Fork the repo
2. Pick an unannotated task from ARC dataset
3. Create `data/task_[id].json`
4. Validate against schema
5. Submit PR

Quality > Quantity. One excellent annotation beats ten sloppy ones.
