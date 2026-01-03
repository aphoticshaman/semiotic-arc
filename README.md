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
├── ATLAS_TAXONOMY.md      # 16 abstraction types (ARC covers ~28%)
├── POD_METHODOLOGY.md     # Formal knowledge generation methodology
├── atlas_gen.py           # Meta-benchmark generator (working code)
├── src/
│   ├── rust/              # Core reasoning engine (Rust, 16 modules)
│   │   └── src/
│   │       ├── lib.rs             # Main engine (13-layer cognitive pipeline)
│   │       ├── grid.rs            # 64x64 packed 4-bit grid representation
│   │       ├── operator.rs        # GridOperator trait, compose(), invert()
│   │       ├── hypothesis.rs      # Beam search over operator compositions
│   │       ├── grammar.rs         # Formal DSL grammar for grid operations
│   │       ├── prior_bank.rs      # Amortized inference via compiled priors
│   │       ├── discrimination.rs  # Hypothesis space partitioning
│   │       ├── simulator.rs       # World simulation for off-policy planning
│   │       ├── planner.rs         # A* planning with viability tracking
│   │       ├── memory.rs          # 3-tier memory (Working/Episodic/Semantic)
│   │       ├── cognitive_regulation.rs  # CBT/DBT-inspired self-healing
│   │       ├── critical_analysis.rs     # Adversarial trap detection
│   │       └── [more...]
│   └── python/            # Training data generation
├── research/              # Research materials
│   ├── LLM_SYSTEM_PROMPTS.py       # 30+ system prompts for game-playing agents
│   ├── TACTICAL_GUIDE.md           # Semiotic analysis methodology
│   ├── CROSS_DOMAIN_INSIGHTS.md    # 55 eigenvalue-based insights
│   ├── MULTI_MODEL_PROMPTS.md      # Prompts for multi-model coordination
│   ├── ARC_AGI_3_INSIGHTS.md       # Interactive game architecture notes
│   ├── ANTI_CHEAT_ANALYSIS.md      # Human-like agent behavior patterns
│   └── ratchet_loss.py             # Asymmetric ratcheting loss
├── data/
│   └── annotations/       # Semiotic puzzle annotations
├── prompts/
│   └── system_prompt.md
├── docs/
│   └── METHODOLOGY.md     # Field manual
└── MANIFEST.md            # File inclusion manifest
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
