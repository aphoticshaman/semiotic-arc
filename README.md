# Semiotic Reasoning for ARC-AGI

**Crystalline Labs x Ryan Cardwell**

---

## Core Thesis

Human visual cognition operates through two channels:
1. **Evolved pattern recognition** - Hardwired detectors for edges, symmetry, containment
2. **Conditioned symbolic reasoning** - Learned associations between visual forms and meaning

ARC-AGI puzzles are **compressed reality models** - they test the same cognitive primitives humans use to understand the physical world.

The gap between human and machine performance on ARC isn't computational - it's **representational**. Machines lack explicit models of how humans decompose visual scenes into meaningful symbols.

**This repo provides that explicit model.**

---

## What This Is

Training data and annotation schema for fine-tuning models on semiotic reasoning patterns.

- **Schema**: JSON format for annotating ARC tasks with cognitive primitives
- **Data**: Seed examples showing human reasoning traces
- **Guide**: Documentation for crowdsourced annotation

---

## The 13-Layer Cognitive Pipeline

```
Layer 1:  RAW PERCEPTION      - Pixel grid input
Layer 2:  FIGURE/GROUND       - Separate objects from background
Layer 3:  OBJECT PRIMITIVES   - Identify discrete entities
Layer 4:  SPATIAL RELATIONS   - Above/below/inside/adjacent
Layer 5:  GROUPING            - Cluster by color/shape/proximity
Layer 6:  SYMMETRY DETECTION  - Mirror/rotational/translational
Layer 7:  PATTERN EXTRACTION  - Repeating motifs, sequences
Layer 8:  TRANSFORMATION ID   - What changed between input/output
Layer 9:  RULE ABSTRACTION    - Generalize the transformation
Layer 10: SYMBOLIC BINDING    - Assign meaning to visual tokens
Layer 11: HYPOTHESIS SPACE    - Generate candidate solutions
Layer 12: VERIFICATION        - Test against examples
Layer 13: OUTPUT GENERATION   - Render the answer
```

---

## Schema Overview

Each annotated task includes:

```json
{
  "task_id": "arc_task_xyz",
  "layers": {
    "objects": [...],
    "relations": [...],
    "transformations": [...],
    "symbols": [...],
    "reasoning_trace": "..."
  }
}
```

See `/schema` for full specification.

---

## Why This Matters

The insight is free. Anyone can read this and understand.

But **training data** turns insight into capability. Fine-tuned models with semiotic reasoning patterns will outperform promptimg alone.

This is asymmetric warfare. We enable the community.

---

## Contributing

1. Read the annotation guide in `/docs`
2. Pick an unannotated task
3. Apply the schema
4. Submit PR

---

## License

MIT. Use it. Build on it. Win.
