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
# TYPE A2: CHROMATIC (ARC-COMPATIBLE)
# =============================================================================

class ChromaticGenerator(AbstractionGenerator):
    """Generator for chromatic/color abstraction problems (ARC-compatible)."""

    abstraction_type = AbstractionType.A2_CHROMATIC

    COLOR_RULES = [
        "swap",           # Swap two colors
        "shift",          # Shift all colors by n
        "complement",     # Map to complementary colors
        "threshold",      # Colors above/below threshold
        "majority",       # Replace with majority neighbor color
        "boundary",       # Color boundaries between regions
    ]

    def generate(self, difficulty: int, seed: Optional[int] = None) -> Problem:
        if seed:
            random.seed(seed)
            np.random.seed(seed)

        grid_size = min(5 + difficulty, 20)
        num_colors = min(3 + difficulty // 2, 9)

        # Generate input grid with color regions
        input_grid = np.zeros((grid_size, grid_size), dtype=int)

        # Create random color regions
        for _ in range(num_colors * 2):
            color = random.randint(1, num_colors)
            x, y = random.randint(0, grid_size-1), random.randint(0, grid_size-1)
            size = random.randint(2, 4)
            input_grid[max(0,x-size):min(grid_size,x+size),
                      max(0,y-size):min(grid_size,y+size)] = color

        # Select and apply color rule
        rule = random.choice(self.COLOR_RULES)
        output_grid = self._apply_color_rule(input_grid.copy(), rule, num_colors)

        return Problem(
            id=self._make_id(difficulty, seed or 0),
            abstraction_type=self.abstraction_type,
            difficulty=difficulty,
            input_data=input_grid.tolist(),
            output_data=output_grid.tolist(),
            ground_truth_abstraction=f"color_rule={rule}",
            metadata={
                "rule": rule,
                "num_colors": num_colors,
                "arc_compatible": True
            }
        )

    def _apply_color_rule(self, grid: np.ndarray, rule: str, num_colors: int) -> np.ndarray:
        if rule == "swap":
            c1, c2 = random.sample(range(1, num_colors + 1), 2)
            result = grid.copy()
            result[grid == c1] = c2
            result[grid == c2] = c1
            return result
        elif rule == "shift":
            shift = random.randint(1, num_colors - 1)
            result = np.where(grid > 0, ((grid - 1 + shift) % num_colors) + 1, 0)
            return result
        elif rule == "complement":
            return np.where(grid > 0, num_colors + 1 - grid, 0)
        elif rule == "threshold":
            thresh = num_colors // 2
            return np.where(grid > thresh, grid, 0)
        elif rule == "majority":
            result = grid.copy()
            for i in range(grid.shape[0]):
                for j in range(grid.shape[1]):
                    neighbors = []
                    for di, dj in [(-1,0), (1,0), (0,-1), (0,1)]:
                        ni, nj = i + di, j + dj
                        if 0 <= ni < grid.shape[0] and 0 <= nj < grid.shape[1]:
                            neighbors.append(grid[ni, nj])
                    if neighbors:
                        result[i, j] = max(set(neighbors), key=neighbors.count)
            return result
        else:  # boundary
            result = np.zeros_like(grid)
            for i in range(grid.shape[0]):
                for j in range(grid.shape[1]):
                    if grid[i, j] > 0:
                        is_boundary = False
                        for di, dj in [(-1,0), (1,0), (0,-1), (0,1)]:
                            ni, nj = i + di, j + dj
                            if 0 <= ni < grid.shape[0] and 0 <= nj < grid.shape[1]:
                                if grid[ni, nj] != grid[i, j]:
                                    is_boundary = True
                        if is_boundary:
                            result[i, j] = grid[i, j]
            return result

    def validate(self, problem: Problem) -> bool:
        inp = np.array(problem.input_data)
        out = np.array(problem.output_data)
        return np.sum(inp) > 0 and not np.array_equal(inp, out)


# =============================================================================
# TYPE A3: TOPOLOGICAL (ARC-COMPATIBLE)
# =============================================================================

class TopologicalGenerator(AbstractionGenerator):
    """Generator for topological abstraction problems (ARC-compatible)."""

    abstraction_type = AbstractionType.A3_TOPOLOGICAL

    TOPO_RULES = [
        "fill_enclosed",     # Fill enclosed regions
        "count_components",  # Output = number of connected components
        "largest_component", # Keep only largest connected component
        "holes",            # Identify and mark holes
        "skeleton",         # Reduce to skeleton/medial axis
        "flood_fill",       # Flood fill from corners
    ]

    def generate(self, difficulty: int, seed: Optional[int] = None) -> Problem:
        if seed:
            random.seed(seed)
            np.random.seed(seed)

        grid_size = min(8 + difficulty, 25)

        # Generate input with topological features
        input_grid = np.zeros((grid_size, grid_size), dtype=int)

        # Create random closed shapes
        num_shapes = 1 + difficulty // 3
        for _ in range(num_shapes):
            self._draw_closed_shape(input_grid, grid_size)

        rule = random.choice(self.TOPO_RULES)
        output_grid = self._apply_topo_rule(input_grid.copy(), rule)

        return Problem(
            id=self._make_id(difficulty, seed or 0),
            abstraction_type=self.abstraction_type,
            difficulty=difficulty,
            input_data=input_grid.tolist(),
            output_data=output_grid.tolist(),
            ground_truth_abstraction=f"topo_rule={rule}",
            metadata={
                "rule": rule,
                "arc_compatible": True
            }
        )

    def _draw_closed_shape(self, grid: np.ndarray, size: int):
        """Draw a closed rectangular shape."""
        color = random.randint(1, 5)
        x1, y1 = random.randint(1, size//2), random.randint(1, size//2)
        x2, y2 = x1 + random.randint(3, size//2), y1 + random.randint(3, size//2)
        x2, y2 = min(x2, size-1), min(y2, size-1)

        # Draw rectangle border
        grid[x1, y1:y2+1] = color
        grid[x2, y1:y2+1] = color
        grid[x1:x2+1, y1] = color
        grid[x1:x2+1, y2] = color

    def _apply_topo_rule(self, grid: np.ndarray, rule: str) -> np.ndarray:
        if rule == "fill_enclosed":
            result = grid.copy()
            # Simple flood fill from edges to find non-enclosed
            mask = np.zeros_like(grid, dtype=bool)
            stack = [(0, 0)]
            while stack:
                x, y = stack.pop()
                if x < 0 or x >= grid.shape[0] or y < 0 or y >= grid.shape[1]:
                    continue
                if mask[x, y] or grid[x, y] > 0:
                    continue
                mask[x, y] = True
                for dx, dy in [(-1,0), (1,0), (0,-1), (0,1)]:
                    stack.append((x+dx, y+dy))
            # Fill enclosed areas
            result[~mask & (grid == 0)] = 9
            return result

        elif rule == "count_components":
            # Count connected components and output as single number in grid
            visited = np.zeros_like(grid, dtype=bool)
            count = 0
            for i in range(grid.shape[0]):
                for j in range(grid.shape[1]):
                    if grid[i, j] > 0 and not visited[i, j]:
                        count += 1
                        stack = [(i, j)]
                        while stack:
                            x, y = stack.pop()
                            if x < 0 or x >= grid.shape[0] or y < 0 or y >= grid.shape[1]:
                                continue
                            if visited[x, y] or grid[x, y] == 0:
                                continue
                            visited[x, y] = True
                            for dx, dy in [(-1,0), (1,0), (0,-1), (0,1)]:
                                stack.append((x+dx, y+dy))
            result = np.zeros((3, 3), dtype=int)
            result[1, 1] = min(count, 9)
            return result

        elif rule == "largest_component":
            visited = np.zeros_like(grid, dtype=bool)
            components = []
            for i in range(grid.shape[0]):
                for j in range(grid.shape[1]):
                    if grid[i, j] > 0 and not visited[i, j]:
                        component = []
                        stack = [(i, j)]
                        while stack:
                            x, y = stack.pop()
                            if x < 0 or x >= grid.shape[0] or y < 0 or y >= grid.shape[1]:
                                continue
                            if visited[x, y] or grid[x, y] == 0:
                                continue
                            visited[x, y] = True
                            component.append((x, y))
                            for dx, dy in [(-1,0), (1,0), (0,-1), (0,1)]:
                                stack.append((x+dx, y+dy))
                        components.append(component)

            result = np.zeros_like(grid)
            if components:
                largest = max(components, key=len)
                for x, y in largest:
                    result[x, y] = grid[x, y]
            return result

        elif rule == "holes":
            # Mark enclosed empty regions
            result = grid.copy()
            mask = np.zeros_like(grid, dtype=bool)
            # Flood from edges
            for i in range(grid.shape[0]):
                for j in [0, grid.shape[1]-1]:
                    if grid[i, j] == 0:
                        stack = [(i, j)]
                        while stack:
                            x, y = stack.pop()
                            if x < 0 or x >= grid.shape[0] or y < 0 or y >= grid.shape[1]:
                                continue
                            if mask[x, y] or grid[x, y] > 0:
                                continue
                            mask[x, y] = True
                            for dx, dy in [(-1,0), (1,0), (0,-1), (0,1)]:
                                stack.append((x+dx, y+dy))
            for j in range(grid.shape[1]):
                for i in [0, grid.shape[0]-1]:
                    if grid[i, j] == 0 and not mask[i, j]:
                        stack = [(i, j)]
                        while stack:
                            x, y = stack.pop()
                            if x < 0 or x >= grid.shape[0] or y < 0 or y >= grid.shape[1]:
                                continue
                            if mask[x, y] or grid[x, y] > 0:
                                continue
                            mask[x, y] = True
                            for dx, dy in [(-1,0), (1,0), (0,-1), (0,1)]:
                                stack.append((x+dx, y+dy))
            # Mark holes
            result[~mask & (grid == 0)] = 8
            return result

        elif rule == "skeleton":
            # Simple skeleton: keep only cells with exactly 2 neighbors
            result = np.zeros_like(grid)
            for i in range(grid.shape[0]):
                for j in range(grid.shape[1]):
                    if grid[i, j] > 0:
                        neighbors = 0
                        for di, dj in [(-1,0), (1,0), (0,-1), (0,1)]:
                            ni, nj = i + di, j + dj
                            if 0 <= ni < grid.shape[0] and 0 <= nj < grid.shape[1]:
                                if grid[ni, nj] > 0:
                                    neighbors += 1
                        if neighbors <= 2:
                            result[i, j] = grid[i, j]
            return result

        else:  # flood_fill
            result = grid.copy()
            # Flood fill from corner with new color
            stack = [(0, 0)]
            fill_color = 7
            while stack:
                x, y = stack.pop()
                if x < 0 or x >= grid.shape[0] or y < 0 or y >= grid.shape[1]:
                    continue
                if result[x, y] != 0:
                    continue
                result[x, y] = fill_color
                for dx, dy in [(-1,0), (1,0), (0,-1), (0,1)]:
                    stack.append((x+dx, y+dy))
            return result

    def validate(self, problem: Problem) -> bool:
        inp = np.array(problem.input_data)
        return np.sum(inp) > 0


# =============================================================================
# TYPE A4: ARITHMETIC (ARC-COMPATIBLE)
# =============================================================================

class ArithmeticGenerator(AbstractionGenerator):
    """Generator for arithmetic abstraction problems (ARC-compatible)."""

    abstraction_type = AbstractionType.A4_ARITHMETIC

    def generate(self, difficulty: int, seed: Optional[int] = None) -> Problem:
        if seed:
            random.seed(seed)
            np.random.seed(seed)

        rule_type = random.choice(["count", "sum", "multiply", "mod", "compare"])
        grid_size = min(5 + difficulty, 15)

        input_grid = np.zeros((grid_size, grid_size), dtype=int)

        # Place objects to count/sum
        num_objects = random.randint(2, min(5, 1 + difficulty))
        object_color = random.randint(1, 5)

        for _ in range(num_objects):
            x, y = random.randint(0, grid_size-2), random.randint(0, grid_size-2)
            size = random.randint(1, 3)
            input_grid[x:x+size, y:y+size] = object_color

        if rule_type == "count":
            # Count distinct connected objects
            visited = np.zeros_like(input_grid, dtype=bool)
            count = 0
            for i in range(grid_size):
                for j in range(grid_size):
                    if input_grid[i, j] > 0 and not visited[i, j]:
                        count += 1
                        stack = [(i, j)]
                        while stack:
                            x, y = stack.pop()
                            if 0 <= x < grid_size and 0 <= y < grid_size:
                                if not visited[x, y] and input_grid[x, y] > 0:
                                    visited[x, y] = True
                                    for dx, dy in [(-1,0), (1,0), (0,-1), (0,1)]:
                                        stack.append((x+dx, y+dy))
            output_grid = np.zeros((1, 1), dtype=int)
            output_grid[0, 0] = min(count, 9)
            rule = f"count_objects={count}"

        elif rule_type == "sum":
            total = int(np.sum(input_grid))
            output_grid = np.zeros((1, 1), dtype=int)
            output_grid[0, 0] = total % 10
            rule = f"sum_mod_10={total % 10}"

        elif rule_type == "multiply":
            # Multiply grid dimensions
            rows = np.any(input_grid > 0, axis=1).sum()
            cols = np.any(input_grid > 0, axis=0).sum()
            output_grid = np.zeros((1, 1), dtype=int)
            output_grid[0, 0] = (rows * cols) % 10
            rule = f"rows*cols_mod_10={(rows * cols) % 10}"

        elif rule_type == "mod":
            # Apply modular arithmetic
            mod_val = random.randint(2, 4)
            output_grid = input_grid % mod_val
            rule = f"mod_{mod_val}"

        else:  # compare
            # Compare regions
            mid = grid_size // 2
            left_sum = np.sum(input_grid[:, :mid])
            right_sum = np.sum(input_grid[:, mid:])
            output_grid = np.zeros((1, 1), dtype=int)
            output_grid[0, 0] = 1 if left_sum > right_sum else (2 if right_sum > left_sum else 0)
            rule = f"compare_halves"

        return Problem(
            id=self._make_id(difficulty, seed or 0),
            abstraction_type=self.abstraction_type,
            difficulty=difficulty,
            input_data=input_grid.tolist(),
            output_data=output_grid.tolist(),
            ground_truth_abstraction=rule,
            metadata={
                "rule_type": rule_type,
                "arc_compatible": True
            }
        )

    def validate(self, problem: Problem) -> bool:
        inp = np.array(problem.input_data)
        return np.sum(inp) > 0


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
# TYPE A6: TEMPORAL (NOT IN ARC)
# =============================================================================

class TemporalGenerator(AbstractionGenerator):
    """Generator for temporal/dynamics abstraction problems."""

    abstraction_type = AbstractionType.A6_TEMPORAL

    DYNAMICS = [
        "linear",       # x(t+1) = x(t) + v
        "exponential",  # x(t+1) = r * x(t)
        "oscillating",  # x(t) = A*sin(ωt)
        "decay",        # x(t+1) = x(t) * decay
        "game_of_life", # Conway's rules
    ]

    def generate(self, difficulty: int, seed: Optional[int] = None) -> Problem:
        if seed:
            random.seed(seed)
            np.random.seed(seed)

        dynamics = random.choice(self.DYNAMICS)
        num_steps = 3 + difficulty // 2
        grid_size = min(5 + difficulty // 2, 15)

        if dynamics == "game_of_life":
            # Generate game of life sequence
            state = np.random.randint(0, 2, size=(grid_size, grid_size))
            trajectory = [state.copy()]
            for _ in range(num_steps):
                state = self._game_of_life_step(state)
                trajectory.append(state.copy())
        else:
            # Generate 1D trajectory as a time series
            trajectory = self._generate_trajectory(dynamics, num_steps + 1, difficulty)

        return Problem(
            id=self._make_id(difficulty, seed or 0),
            abstraction_type=self.abstraction_type,
            difficulty=difficulty,
            input_data=[t.tolist() if isinstance(t, np.ndarray) else t for t in trajectory[:-1]],
            output_data=trajectory[-1].tolist() if isinstance(trajectory[-1], np.ndarray) else trajectory[-1],
            ground_truth_abstraction=f"dynamics={dynamics}",
            metadata={
                "dynamics_type": dynamics,
                "num_steps": num_steps,
                "arc_compatible": False
            }
        )

    def _game_of_life_step(self, grid: np.ndarray) -> np.ndarray:
        """Apply Conway's Game of Life rules."""
        result = np.zeros_like(grid)
        for i in range(grid.shape[0]):
            for j in range(grid.shape[1]):
                neighbors = 0
                for di in [-1, 0, 1]:
                    for dj in [-1, 0, 1]:
                        if di == 0 and dj == 0:
                            continue
                        ni, nj = (i + di) % grid.shape[0], (j + dj) % grid.shape[1]
                        neighbors += grid[ni, nj]
                if grid[i, j] == 1:
                    result[i, j] = 1 if neighbors in [2, 3] else 0
                else:
                    result[i, j] = 1 if neighbors == 3 else 0
        return result

    def _generate_trajectory(self, dynamics: str, steps: int, difficulty: int) -> List:
        if dynamics == "linear":
            x0 = random.randint(1, 5)
            v = random.randint(1, 3)
            return [x0 + v * t for t in range(steps)]
        elif dynamics == "exponential":
            x0 = random.randint(1, 3)
            r = random.choice([1.5, 2.0])
            return [int(x0 * (r ** t)) for t in range(steps)]
        elif dynamics == "oscillating":
            A = random.randint(3, 5)
            period = random.randint(3, 5)
            return [int(A * np.sin(2 * np.pi * t / period) + A) for t in range(steps)]
        else:  # decay
            x0 = random.randint(50, 100)
            decay = 0.8
            result = [x0]
            for _ in range(steps - 1):
                result.append(int(result[-1] * decay))
            return result

    def validate(self, problem: Problem) -> bool:
        return len(problem.input_data) >= 2


# =============================================================================
# TYPE A8: HIERARCHICAL (NOT IN ARC)
# =============================================================================

class HierarchicalGenerator(AbstractionGenerator):
    """Generator for hierarchical/part-whole abstraction problems."""

    abstraction_type = AbstractionType.A8_HIERARCHICAL

    def generate(self, difficulty: int, seed: Optional[int] = None) -> Problem:
        if seed:
            random.seed(seed)

        depth = 2 + difficulty // 3
        branching = random.randint(2, 3)

        # Generate tree structure
        tree = self._generate_tree(depth, branching)

        # Query types
        query_type = random.choice(["count_leaves", "depth_of", "parent_of", "subtree_size"])

        if query_type == "count_leaves":
            answer = self._count_leaves(tree)
            query = "Count all leaf nodes"
        elif query_type == "depth_of":
            node = self._random_node(tree)
            answer = self._depth_of(tree, node)
            query = f"Depth of node {node}"
        elif query_type == "parent_of":
            node = self._random_node(tree, exclude_root=True)
            answer = self._parent_of(tree, node)
            query = f"Parent of node {node}"
        else:  # subtree_size
            node = self._random_node(tree)
            answer = self._subtree_size(tree, node)
            query = f"Size of subtree rooted at {node}"

        return Problem(
            id=self._make_id(difficulty, seed or 0),
            abstraction_type=self.abstraction_type,
            difficulty=difficulty,
            input_data={
                "tree": tree,
                "query": query
            },
            output_data=answer,
            ground_truth_abstraction=f"hierarchy_query={query_type}",
            metadata={
                "depth": depth,
                "query_type": query_type,
                "arc_compatible": False
            }
        )

    def _generate_tree(self, depth: int, branching: int) -> Dict:
        """Generate a tree as nested dict."""
        counter = [0]
        def build(d):
            node_id = counter[0]
            counter[0] += 1
            if d == 0:
                return {"id": node_id, "children": []}
            children = [build(d - 1) for _ in range(random.randint(1, branching))]
            return {"id": node_id, "children": children}
        return build(depth)

    def _count_leaves(self, tree: Dict) -> int:
        if not tree["children"]:
            return 1
        return sum(self._count_leaves(c) for c in tree["children"])

    def _depth_of(self, tree: Dict, target: int, current_depth: int = 0) -> int:
        if tree["id"] == target:
            return current_depth
        for child in tree["children"]:
            result = self._depth_of(child, target, current_depth + 1)
            if result >= 0:
                return result
        return -1

    def _parent_of(self, tree: Dict, target: int, parent: int = -1) -> int:
        if tree["id"] == target:
            return parent
        for child in tree["children"]:
            result = self._parent_of(child, target, tree["id"])
            if result >= 0:
                return result
        return -1

    def _subtree_size(self, tree: Dict) -> int:
        return 1 + sum(self._subtree_size(c) for c in tree["children"])

    def _random_node(self, tree: Dict, exclude_root: bool = False) -> int:
        nodes = []
        def collect(t):
            nodes.append(t["id"])
            for c in t["children"]:
                collect(c)
        collect(tree)
        if exclude_root and len(nodes) > 1:
            nodes = nodes[1:]
        return random.choice(nodes)

    def validate(self, problem: Problem) -> bool:
        return "tree" in problem.input_data


# =============================================================================
# TYPE A10: LINGUISTIC (NOT IN ARC)
# =============================================================================

class LinguisticGenerator(AbstractionGenerator):
    """Generator for linguistic abstraction problems."""

    abstraction_type = AbstractionType.A10_LINGUISTIC

    # Simple grammar rules
    TEMPLATES = [
        ("The {adj} {noun} {verb}s.", {"adj": ["big", "small", "red", "fast"],
                                        "noun": ["cat", "dog", "bird", "car"],
                                        "verb": ["run", "jump", "fly", "move"]}),
        ("{name} {verb}s to the {place}.", {"name": ["Alice", "Bob", "Carol"],
                                             "verb": ["walk", "run", "drive"],
                                             "place": ["store", "park", "school"]}),
    ]

    TRANSFORMS = [
        "negate",       # Add "not"
        "question",     # Convert to question
        "passive",      # Active to passive
        "plural",       # Singular to plural
    ]

    def generate(self, difficulty: int, seed: Optional[int] = None) -> Problem:
        if seed:
            random.seed(seed)

        template, slots = random.choice(self.TEMPLATES)
        filled = {k: random.choice(v) for k, v in slots.items()}
        sentence = template.format(**filled)

        transform = random.choice(self.TRANSFORMS)
        transformed = self._apply_transform(sentence, transform, filled)

        return Problem(
            id=self._make_id(difficulty, seed or 0),
            abstraction_type=self.abstraction_type,
            difficulty=difficulty,
            input_data={
                "sentence": sentence,
                "transform": transform
            },
            output_data=transformed,
            ground_truth_abstraction=f"linguistic_transform={transform}",
            metadata={
                "template": template,
                "slots": filled,
                "arc_compatible": False
            }
        )

    def _apply_transform(self, sentence: str, transform: str, slots: Dict) -> str:
        if transform == "negate":
            # Simple negation
            return sentence.replace("s.", "s not.").replace("s to", "s not to")
        elif transform == "question":
            # Simple question form
            return "Does " + sentence[0].lower() + sentence[1:].replace("s.", "?")
        elif transform == "passive":
            # Very simplified passive
            return "It is done."  # Placeholder for complex transform
        else:  # plural
            return sentence.replace("The ", "The two ").replace("s.", ".")

    def validate(self, problem: Problem) -> bool:
        return len(problem.input_data.get("sentence", "")) > 0


# =============================================================================
# TYPE A11: SOCIAL/THEORY OF MIND (NOT IN ARC)
# =============================================================================

class SocialGenerator(AbstractionGenerator):
    """Generator for social/theory of mind abstraction problems."""

    abstraction_type = AbstractionType.A11_SOCIAL

    SCENARIOS = [
        {
            "setup": "{agent1} puts the {object} in the {location1}. {agent1} leaves. {agent2} moves the {object} to the {location2}. {agent1} returns.",
            "question": "Where will {agent1} look for the {object}?",
            "answer_key": "location1",  # False belief
            "type": "false_belief"
        },
        {
            "setup": "{agent1} wants {object}. {agent2} has {object}. {agent2} offers {object} to {agent1}.",
            "question": "Will {agent1} accept?",
            "answer_key": "yes",
            "type": "desire_inference"
        },
        {
            "setup": "{agent1} tells {agent2} that the {object} is in {location1}. But {agent1} knows it's actually in {location2}.",
            "question": "Is {agent1} lying?",
            "answer_key": "yes",
            "type": "deception_detection"
        },
    ]

    AGENTS = ["Alice", "Bob", "Carol", "Dave"]
    OBJECTS = ["ball", "key", "book", "toy"]
    LOCATIONS = ["box", "basket", "drawer", "shelf"]

    def generate(self, difficulty: int, seed: Optional[int] = None) -> Problem:
        if seed:
            random.seed(seed)

        scenario = random.choice(self.SCENARIOS)
        agents = random.sample(self.AGENTS, 2)
        obj = random.choice(self.OBJECTS)
        locs = random.sample(self.LOCATIONS, 2)

        filled = {
            "agent1": agents[0],
            "agent2": agents[1],
            "object": obj,
            "location1": locs[0],
            "location2": locs[1]
        }

        setup = scenario["setup"].format(**filled)
        question = scenario["question"].format(**filled)

        if scenario["answer_key"] in filled:
            answer = filled[scenario["answer_key"]]
        else:
            answer = scenario["answer_key"]

        return Problem(
            id=self._make_id(difficulty, seed or 0),
            abstraction_type=self.abstraction_type,
            difficulty=difficulty,
            input_data={
                "scenario": setup,
                "question": question
            },
            output_data=answer,
            ground_truth_abstraction=f"tom_type={scenario['type']}",
            metadata={
                "scenario_type": scenario["type"],
                "agents": agents,
                "arc_compatible": False
            }
        )

    def validate(self, problem: Problem) -> bool:
        return len(problem.input_data.get("scenario", "")) > 0


# =============================================================================
# TYPE A12: COUNTERFACTUAL (NOT IN ARC)
# =============================================================================

class CounterfactualGenerator(AbstractionGenerator):
    """Generator for counterfactual reasoning problems."""

    abstraction_type = AbstractionType.A12_COUNTERFACTUAL

    def generate(self, difficulty: int, seed: Optional[int] = None) -> Problem:
        if seed:
            random.seed(seed)
            np.random.seed(seed)

        # Generate a simple causal chain
        num_vars = 3 + difficulty // 2
        chain = list(range(num_vars))

        # Actual values
        actual_values = {i: random.randint(0, 5) for i in chain}

        # Causal functions: each var = prev_var + noise
        def compute_downstream(values, start_idx):
            result = values.copy()
            for i in range(start_idx + 1, num_vars):
                result[i] = result[i-1] + random.randint(0, 2)
            return result

        # Intervention
        intervention_var = random.randint(0, num_vars - 2)
        intervention_value = random.randint(0, 5)
        target_var = num_vars - 1

        # Counterfactual computation
        cf_values = actual_values.copy()
        cf_values[intervention_var] = intervention_value
        cf_values = compute_downstream(cf_values, intervention_var)

        return Problem(
            id=self._make_id(difficulty, seed or 0),
            abstraction_type=self.abstraction_type,
            difficulty=difficulty,
            input_data={
                "actual_values": actual_values,
                "causal_chain": chain,
                "intervention": {
                    "variable": intervention_var,
                    "value": intervention_value
                },
                "query": f"What would variable {target_var} be if variable {intervention_var} had been {intervention_value}?"
            },
            output_data=cf_values[target_var],
            ground_truth_abstraction=f"counterfactual_chain",
            metadata={
                "intervention_var": intervention_var,
                "target_var": target_var,
                "arc_compatible": False
            }
        )

    def validate(self, problem: Problem) -> bool:
        return "actual_values" in problem.input_data


# =============================================================================
# TYPE A14: PROBABILISTIC (NOT IN ARC)
# =============================================================================

class ProbabilisticGenerator(AbstractionGenerator):
    """Generator for probabilistic abstraction problems."""

    abstraction_type = AbstractionType.A14_PROBABILISTIC

    def generate(self, difficulty: int, seed: Optional[int] = None) -> Problem:
        if seed:
            random.seed(seed)
            np.random.seed(seed)

        problem_type = random.choice(["bayes", "expected_value", "distribution_id", "conditional"])

        if problem_type == "bayes":
            # Simple Bayes problem
            prior = round(random.uniform(0.1, 0.5), 2)
            likelihood = round(random.uniform(0.6, 0.95), 2)
            false_positive = round(random.uniform(0.05, 0.2), 2)

            # P(A|B) = P(B|A)P(A) / (P(B|A)P(A) + P(B|~A)P(~A))
            posterior = (likelihood * prior) / (likelihood * prior + false_positive * (1 - prior))

            input_data = {
                "type": "bayes",
                "prior_probability": prior,
                "true_positive_rate": likelihood,
                "false_positive_rate": false_positive,
                "query": "Given positive test, what is probability of condition?"
            }
            answer = round(posterior, 2)
            rule = "bayes_theorem"

        elif problem_type == "expected_value":
            # Expected value calculation
            outcomes = [random.randint(1, 10) for _ in range(3 + difficulty // 2)]
            probs = [random.random() for _ in outcomes]
            probs = [p / sum(probs) for p in probs]  # Normalize
            probs = [round(p, 2) for p in probs]
            # Adjust last prob to sum to 1
            probs[-1] = round(1 - sum(probs[:-1]), 2)

            ev = sum(o * p for o, p in zip(outcomes, probs))

            input_data = {
                "type": "expected_value",
                "outcomes": outcomes,
                "probabilities": probs,
                "query": "What is the expected value?"
            }
            answer = round(ev, 2)
            rule = "expected_value"

        elif problem_type == "distribution_id":
            # Identify distribution from samples
            dist_type = random.choice(["uniform", "normal", "bimodal"])
            if dist_type == "uniform":
                samples = [random.randint(1, 10) for _ in range(20)]
            elif dist_type == "normal":
                samples = [int(np.clip(np.random.normal(5, 1), 1, 10)) for _ in range(20)]
            else:  # bimodal
                samples = [random.choice([2, 8]) + random.randint(-1, 1) for _ in range(20)]

            input_data = {
                "type": "distribution_id",
                "samples": samples,
                "query": "What type of distribution?"
            }
            answer = dist_type
            rule = "distribution_identification"

        else:  # conditional
            # P(A and B) given P(A), P(B|A)
            p_a = round(random.uniform(0.2, 0.8), 2)
            p_b_given_a = round(random.uniform(0.3, 0.9), 2)
            p_a_and_b = p_a * p_b_given_a

            input_data = {
                "type": "conditional",
                "P_A": p_a,
                "P_B_given_A": p_b_given_a,
                "query": "What is P(A and B)?"
            }
            answer = round(p_a_and_b, 2)
            rule = "conditional_probability"

        return Problem(
            id=self._make_id(difficulty, seed or 0),
            abstraction_type=self.abstraction_type,
            difficulty=difficulty,
            input_data=input_data,
            output_data=answer,
            ground_truth_abstraction=rule,
            metadata={
                "problem_type": problem_type,
                "arc_compatible": False
            }
        )

    def validate(self, problem: Problem) -> bool:
        return "type" in problem.input_data


# =============================================================================
# TYPE A15: COMPOSITIONAL (ARC-COMPATIBLE)
# =============================================================================

class CompositionalGenerator(AbstractionGenerator):
    """Generator for compositional abstraction problems (ARC-compatible)."""

    abstraction_type = AbstractionType.A15_COMPOSITIONAL

    # Primitive operations that can be composed
    PRIMITIVES = ["rotate_90", "flip_h", "flip_v", "scale_2x", "invert_colors", "shift_right"]

    def generate(self, difficulty: int, seed: Optional[int] = None) -> Problem:
        if seed:
            random.seed(seed)
            np.random.seed(seed)

        grid_size = min(4 + difficulty // 2, 10)
        num_ops = min(2 + difficulty // 3, 4)

        # Generate simple input pattern
        input_grid = np.zeros((grid_size, grid_size), dtype=int)
        # Place a simple shape
        mid = grid_size // 2
        input_grid[mid-1:mid+1, mid-1:mid+1] = random.randint(1, 5)
        input_grid[mid, mid] = random.randint(1, 5)

        # Select and compose operations
        ops = random.sample(self.PRIMITIVES, min(num_ops, len(self.PRIMITIVES)))
        output_grid = input_grid.copy()
        for op in ops:
            output_grid = self._apply_primitive(output_grid, op)

        return Problem(
            id=self._make_id(difficulty, seed or 0),
            abstraction_type=self.abstraction_type,
            difficulty=difficulty,
            input_data=input_grid.tolist(),
            output_data=output_grid.tolist(),
            ground_truth_abstraction=f"composition={' -> '.join(ops)}",
            metadata={
                "operations": ops,
                "num_operations": len(ops),
                "arc_compatible": True
            }
        )

    def _apply_primitive(self, grid: np.ndarray, op: str) -> np.ndarray:
        if op == "rotate_90":
            return np.rot90(grid)
        elif op == "flip_h":
            return np.fliplr(grid)
        elif op == "flip_v":
            return np.flipud(grid)
        elif op == "scale_2x":
            return np.repeat(np.repeat(grid, 2, axis=0), 2, axis=1)[:grid.shape[0], :grid.shape[1]]
        elif op == "invert_colors":
            max_val = grid.max()
            return np.where(grid > 0, max_val + 1 - grid, 0)
        else:  # shift_right
            return np.roll(grid, 1, axis=1)

    def validate(self, problem: Problem) -> bool:
        inp = np.array(problem.input_data)
        out = np.array(problem.output_data)
        return np.sum(inp) > 0 and not np.array_equal(inp, out)


# =============================================================================
# TYPE A16: CROSS-MODAL (NOT IN ARC)
# =============================================================================

class CrossModalGenerator(AbstractionGenerator):
    """Generator for cross-modal abstraction problems."""

    abstraction_type = AbstractionType.A16_CROSSMODAL

    # Simple mappings between modalities
    SHAPE_TO_WORD = {
        "square": "stability",
        "circle": "unity",
        "triangle": "direction",
        "line": "connection"
    }

    COLOR_TO_EMOTION = {
        1: "calm",      # blue
        2: "energy",    # red
        3: "growth",    # green
        4: "warmth",    # yellow
        5: "mystery"    # purple
    }

    PATTERN_TO_RHYTHM = {
        "regular": "steady",
        "alternating": "syncopated",
        "clustered": "burst",
        "random": "chaotic"
    }

    def generate(self, difficulty: int, seed: Optional[int] = None) -> Problem:
        if seed:
            random.seed(seed)
            np.random.seed(seed)

        modality_pair = random.choice(["visual_to_verbal", "color_to_emotion", "pattern_to_rhythm"])
        grid_size = 5 + difficulty // 2

        if modality_pair == "visual_to_verbal":
            # Generate grid with shape, ask for word
            shape = random.choice(list(self.SHAPE_TO_WORD.keys()))
            grid = self._draw_shape(shape, grid_size)
            input_data = {
                "modality": "visual",
                "grid": grid.tolist(),
                "query": "What concept does this shape represent?"
            }
            answer = self.SHAPE_TO_WORD[shape]

        elif modality_pair == "color_to_emotion":
            # Grid with dominant color, ask for emotion
            color = random.randint(1, 5)
            grid = np.full((grid_size, grid_size), color, dtype=int)
            input_data = {
                "modality": "color",
                "grid": grid.tolist(),
                "query": "What emotion does this color evoke?"
            }
            answer = self.COLOR_TO_EMOTION[color]

        else:  # pattern_to_rhythm
            # Generate pattern type, ask for rhythm
            pattern_type = random.choice(list(self.PATTERN_TO_RHYTHM.keys()))
            grid = self._generate_pattern(pattern_type, grid_size)
            input_data = {
                "modality": "pattern",
                "grid": grid.tolist(),
                "pattern_type": pattern_type,
                "query": "What rhythm does this pattern suggest?"
            }
            answer = self.PATTERN_TO_RHYTHM[pattern_type]

        return Problem(
            id=self._make_id(difficulty, seed or 0),
            abstraction_type=self.abstraction_type,
            difficulty=difficulty,
            input_data=input_data,
            output_data=answer,
            ground_truth_abstraction=f"crossmodal={modality_pair}",
            metadata={
                "modality_pair": modality_pair,
                "arc_compatible": False
            }
        )

    def _draw_shape(self, shape: str, size: int) -> np.ndarray:
        grid = np.zeros((size, size), dtype=int)
        mid = size // 2
        if shape == "square":
            grid[mid-2:mid+2, mid-2:mid+2] = 1
        elif shape == "circle":
            for i in range(size):
                for j in range(size):
                    if (i - mid)**2 + (j - mid)**2 <= (size//3)**2:
                        grid[i, j] = 1
        elif shape == "triangle":
            for i in range(size//2):
                grid[mid-i:mid+i+1, i] = 1
        else:  # line
            grid[mid, :] = 1
        return grid

    def _generate_pattern(self, pattern_type: str, size: int) -> np.ndarray:
        grid = np.zeros((size, size), dtype=int)
        if pattern_type == "regular":
            grid[::2, ::2] = 1
        elif pattern_type == "alternating":
            for i in range(size):
                grid[i, (i % 2)::2] = 1
        elif pattern_type == "clustered":
            for _ in range(3):
                x, y = random.randint(0, size-2), random.randint(0, size-2)
                grid[x:x+2, y:y+2] = 1
        else:  # random
            grid = np.random.randint(0, 2, size=(size, size))
        return grid

    def validate(self, problem: Problem) -> bool:
        return "modality" in problem.input_data


# =============================================================================
# ATLAS-GEN: MASTER GENERATOR
# =============================================================================

class ATLASGenerator:
    """Master generator that orchestrates all type-specific generators."""
    
    def __init__(self):
        self.generators: Dict[AbstractionType, AbstractionGenerator] = {
            # ARC-compatible types
            AbstractionType.A1_GEOMETRIC: GeometricGenerator(),
            AbstractionType.A2_CHROMATIC: ChromaticGenerator(),
            AbstractionType.A3_TOPOLOGICAL: TopologicalGenerator(),
            AbstractionType.A4_ARITHMETIC: ArithmeticGenerator(),
            AbstractionType.A15_COMPOSITIONAL: CompositionalGenerator(),
            # Beyond-ARC types
            AbstractionType.A5_SEQUENTIAL: SequentialGenerator(),
            AbstractionType.A6_TEMPORAL: TemporalGenerator(),
            AbstractionType.A7_CAUSAL: CausalGenerator(),
            AbstractionType.A8_HIERARCHICAL: HierarchicalGenerator(),
            AbstractionType.A9_ANALOGICAL: AnalogicalGenerator(),
            AbstractionType.A10_LINGUISTIC: LinguisticGenerator(),
            AbstractionType.A11_SOCIAL: SocialGenerator(),
            AbstractionType.A12_COUNTERFACTUAL: CounterfactualGenerator(),
            AbstractionType.A13_RECURSIVE: RecursiveGenerator(),
            AbstractionType.A14_PROBABILISTIC: ProbabilisticGenerator(),
            AbstractionType.A16_CROSSMODAL: CrossModalGenerator(),
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
