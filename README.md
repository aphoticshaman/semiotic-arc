# Semiotic Reasoning for Abstract Visual Tasks

A dual-channel cognitive architecture for ARC-AGI and related benchmarks.

---

## Core Thesis

Human visual reasoning exploits two parallel systems:

1. **Evolved patterns** — Phylogenetically hardwired detectors (edges, symmetry, enclosure, faces)
2. **Conditioned symbols** — Ontogenetically learned associations (color meanings, spatial semantics)

Furthermore, visual puzzles are **compressed reality models** — they simulate physical, mathematical, or social phenomena.

ARC puzzles exploit both channels. Machines fail because they lack explicit access.

**We provide that access.**

---

## The Insight

> "I am not smart. I just question things everyone else accepts."

We questioned the assumption that ARC puzzles are arbitrary transformations.

They're not. They're reality simulations.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    SEMIOTIC REASONING ENGINE                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  INPUT: Grid + Training Examples                                │
│     │                                                           │
│     ▼                                                           │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ EVOLVED PATTERN DETECTION                                │   │
│  │ • Symmetry (bilateral, radial)                          │   │
│  │ • Enclosure (containment)                               │   │
│  │ • Proximity (grouping)                                  │   │
│  │ • Edges (boundaries)                                    │   │
│  └─────────────────────────────────────────────────────────┘   │
│     │                                                           │
│     ▼                                                           │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ CONDITIONED SYMBOL RECOGNITION                           │   │
│  │ • Color semantics (red=danger, blue=calm)               │   │
│  │ • Position semantics (center=important)                 │   │
│  │ • Quantity semantics (1=unique, 2=pair)                 │   │
│  └─────────────────────────────────────────────────────────┘   │
│     │                                                           │
│     ▼                                                           │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ REALITY MODEL INFERENCE                                  │   │
│  │ • Physical: gravity, fluid, collision                   │   │
│  │ • Mathematical: sets, graphs, sequences                 │   │
│  │ • Cognitive: attention, memory, analogy                 │   │
│  └─────────────────────────────────────────────────────────┘   │
│     │                                                           │
│     ▼                                                           │
│  OUTPUT: Transformation Rule                                    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Repository Structure

```
semiotic-arc/
├── src/
│   ├── rust/           # Core reasoning engine (Rust)
│   └── python/         # Training data generation
├── data/
│   └── annotations/    # Semiotic puzzle annotations
├── prompts/
│   └── system_prompt.md
├── docs/
│   └── METHODOLOGY.md  # Field manual
└── experiments/
```

---

## Quick Start

### Run the Rust engine

```bash
cd src/rust
cargo test
cargo build --release
```

### Generate training data

```bash
cd src/python
python generate_semiotic_training.py
```

---

## The Dataset

**This is where you come in.**

We provide the methodology. We provide the annotation schema. We provide seed examples.

You provide the annotations.

Every ARC puzzle annotated with semiotic analysis is ammunition.

See [`data/annotations/SCHEMA.md`](data/annotations/SCHEMA.md) for the annotation format.

---

## Contributing

1. Fork this repository
2. Annotate puzzles using the schema
3. Submit a pull request
4. Get credited in CITATION.cff

We don't win alone.

---

## Citation

```bibtex
@software{semiotic_arc_2024,
  author = {Cardwell, Ryan and {Crystalline Labs}},
  title = {Semiotic Reasoning for Abstract Visual Tasks},
  year = {2024},
  url = {https://github.com/crystalline-labs/semiotic-arc}
}
```

---

## License

MIT License. See [LICENSE](LICENSE).

---

## References

- Chollet, F. (2019). On the Measure of Intelligence. arXiv:1911.01547
- Peirce, C.S. (1931-58). Collected Papers of Charles Sanders Peirce
- Gibson, J.J. (1979). The Ecological Approach to Visual Perception
- Spelke, E.S. (2007). Core Knowledge. Developmental Science, 10(1)
- Lake, B.M. et al. (2017). Building Machines That Learn and Think Like People

---

*This is asymmetric warfare. The insight is free. Use it.*
