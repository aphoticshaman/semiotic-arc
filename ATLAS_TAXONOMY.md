# ATLAS: Abstraction Taxonomy for Learning and Adaptive Systems

**Version**: 1.0.0
**Author**: Crystalline Labs
**License**: CC BY-NC-SA 4.0  

---

## Abstract

We present ATLAS, a formal taxonomy of abstraction types required for general intelligence. We demonstrate that existing benchmarks, including ARC (Abstraction and Reasoning Corpus), test only a narrow subset of abstraction capabilities. We introduce a meta-benchmark generator that produces problem instances across the full taxonomy, enabling more comprehensive evaluation of machine intelligence.

---

## 1. Motivation

Current AI benchmarks suffer from a fundamental limitation: they operationalize "intelligence" or "abstraction" without formally defining the space of possible abstraction types. This leads to:

1. **Benchmark overfitting**: Systems optimized for specific abstraction types presented as "general"
2. **False progress signals**: Solving narrow benchmarks mistaken for broad capability
3. **Misdirected research**: Resources allocated to benchmark-specific solutions rather than general mechanisms

ARC (Chollet, 2019) represents a valuable contribution but explicitly tests only geometric/visual abstraction on discrete grids. Solving ARC demonstrates competence in ~4 of 16 identified abstraction types.

---

## 2. The Abstraction Taxonomy

### 2.1 Formal Definition

**Definition 2.1 (Abstraction)**: An abstraction A is a mapping:
```
A: D_concrete → D_abstract
```
where:
- D_concrete is a domain of specific instances
- D_abstract is a domain of general patterns
- A preserves task-relevant structure while discarding task-irrelevant detail

**Definition 2.2 (Abstraction Type)**: An abstraction type T is characterized by:
- **Source domain**: What kind of concrete instances
- **Target domain**: What kind of abstract patterns
- **Invariance class**: What transformations the abstraction is robust to
- **Composition rules**: How abstractions of this type combine

### 2.2 The Sixteen Abstraction Types

| ID | Type | Source → Target | Invariance Class | ARC Coverage |
|----|------|-----------------|------------------|--------------|
| A1 | **Geometric** | Spatial configs → Shapes | Rotation, reflection, translation | ✓ Full |
| A2 | **Chromatic** | Color patterns → Color rules | Palette permutation | ✓ Full |
| A3 | **Topological** | Connected regions → Connectivity | Continuous deformation | ✓ Partial |
| A4 | **Arithmetic** | Quantities → Numerical relations | Representation base | ✓ Partial |
| A5 | **Sequential** | Ordered items → Sequence rules | Index shift | ✗ None |
| A6 | **Temporal** | Events over time → Dynamics | Time scaling | ✗ None |
| A7 | **Causal** | Interventions → Mechanisms | Confound variation | ✗ None |
| A8 | **Hierarchical** | Parts → Wholes → Systems | Level shift | ✗ None |
| A9 | **Analogical** | Domain₁ → Domain₂ mapping | Surface features | ✗ None |
| A10 | **Linguistic** | Utterances → Meanings | Paraphrase | ✗ None |
| A11 | **Social** | Behaviors → Intentions | Actor substitution | ✗ None |
| A12 | **Counterfactual** | Actuals → Possibles | Intervention choice | ✗ None |
| A13 | **Recursive** | Patterns → Meta-patterns | Depth | ✗ None |
| A14 | **Probabilistic** | Samples → Distributions | Sample variation | ✗ None |
| A15 | **Compositional** | Primitives → Combinations | Primitive substitution | ✓ Partial |
| A16 | **Cross-modal** | Modality₁ → Modality₂ | Encoding format | ✗ None |

**ARC Coverage**: 4 full + 3 partial = ~28% of taxonomy

### 2.3 Type Definitions

#### A1: Geometric Abstraction
```
Source: Grid configurations G ∈ {0,1,...,9}^(n×m)
Target: Shape classes S = {rectangle, line, L-shape, ...}
Invariance: SO(2) rotations, reflections, translations
Test: "All rotations of this shape map to same output"
```

#### A5: Sequential Abstraction (NOT in ARC)
```
Source: Ordered sequences s = (s₁, s₂, ..., sₙ)
Target: Sequence generators f: ℕ → Σ
Invariance: Index shift (f(i) = f(i+k) under period)
Test: "Predict next element given pattern"
Example: Musical progressions, narrative arcs, proof steps
```

#### A6: Temporal Abstraction (NOT in ARC)
```
Source: State trajectories τ = {s(t) : t ∈ [0,T]}
Target: Dynamical laws ds/dt = f(s)
Invariance: Time scaling, initial condition variation
Test: "Predict future state given dynamics"
Example: Physics simulation, behavior prediction
```

#### A7: Causal Abstraction (NOT in ARC)
```
Source: Intervention-outcome pairs {(do(X=x), Y=y)}
Target: Causal graphs G = (V, E) with mechanisms
Invariance: Confound variation, context shift
Test: "Predict outcome of novel intervention"
Example: "If I do X instead of Y, what changes?"
```

#### A9: Analogical Abstraction (NOT in ARC)
```
Source: Relational structures R₁ = (E₁, R₁)
Target: Structure-preserving maps φ: R₁ → R₂
Invariance: Surface feature variation
Test: "Find corresponding element in new domain"
Example: "Atom is to nucleus as solar system is to ___"
```

#### A11: Social/Theory of Mind Abstraction (NOT in ARC)
```
Source: Observable behaviors B = {b₁, b₂, ...}
Target: Mental state attributions M = {beliefs, desires, intentions}
Invariance: Actor substitution, context variation
Test: "Predict behavior from inferred mental state"
Example: "Why did agent take that action?"
```

#### A13: Recursive/Meta Abstraction (NOT in ARC)
```
Source: Patterns at level L
Target: Patterns over patterns at level L+1
Invariance: Depth of recursion
Test: "Apply the pattern-finding operation to its own output"
Example: "The rule that generates rules"
```

### 2.4 Composition Algebra

Abstraction types compose:

```
A_i ∘ A_j : D_concrete → D_abstract(i) → D_abstract(j)
```

**Valid compositions** (produce meaningful abstractions):
- Geometric ∘ Hierarchical: Parts → Shapes → Assemblies
- Sequential ∘ Causal: Events → Sequences → Causal chains
- Analogical ∘ Cross-modal: Domain₁ → Domain₂ → Modality₂

**Invalid compositions** (degenerate):
- Chromatic ∘ Temporal: Color has no inherent dynamics
- Social ∘ Arithmetic: Mental states aren't quantities

---

## 3. Meta-Benchmark Generator (ATLAS-Gen)

### 3.1 Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    ATLAS-Gen                            │
├─────────────────────────────────────────────────────────┤
│  Input:                                                 │
│    - Abstraction type T ∈ {A1, ..., A16}               │
│    - Difficulty level d ∈ [1, 10]                      │
│    - Instance count n                                   │
│    - Composition depth c (for compound types)          │
│                                                         │
│  Process:                                               │
│    1. Sample concrete instances from D_concrete(T)     │
│    2. Apply abstraction A_T to generate pattern        │
│    3. Generate problem instance (input, output)        │
│    4. Validate solvability                             │
│    5. Calibrate difficulty                             │
│                                                         │
│  Output:                                                │
│    - Problem set P = {(input_i, output_i)}            │
│    - Ground truth abstractions {A_i}                   │
│    - Difficulty scores {d_i}                           │
│    - Type annotations {T_i}                            │
└─────────────────────────────────────────────────────────┘
```

### 3.2 Type-Specific Generators

#### Generator: A1 (Geometric) — ARC-Compatible
```python
def generate_geometric(difficulty: int) -> Problem:
    """Generate geometric abstraction problem."""
    # Sample shape class
    shape = sample_shape(complexity=difficulty)
    
    # Apply random geometric transform
    transform = sample_transform(SO2 + reflections + translations)
    
    # Generate input-output pair
    input_grid = render(shape)
    output_grid = render(transform(shape))
    
    # The abstraction: shape identity invariant under transform
    return Problem(
        input=input_grid,
        output=output_grid,
        abstraction_type="A1_geometric",
        ground_truth=f"shape_class={shape.class_id}"
    )
```

#### Generator: A7 (Causal) — NOT in ARC
```python
def generate_causal(difficulty: int) -> Problem:
    """Generate causal abstraction problem."""
    # Sample causal graph
    graph = sample_causal_graph(nodes=difficulty+2)
    
    # Generate observational data
    observations = simulate(graph, interventions=None, n=10)
    
    # Generate intervention query
    intervention = sample_intervention(graph)
    
    # The abstraction: causal mechanism
    return Problem(
        input={"observations": observations, "query": intervention},
        output=simulate(graph, interventions=intervention, n=1),
        abstraction_type="A7_causal",
        ground_truth=graph.to_dict()
    )
```

#### Generator: A9 (Analogical) — NOT in ARC
```python
def generate_analogical(difficulty: int) -> Problem:
    """Generate analogical abstraction problem."""
    # Sample source domain structure
    source = sample_relational_structure(complexity=difficulty)
    
    # Sample target domain (different surface, same structure)
    target = isomorphic_structure(source, new_domain=True)
    
    # Hide one element in target
    query_element = target.random_element()
    target_partial = target.remove(query_element)
    
    # The abstraction: structural correspondence
    return Problem(
        input={
            "source_domain": source,
            "target_domain_partial": target_partial,
            "query": f"What corresponds to {source.random_element()}?"
        },
        output=query_element,
        abstraction_type="A9_analogical",
        ground_truth=f"isomorphism={source.mapping_to(target)}"
    )
```

#### Generator: A13 (Recursive) — NOT in ARC
```python
def generate_recursive(difficulty: int) -> Problem:
    """Generate recursive/meta abstraction problem."""
    # Sample base pattern
    base_pattern = sample_pattern(complexity=difficulty//2)
    
    # Apply pattern to itself (meta-level)
    meta_pattern = apply_to_self(base_pattern, depth=difficulty//3 + 1)
    
    # Generate sequence showing recursion
    sequence = [base_pattern]
    for _ in range(3):
        sequence.append(apply_to_self(sequence[-1]))
    
    # The abstraction: the self-application rule
    return Problem(
        input=sequence[:-1],
        output=sequence[-1],
        abstraction_type="A13_recursive",
        ground_truth=f"meta_rule={base_pattern.self_application_rule()}"
    )
```

### 3.3 Difficulty Calibration

Difficulty is calibrated across types using:

```python
def calibrate_difficulty(problem: Problem, type: AbstractionType) -> float:
    """Normalize difficulty across abstraction types."""
    
    # Type-specific complexity metrics
    if type == "A1_geometric":
        raw = grid_size * num_objects * transform_complexity
    elif type == "A7_causal":
        raw = num_variables * graph_density * confound_count
    elif type == "A9_analogical":
        raw = domain_distance * structure_complexity * mapping_depth
    elif type == "A13_recursive":
        raw = recursion_depth * base_pattern_complexity
    # ... etc
    
    # Normalize to [1, 10] using human baseline data
    normalized = normalize_to_human_baseline(raw, type)
    
    return normalized
```

---

## 4. ARC as Subset: Formal Analysis

### 4.1 Coverage Mapping

```
ARC Problem Space ⊂ ATLAS Problem Space

Specifically:
ARC ⊆ (A1 ∪ A2 ∪ A3_partial ∪ A4_partial ∪ A15_partial)

Coverage ratio: |ARC| / |ATLAS| ≈ 0.28
```

### 4.2 What ARC Cannot Test

| Capability | Why ARC Cannot Test | ATLAS Type |
|------------|--------------------| ------------|
| Temporal reasoning | Static grids only | A6 |
| Causal inference | No interventions | A7 |
| Cross-domain analogy | Single domain (grids) | A9 |
| Language grounding | No linguistic input | A10 |
| Theory of mind | No agents | A11 |
| Counterfactual reasoning | No "what if" structure | A12 |
| Meta-learning | Fixed abstraction level | A13 |

### 4.3 Implications

**Claim**: A system that achieves 100% on ARC has demonstrated competence in geometric, chromatic, and simple compositional abstraction on discrete grids.

**Non-claim**: Such a system has demonstrated general abstraction capability, intelligence, or reasoning ability outside the tested types.

---

## 5. Benchmark Suite: ATLAS-16

We propose ATLAS-16, a comprehensive benchmark suite with:

- 100 problems per abstraction type
- 10 difficulty levels per type
- Calibrated to human performance baselines
- Composition problems testing type interactions
- Public evaluation set + held-out test set

### 5.1 Evaluation Protocol

```python
def evaluate_atlas16(solver: Solver) -> ATLASScore:
    """Comprehensive abstraction evaluation."""
    
    scores = {}
    for type_id in ABSTRACTION_TYPES:
        type_problems = load_problems(type_id)
        
        correct = 0
        for problem in type_problems:
            prediction = solver.solve(problem.input)
            if prediction == problem.output:
                correct += 1
        
        scores[type_id] = correct / len(type_problems)
    
    # Aggregate scores
    return ATLASScore(
        type_scores=scores,
        coverage=sum(1 for s in scores.values() if s > 0.5) / 16,
        arc_equivalent=mean([scores["A1"], scores["A2"], scores["A15"]]),
        beyond_arc=mean([scores[t] for t in NON_ARC_TYPES]),
        overall=mean(scores.values())
    )
```

### 5.2 Reporting Standard

Systems evaluated on ATLAS-16 must report:

1. **Type-level scores**: Performance on each of 16 types
2. **Coverage**: Fraction of types with >50% accuracy
3. **ARC-equivalent**: Performance on ARC-covered types (for comparison)
4. **Beyond-ARC**: Performance on types ARC cannot test
5. **Composition scores**: Performance on multi-type problems

This prevents "ARC-style" benchmark gaming by requiring breadth.

---

## 6. Conclusion

ARC was a valuable contribution that operationalized one notion of abstraction. However, conflating ARC performance with general abstraction capability is a category error. ATLAS provides:

1. **Formal taxonomy**: 16 abstraction types with mathematical definitions
2. **Coverage analysis**: ARC tests ~28% of the taxonomy
3. **Meta-generator**: Tool for creating problems across all types
4. **Comprehensive benchmark**: ATLAS-16 for honest evaluation

We release ATLAS-Gen and ATLAS-16 to enable more rigorous evaluation of machine abstraction capabilities.

---

## References

- Chollet, F. (2019). On the Measure of Intelligence. arXiv:1911.01547
- Gentner, D. (1983). Structure-mapping: A theoretical framework for analogy. Cognitive Science.
- Pearl, J. (2009). Causality: Models, Reasoning, and Inference.
- Hofstadter, D. (1979). Gödel, Escher, Bach: An Eternal Golden Braid.
- Lake, B., et al. (2017). Building machines that learn and think like people.

---

## Appendix A: ATLAS-Gen Source Code

*[See accompanying repository: crystalline-labs/atlas-gen]*

---

## Appendix B: Human Baseline Data Collection Protocol

*[IRB approval pending for human calibration study]*

---

**Acknowledgments**: Developed by Crystalline Labs for the ARC Prize community.
