#!/usr/bin/env python3
"""
ATLAS-Gen: Meta-Benchmark Generator for Abstraction Types

Generates problem instances across all 16 abstraction types defined in ATLAS.
ARC-compatible format for geometric types, extended formats for beyond-ARC types.

Author: Crystalline Labs
License: CC BY-NC-SA 4.0
"""

import json
import random
import numpy as np
from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from typing import List, Dict, Any, Optional, Tuple
from enum import Enum
import hashlib

# =============================================================================
# CORE DATA STRUCTURES
# =============================================================================

class AbstractionType(Enum):
    """The 16 abstraction types in ATLAS taxonomy."""
    A1_GEOMETRIC = "geometric"
    A2_CHROMATIC = "chromatic"
    A3_TOPOLOGICAL = "topological"
    A4_ARITHMETIC = "arithmetic"
    A5_SEQUENTIAL = "sequential"
    A6_TEMPORAL = "temporal"
    A7_CAUSAL = "causal"
    A8_HIERARCHICAL = "hierarchical"
    A9_ANALOGICAL = "analogical"
    A10_LINGUISTIC = "linguistic"
    A11_SOCIAL = "social"
    A12_COUNTERFACTUAL = "counterfactual"
    A13_RECURSIVE = "recursive"
    A14_PROBABILISTIC = "probabilistic"
    A15_COMPOSITIONAL = "compositional"
    A16_CROSSMODAL = "crossmodal"


@dataclass
class Problem:
    """A single benchmark problem instance."""
    id: str
    abstraction_type: AbstractionType
    difficulty: int  # 1-10
    input_data: Any
    output_data: Any
    ground_truth_abstraction: str
    metadata: Dict[str, Any] = field(default_factory=dict)
    
    def to_arc_format(self) -> Dict:
        """Export in ARC-compatible JSON format."""
        return {
            "id": self.id,
            "type": self.abstraction_type.value,
            "difficulty": self.difficulty,
            "input": self.input_data,
            "output": self.output_data,
            "metadata": {
                "abstraction": self.ground_truth_abstraction,
                **self.metadata
            }
        }
    
    def to_json(self) -> str:
        return json.dumps(self.to_arc_format(), indent=2)


@dataclass 
class ProblemSet:
    """Collection of problems for a benchmark."""
    name: str
    problems: List[Problem]
    type_distribution: Dict[AbstractionType, int] = field(default_factory=dict)
    
    def summary(self) -> Dict:
        """Generate coverage summary."""
        type_counts = {}
        for p in self.problems:
            t = p.abstraction_type.value
            type_counts[t] = type_counts.get(t, 0) + 1
        
        arc_types = ["geometric", "chromatic", "topological", "arithmetic", "compositional"]
        arc_coverage = sum(type_counts.get(t, 0) for t in arc_types)
        beyond_arc = len(self.problems) - arc_coverage
        
        return {
            "total_problems": len(self.problems),
            "type_distribution": type_counts,
            "arc_equivalent_count": arc_coverage,
            "beyond_arc_count": beyond_arc,
            "coverage_ratio": len(type_counts) / 16
        }


# =============================================================================
# ABSTRACT GENERATOR BASE
# =============================================================================

class AbstractionGenerator(ABC):
    """Base class for type-specific problem generators."""
    
    abstraction_type: AbstractionType
    
    @abstractmethod
    def generate(self, difficulty: int, seed: Optional[int] = None) -> Problem:
        """Generate a single problem instance."""
        pass
    
    @abstractmethod
    def validate(self, problem: Problem) -> bool:
        """Validate problem is solvable and well-formed."""
        pass
    
    def generate_batch(self, n: int, difficulty: int) -> List[Problem]:
        """Generate n problems at given difficulty."""
        problems = []
        for i in range(n):
            p = self.generate(difficulty, seed=random.randint(0, 2**32))
            if self.validate(p):
                problems.append(p)
        return problems
    
    def _make_id(self, difficulty: int, seed: int) -> str:
        """Generate unique problem ID."""
        data = f"{self.abstraction_type.value}_{difficulty}_{seed}"
        return hashlib.md5(data.encode()).hexdigest()[:12]


# =============================================================================
# TYPE A1: GEOMETRIC (ARC-COMPATIBLE)
# =============================================================================

class GeometricGenerator(AbstractionGenerator):
    """Generator for geometric abstraction problems (ARC-compatible)."""
    
    abstraction_type = AbstractionType.A1_GEOMETRIC
    
    SHAPES = ["rectangle", "square", "line_h", "line_v", "L_shape", "T_shape", "cross"]
    TRANSFORMS = ["rotate_90", "rotate_180", "rotate_270", "flip_h", "flip_v", "identity"]
    
    def generate(self, difficulty: int, seed: Optional[int] = None) -> Problem:
        if seed:
            random.seed(seed)
            np.random.seed(seed)
        
        # Scale grid size with difficulty
        grid_size = min(5 + difficulty, 30)
        num_objects = min(1 + difficulty // 2, 5)
        
        # Generate input grid
        input_grid = np.zeros((grid_size, grid_size), dtype=int)
        shapes_placed = []
        
        for _ in range(num_objects):
            shape = random.choice(self.SHAPES)
            color = random.randint(1, 9)
            pos = (random.randint(0, grid_size-3), random.randint(0, grid_size-3))
            self._place_shape(input_grid, shape, color, pos)
            shapes_placed.append((shape, color, pos))
        
        # Select transform
        transform = random.choice(self.TRANSFORMS)
        
        # Apply transform to get output
        output_grid = self._apply_transform(input_grid, transform)
        
        return Problem(
            id=self._make_id(difficulty, seed or 0),
            abstraction_type=self.abstraction_type,
            difficulty=difficulty,
            input_data=input_grid.tolist(),
            output_data=output_grid.tolist(),
            ground_truth_abstraction=f"transform={transform}",
            metadata={
                "shapes": shapes_placed,
                "transform": transform,
                "arc_compatible": True
            }
        )
    
    def _place_shape(self, grid: np.ndarray, shape: str, color: int, pos: Tuple[int, int]):
        """Place a shape on the grid."""
        r, c = pos
        if shape == "rectangle":
            grid[r:r+2, c:c+3] = color
        elif shape == "square":
            grid[r:r+2, c:c+2] = color
        elif shape == "line_h":
            grid[r, c:c+3] = color
        elif shape == "line_v":
            grid[r:r+3, c] = color
        elif shape == "L_shape":
            grid[r:r+3, c] = color
            grid[r+2, c:c+2] = color
        elif shape == "T_shape":
            grid[r, c:c+3] = color
            grid[r:r+2, c+1] = color
        elif shape == "cross":
            grid[r+1, c:c+3] = color
            grid[r:r+3, c+1] = color
    
    def _apply_transform(self, grid: np.ndarray, transform: str) -> np.ndarray:
        """Apply geometric transform."""
        if transform == "rotate_90":
            return np.rot90(grid, k=1)
        elif transform == "rotate_180":
            return np.rot90(grid, k=2)
        elif transform == "rotate_270":
            return np.rot90(grid, k=3)
        elif transform == "flip_h":
            return np.fliplr(grid)
        elif transform == "flip_v":
            return np.flipud(grid)
        else:  # identity
            return grid.copy()
    
    def validate(self, problem: Problem) -> bool:
        """Check problem is valid."""
        inp = np.array(problem.input_data)
        out = np.array(problem.output_data)
        # Must have some non-zero content
        return np.sum(inp) > 0 and np.sum(out) > 0


# =============================================================================
# TYPE A5: SEQUENTIAL (NOT IN ARC)
# =============================================================================

class SequentialGenerator(AbstractionGenerator):
    """Generator for sequential abstraction problems."""
    
    abstraction_type = AbstractionType.A5_SEQUENTIAL
    
    SEQUENCE_TYPES = [
        "arithmetic",      # a, a+d, a+2d, ...
        "geometric",       # a, a*r, a*r^2, ...
        "fibonacci_like",  # each term = f(previous terms)
        "modular",         # cyclic patterns
        "polynomial"       # n^2, n^3, etc.
    ]
    
    def generate(self, difficulty: int, seed: Optional[int] = None) -> Problem:
        if seed:
            random.seed(seed)
        
        seq_type = random.choice(self.SEQUENCE_TYPES)
        seq_length = 5 + difficulty
        
        if seq_type == "arithmetic":
            a = random.randint(1, 10)
            d = random.randint(1, 5) * random.choice([1, -1])
            sequence = [a + i*d for i in range(seq_length)]
            rule = f"a_n = {a} + {d}*n"
            
        elif seq_type == "geometric":
            a = random.randint(1, 5)
            r = random.randint(2, 3)
            sequence = [a * (r**i) for i in range(seq_length)]
            rule = f"a_n = {a} * {r}^n"
            
        elif seq_type == "fibonacci_like":
            a, b = random.randint(1, 5), random.randint(1, 5)
            sequence = [a, b]
            for _ in range(seq_length - 2):
                sequence.append(sequence[-1] + sequence[-2])
            rule = f"a_n = a_(n-1) + a_(n-2), a_0={a}, a_1={b}"
            
        elif seq_type == "modular":
            period = random.randint(2, 4)
            base = [random.randint(1, 9) for _ in range(period)]
            sequence = [base[i % period] for i in range(seq_length)]
            rule = f"a_n = pattern[n mod {period}], pattern={base}"
            
        else:  # polynomial
            power = random.randint(2, 3)
            sequence = [(i+1)**power for i in range(seq_length)]
            rule = f"a_n = n^{power}"
        
        # Input is all but last, output is last element
        return Problem(
            id=self._make_id(difficulty, seed or 0),
            abstraction_type=self.abstraction_type,
            difficulty=difficulty,
            input_data=sequence[:-1],
            output_data=sequence[-1],
            ground_truth_abstraction=rule,
            metadata={
                "sequence_type": seq_type,
                "full_sequence": sequence,
                "arc_compatible": False
            }
        )
    
    def validate(self, problem: Problem) -> bool:
        return len(problem.input_data) >= 3


# =============================================================================
# TYPE A7: CAUSAL (NOT IN ARC)
# =============================================================================

class CausalGenerator(AbstractionGenerator):
    """Generator for causal abstraction problems."""
    
    abstraction_type = AbstractionType.A7_CAUSAL
    
    def generate(self, difficulty: int, seed: Optional[int] = None) -> Problem:
        if seed:
            random.seed(seed)
            np.random.seed(seed)
        
        num_vars = 3 + difficulty // 2
        var_names = [f"X{i}" for i in range(num_vars)]
        
        # Generate random DAG (causal structure)
        adjacency = np.zeros((num_vars, num_vars))
        for i in range(num_vars):
            for j in range(i+1, num_vars):
                if random.random() < 0.4:  # Edge probability
                    adjacency[i, j] = random.uniform(0.5, 2.0) * random.choice([1, -1])
        
        # Generate observational data
        n_obs = 10 + difficulty * 2
        observations = self._simulate_dag(adjacency, n_obs)
        
        # Generate intervention query
        intervention_var = random.randint(0, num_vars - 2)
        intervention_value = random.uniform(-2, 2)
        target_var = random.randint(intervention_var + 1, num_vars - 1)
        
        # Compute intervention result
        result = self._compute_intervention(
            adjacency, intervention_var, intervention_value, target_var
        )
        
        return Problem(
            id=self._make_id(difficulty, seed or 0),
            abstraction_type=self.abstraction_type,
            difficulty=difficulty,
            input_data={
                "observations": observations.tolist(),
                "variable_names": var_names,
                "intervention": {
                    "variable": var_names[intervention_var],
                    "value": intervention_value
                },
                "query": f"E[{var_names[target_var]} | do({var_names[intervention_var]}={intervention_value:.2f})]"
            },
            output_data=round(result, 2),
            ground_truth_abstraction=f"DAG edges: {self._dag_to_edges(adjacency, var_names)}",
            metadata={
                "adjacency_matrix": adjacency.tolist(),
                "arc_compatible": False
            }
        )
    
    def _simulate_dag(self, adj: np.ndarray, n: int) -> np.ndarray:
        """Simulate observational data from DAG."""
        num_vars = adj.shape[0]
        data = np.zeros((n, num_vars))
        
        for v in range(num_vars):
            noise = np.random.randn(n) * 0.5
            parent_effect = data @ adj[:, v]
            data[:, v] = parent_effect + noise
        
        return data
    
    def _compute_intervention(self, adj: np.ndarray, int_var: int, 
                              int_val: float, target: int) -> float:
        """Compute E[target | do(int_var = int_val)]."""
        # Simplified: trace causal paths
        effect = 0.0
        for path in self._find_paths(adj, int_var, target):
            path_effect = int_val
            for i in range(len(path) - 1):
                path_effect *= adj[path[i], path[i+1]]
            effect += path_effect
        return effect
    
    def _find_paths(self, adj: np.ndarray, source: int, target: int) -> List[List[int]]:
        """Find all directed paths from source to target."""
        paths = []
        stack = [(source, [source])]
        while stack:
            node, path = stack.pop()
            if node == target:
                paths.append(path)
                continue
            for next_node in range(node + 1, adj.shape[0]):
                if adj[node, next_node] != 0:
                    stack.append((next_node, path + [next_node]))
        return paths
    
    def _dag_to_edges(self, adj: np.ndarray, names: List[str]) -> List[str]:
        """Convert adjacency matrix to edge list."""
        edges = []
        for i in range(adj.shape[0]):
            for j in range(adj.shape[1]):
                if adj[i, j] != 0:
                    edges.append(f"{names[i]}->{names[j]} ({adj[i,j]:.2f})")
        return edges
    
    def validate(self, problem: Problem) -> bool:
        return len(problem.input_data["observations"]) >= 5


# =============================================================================
# TYPE A9: ANALOGICAL (NOT IN ARC)
# =============================================================================

class AnalogicalGenerator(AbstractionGenerator):
    """Generator for analogical abstraction problems."""
    
    abstraction_type = AbstractionType.A9_ANALOGICAL
    
    # Relational structure templates
    STRUCTURES = [
        {
            "name": "hierarchy",
            "relations": ["contains", "part_of"],
            "template": lambda n: [(i, i+1, "contains") for i in range(n-1)]
        },
        {
            "name": "cycle",
            "relations": ["follows"],
            "template": lambda n: [(i, (i+1) % n, "follows") for i in range(n)]
        },
        {
            "name": "star",
            "relations": ["connected_to"],
            "template": lambda n: [(0, i, "connected_to") for i in range(1, n)]
        }
    ]
    
    # Domain vocabularies for surface variation
    DOMAINS = {
        "astronomy": ["sun", "planet", "moon", "asteroid", "comet", "star", "galaxy"],
        "biology": ["organism", "organ", "cell", "molecule", "atom", "tissue", "system"],
        "company": ["CEO", "VP", "director", "manager", "employee", "intern", "board"],
        "geography": ["country", "state", "city", "district", "street", "building", "room"],
    }
    
    def generate(self, difficulty: int, seed: Optional[int] = None) -> Problem:
        if seed:
            random.seed(seed)
        
        # Select structure and domains
        structure = random.choice(self.STRUCTURES)
        domain1, domain2 = random.sample(list(self.DOMAINS.keys()), 2)
        
        num_elements = 3 + difficulty // 2
        
        # Generate source domain instance
        source_elements = random.sample(self.DOMAINS[domain1], num_elements)
        source_relations = structure["template"](num_elements)
        source_relations = [(source_elements[i], source_elements[j], r) 
                          for i, j, r in source_relations]
        
        # Generate target domain instance (isomorphic)
        target_elements = random.sample(self.DOMAINS[domain2], num_elements)
        target_relations = [(target_elements[i], target_elements[j], r)
                          for i, j, r in structure["template"](num_elements)]
        
        # Create mapping
        mapping = {s: t for s, t in zip(source_elements, target_elements)}
        
        # Hide one element in target for the query
        query_idx = random.randint(0, num_elements - 1)
        query_source = source_elements[query_idx]
        query_answer = target_elements[query_idx]
        
        # Remove from visible target
        visible_target = [(s, o, r) for s, o, r in target_relations 
                         if s != query_answer and o != query_answer]
        
        return Problem(
            id=self._make_id(difficulty, seed or 0),
            abstraction_type=self.abstraction_type,
            difficulty=difficulty,
            input_data={
                "source_domain": {
                    "name": domain1,
                    "elements": source_elements,
                    "relations": source_relations
                },
                "target_domain": {
                    "name": domain2,
                    "visible_elements": [e for e in target_elements if e != query_answer],
                    "visible_relations": visible_target
                },
                "query": f"What in {domain2} corresponds to '{query_source}' in {domain1}?"
            },
            output_data=query_answer,
            ground_truth_abstraction=f"structure={structure['name']}, mapping={mapping}",
            metadata={
                "structure_type": structure["name"],
                "full_mapping": mapping,
                "arc_compatible": False
            }
        )
    
    def validate(self, problem: Problem) -> bool:
        return len(problem.input_data["source_domain"]["elements"]) >= 3


# =============================================================================
# TYPE A13: RECURSIVE (NOT IN ARC)
# =============================================================================

class RecursiveGenerator(AbstractionGenerator):
    """Generator for recursive/meta abstraction problems."""
    
    abstraction_type = AbstractionType.A13_RECURSIVE
    
    def generate(self, difficulty: int, seed: Optional[int] = None) -> Problem:
        if seed:
            random.seed(seed)
            np.random.seed(seed)
        
        recursion_depth = 2 + difficulty // 3
        grid_size = 3 + difficulty // 2
        
        # Generate base pattern
        base = np.random.randint(0, 3, size=(grid_size, grid_size))
        
        # Define meta-rule: how to apply pattern to itself
        meta_rules = [
            ("tile", lambda p: np.tile(p, (2, 2))[:p.shape[0]*2, :p.shape[1]*2]),
            ("overlay", lambda p: (p + np.rot90(p)) % 10),
            ("convolve", lambda p: self._simple_convolve(p)),
        ]
        
        rule_name, rule_fn = random.choice(meta_rules)
        
        # Generate sequence by recursive application
        sequence = [base]
        for _ in range(recursion_depth):
            next_state = rule_fn(sequence[-1])
            # Keep size manageable
            if next_state.shape[0] > 20:
                next_state = next_state[:20, :20]
            sequence.append(next_state)
        
        return Problem(
            id=self._make_id(difficulty, seed or 0),
            abstraction_type=self.abstraction_type,
            difficulty=difficulty,
            input_data=[s.tolist() for s in sequence[:-1]],
            output_data=sequence[-1].tolist(),
            ground_truth_abstraction=f"meta_rule={rule_name}, depth={recursion_depth}",
            metadata={
                "meta_rule": rule_name,
                "recursion_depth": recursion_depth,
                "arc_compatible": False
            }
        )
    
    def _simple_convolve(self, p: np.ndarray) -> np.ndarray:
        """Simple convolution-like operation."""
        result = np.zeros_like(p)
        for i in range(p.shape[0]):
            for j in range(p.shape[1]):
                neighbors = []
                for di in [-1, 0, 1]:
                    for dj in [-1, 0, 1]:
                        ni, nj = i + di, j + dj
                        if 0 <= ni < p.shape[0] and 0 <= nj < p.shape[1]:
                            neighbors.append(p[ni, nj])
                result[i, j] = sum(neighbors) % 10
        return result
    
    def validate(self, problem: Problem) -> bool:
        return len(problem.input_data) >= 2


# =============================================================================
# ATLAS-GEN: MASTER GENERATOR
# =============================================================================

class ATLASGenerator:
    """Master generator that orchestrates all type-specific generators."""
    
    def __init__(self):
        self.generators: Dict[AbstractionType, AbstractionGenerator] = {
            AbstractionType.A1_GEOMETRIC: GeometricGenerator(),
            AbstractionType.A5_SEQUENTIAL: SequentialGenerator(),
            AbstractionType.A7_CAUSAL: CausalGenerator(),
            AbstractionType.A9_ANALOGICAL: AnalogicalGenerator(),
            AbstractionType.A13_RECURSIVE: RecursiveGenerator(),
            # Add remaining generators as implemented
        }
        
        # Track which types are ARC-compatible
        self.arc_types = {
            AbstractionType.A1_GEOMETRIC,
            AbstractionType.A2_CHROMATIC,
            AbstractionType.A3_TOPOLOGICAL,
            AbstractionType.A4_ARITHMETIC,
            AbstractionType.A15_COMPOSITIONAL,
        }
    
    def generate_problem(self, 
                         abstraction_type: AbstractionType,
                         difficulty: int,
                         seed: Optional[int] = None) -> Optional[Problem]:
        """Generate a single problem of specified type."""
        if abstraction_type not in self.generators:
            raise NotImplementedError(f"Generator for {abstraction_type} not implemented")
        
        return self.generators[abstraction_type].generate(difficulty, seed)
    
    def generate_benchmark(self,
                          problems_per_type: int = 100,
                          difficulty_range: Tuple[int, int] = (1, 10),
                          types: Optional[List[AbstractionType]] = None) -> ProblemSet:
        """Generate complete benchmark suite."""
        if types is None:
            types = list(self.generators.keys())
        
        all_problems = []
        
        for atype in types:
            for difficulty in range(difficulty_range[0], difficulty_range[1] + 1):
                per_difficulty = problems_per_type // 10
                problems = self.generators[atype].generate_batch(per_difficulty, difficulty)
                all_problems.extend(problems)
        
        return ProblemSet(
            name="ATLAS-16",
            problems=all_problems
        )
    
    def generate_arc_comparison_set(self, n: int = 100) -> Tuple[ProblemSet, ProblemSet]:
        """Generate matched ARC-type and beyond-ARC problems for comparison."""
        arc_problems = []
        beyond_problems = []
        
        # ARC-compatible types
        for atype in self.arc_types:
            if atype in self.generators:
                probs = self.generators[atype].generate_batch(n // 5, difficulty=5)
                arc_problems.extend(probs)
        
        # Beyond-ARC types
        beyond_types = [t for t in self.generators.keys() if t not in self.arc_types]
        for atype in beyond_types:
            probs = self.generators[atype].generate_batch(n // len(beyond_types), difficulty=5)
            beyond_problems.extend(probs)
        
        return (
            ProblemSet(name="ARC-equivalent", problems=arc_problems),
            ProblemSet(name="Beyond-ARC", problems=beyond_problems)
        )
    
    def export_benchmark(self, problem_set: ProblemSet, output_path: str):
        """Export benchmark to JSON file."""
        data = {
            "name": problem_set.name,
            "summary": problem_set.summary(),
            "problems": [p.to_arc_format() for p in problem_set.problems]
        }
        
        with open(output_path, 'w') as f:
            json.dump(data, f, indent=2)
        
        return output_path


# =============================================================================
# SOLVER EVALUATION
# =============================================================================

@dataclass
class ATLASScore:
    """Comprehensive score across abstraction types."""
    type_scores: Dict[str, float]
    coverage: float  # Fraction of types with >50% accuracy
    arc_equivalent: float  # Performance on ARC-covered types
    beyond_arc: float  # Performance on non-ARC types
    overall: float
    
    def to_dict(self) -> Dict:
        return {
            "type_scores": self.type_scores,
            "coverage": self.coverage,
            "arc_equivalent": self.arc_equivalent,
            "beyond_arc": self.beyond_arc,
            "overall": self.overall
        }
    
    def __str__(self) -> str:
        lines = ["ATLAS-16 Evaluation Results", "=" * 40]
        for t, s in sorted(self.type_scores.items()):
            bar = "█" * int(s * 20) + "░" * (20 - int(s * 20))
            lines.append(f"{t:20} [{bar}] {s:.1%}")
        lines.append("-" * 40)
        lines.append(f"Coverage (>50%):     {self.coverage:.1%}")
        lines.append(f"ARC-equivalent:      {self.arc_equivalent:.1%}")
        lines.append(f"Beyond-ARC:          {self.beyond_arc:.1%}")
        lines.append(f"Overall:             {self.overall:.1%}")
        return "\n".join(lines)


class ATLASEvaluator:
    """Evaluate solvers against ATLAS benchmark."""
    
    ARC_TYPES = ["geometric", "chromatic", "topological", "arithmetic", "compositional"]
    
    def evaluate(self, solver, problem_set: ProblemSet) -> ATLASScore:
        """Run comprehensive evaluation."""
        type_correct = {}
        type_total = {}
        
        for problem in problem_set.problems:
            t = problem.abstraction_type.value
            type_total[t] = type_total.get(t, 0) + 1
            
            try:
                prediction = solver.solve(problem.input_data)
                if self._check_answer(prediction, problem.output_data):
                    type_correct[t] = type_correct.get(t, 0) + 1
            except Exception:
                pass  # Failed attempts count as wrong
        
        # Compute scores
        type_scores = {
            t: type_correct.get(t, 0) / type_total[t] 
            for t in type_total
        }
        
        coverage = sum(1 for s in type_scores.values() if s > 0.5) / len(type_scores)
        
        arc_scores = [type_scores.get(t, 0) for t in self.ARC_TYPES if t in type_scores]
        arc_equivalent = sum(arc_scores) / len(arc_scores) if arc_scores else 0
        
        beyond_types = [t for t in type_scores if t not in self.ARC_TYPES]
        beyond_scores = [type_scores[t] for t in beyond_types]
        beyond_arc = sum(beyond_scores) / len(beyond_scores) if beyond_scores else 0
        
        overall = sum(type_scores.values()) / len(type_scores)
        
        return ATLASScore(
            type_scores=type_scores,
            coverage=coverage,
            arc_equivalent=arc_equivalent,
            beyond_arc=beyond_arc,
            overall=overall
        )
    
    def _check_answer(self, prediction: Any, expected: Any) -> bool:
        """Check if prediction matches expected output."""
        if isinstance(expected, (list, np.ndarray)):
            return np.array_equal(np.array(prediction), np.array(expected))
        return prediction == expected


# =============================================================================
# CLI INTERFACE
# =============================================================================

def main():
    """Generate benchmark and demonstrate usage."""
    import argparse
    
    parser = argparse.ArgumentParser(description="ATLAS Meta-Benchmark Generator")
    parser.add_argument("--generate", type=int, help="Generate N problems per type")
    parser.add_argument("--output", type=str, default="atlas_benchmark.json")
    parser.add_argument("--types", type=str, nargs="+", help="Specific types to generate")
    
    args = parser.parse_args()
    
    generator = ATLASGenerator()
    
    if args.generate:
        print(f"Generating {args.generate} problems per type...")
        benchmark = generator.generate_benchmark(problems_per_type=args.generate)
        generator.export_benchmark(benchmark, args.output)
        print(f"Exported to {args.output}")
        print("\nSummary:")
        print(json.dumps(benchmark.summary(), indent=2))
    else:
        # Demo mode
        print("ATLAS-Gen Demo")
        print("=" * 40)
        
        for atype in generator.generators:
            problem = generator.generate_problem(atype, difficulty=5, seed=42)
            print(f"\n{atype.value.upper()}:")
            print(f"  Input: {str(problem.input_data)[:60]}...")
            print(f"  Output: {str(problem.output_data)[:60]}...")
            print(f"  Abstraction: {problem.ground_truth_abstraction}")


if __name__ == "__main__":
    main()
