#!/usr/bin/env python3
"""
Semiotic Training Data Generator
================================
Generates training examples that teach the model to:
1. Detect evolved (hardwired) visual patterns
2. Recognize conditioned (learned) symbolic associations
3. Identify reality models being simulated

Crystalline Labs × Ryan Cardwell × Claude
"""

import json
import random
from typing import Dict, List, Tuple, Any
from pathlib import Path
from dataclasses import dataclass
from enum import Enum

# ═══════════════════════════════════════════════════════════════════════════
# EVOLVED PATTERNS (HARDWIRED)
# ═══════════════════════════════════════════════════════════════════════════

class EvolvedPattern(Enum):
    BILATERAL_SYMMETRY = "bilateral_symmetry"
    RADIAL_SYMMETRY = "radial_symmetry"
    ENCLOSURE = "enclosure"
    PROXIMITY_GROUPING = "proximity_grouping"
    EDGE_BOUNDARY = "edge_boundary"
    FACE_LIKE = "face_like"
    CONTINUITY = "continuity"
    SMALL_NUMBER = "small_number"  # 1-4 objects (subitizing)

EVOLVED_DESCRIPTIONS = {
    EvolvedPattern.BILATERAL_SYMMETRY: "The pattern shows bilateral (mirror) symmetry along a vertical or horizontal axis. This triggers our evolved face/body detection systems.",
    EvolvedPattern.RADIAL_SYMMETRY: "The pattern shows radial symmetry around a center point. This triggers our evolved flower/eye detection systems.",
    EvolvedPattern.ENCLOSURE: "One region completely surrounds another, creating inside/outside distinction. This triggers our evolved containment reasoning.",
    EvolvedPattern.PROXIMITY_GROUPING: "Objects close together are perceived as a group. This is Gestalt proximity - things near each other belong together.",
    EvolvedPattern.EDGE_BOUNDARY: "Clear edges separate distinct regions. This triggers our evolved edge detection (V1 cortex).",
    EvolvedPattern.FACE_LIKE: "Two elements above one element in triangular arrangement. This triggers pareidolia - our face detection system.",
    EvolvedPattern.CONTINUITY: "Elements arranged along a line or curve. This triggers our evolved motion tracking - lines continue in their direction.",
    EvolvedPattern.SMALL_NUMBER: "1-4 distinct objects that can be instantly counted (subitizing). Our approximate number system handles this without counting.",
}

# ═══════════════════════════════════════════════════════════════════════════
# CONDITIONED SYMBOLS (LEARNED)
# ═══════════════════════════════════════════════════════════════════════════

class ConditionedSymbol(Enum):
    RED_DANGER = "red_danger"
    GREEN_GO = "green_go"
    BLUE_CALM = "blue_calm"
    YELLOW_ATTENTION = "yellow_attention"
    BLACK_BOUNDARY = "black_boundary"
    UP_POSITIVE = "up_positive"
    DOWN_NEGATIVE = "down_negative"
    CENTER_IMPORTANT = "center_important"
    ONE_UNIQUE = "one_unique"
    TWO_RELATION = "two_relation"
    THREE_PATTERN = "three_pattern"
    BIGGER_MORE = "bigger_more"

CONDITIONED_DESCRIPTIONS = {
    ConditionedSymbol.RED_DANGER: "Red signals danger, importance, or stopping. Conditioned through blood, fire, and traffic signs.",
    ConditionedSymbol.GREEN_GO: "Green signals go, safety, or nature. Conditioned through vegetation and traffic lights.",
    ConditionedSymbol.BLUE_CALM: "Blue signals calm, water, or sky. Conditioned through ocean and atmosphere associations.",
    ConditionedSymbol.YELLOW_ATTENTION: "Yellow signals caution or energy. Conditioned through sun and warning signs.",
    ConditionedSymbol.BLACK_BOUNDARY: "Black as background or boundary. Conditioned as void, absence, or edge.",
    ConditionedSymbol.UP_POSITIVE: "Upward direction = good, more, growth. Conditioned through posture and filling vessels.",
    ConditionedSymbol.DOWN_NEGATIVE: "Downward direction = bad, less, decay. Conditioned through falling and draining.",
    ConditionedSymbol.CENTER_IMPORTANT: "Center position = important, focus. Conditioned through attention and targeting.",
    ConditionedSymbol.ONE_UNIQUE: "Single item = unique, special, protagonist. The singleton stands out.",
    ConditionedSymbol.TWO_RELATION: "Pair of items = relationship, comparison, dialogue. Duality implies interaction.",
    ConditionedSymbol.THREE_PATTERN: "Three items = pattern established. The triangle of stability.",
    ConditionedSymbol.BIGGER_MORE: "Larger size = more important, more quantity. Size correlates with significance.",
}

# ═══════════════════════════════════════════════════════════════════════════
# REALITY MODELS
# ═══════════════════════════════════════════════════════════════════════════

class RealityModel(Enum):
    GRAVITY = "gravity"
    FLUID_FILL = "fluid_fill"
    COLLISION = "collision"
    GROWTH = "growth"
    SET_OPERATION = "set_operation"
    GRAPH_OPERATION = "graph_operation"
    CLASSIFICATION = "classification"
    ANALOGY = "analogy"
    ATTENTION = "attention"
    MEMORY = "memory"

REALITY_DESCRIPTIONS = {
    RealityModel.GRAVITY: "Physical simulation where objects fall downward until blocked. Models gravity in the physical world.",
    RealityModel.FLUID_FILL: "Fluid dynamics where color/state spreads to fill enclosed regions. Models liquid filling containers.",
    RealityModel.COLLISION: "Objects interact when they meet. Models physical collision and blocking.",
    RealityModel.GROWTH: "Patterns expand or contract following rules. Models cellular automata, life, decay.",
    RealityModel.SET_OPERATION: "Operations on collections: union, intersection, difference. Models mathematical set theory.",
    RealityModel.GRAPH_OPERATION: "Operations on connectivity: paths, trees, networks. Models graph theory.",
    RealityModel.CLASSIFICATION: "Sorting, categorizing, grouping by properties. Models taxonomy and organization.",
    RealityModel.ANALOGY: "Consistent mapping between domains. If A:B then C:D. Models analogical reasoning.",
    RealityModel.ATTENTION: "Focus, salience, filtering. Some elements matter more than others.",
    RealityModel.MEMORY: "Copy, recall, modify. Information is preserved and transformed.",
}

# ═══════════════════════════════════════════════════════════════════════════
# TRAINING EXAMPLE TEMPLATES
# ═══════════════════════════════════════════════════════════════════════════

def create_semiotic_scan_example(
    grid_description: str,
    evolved_patterns: List[EvolvedPattern],
    conditioned_symbols: List[ConditionedSymbol],
    reality_model: RealityModel,
    transformation_hint: str
) -> Dict[str, str]:
    """Create a training example that teaches semiotic analysis."""

    evolved_text = "\n".join([
        f"- {p.value}: {EVOLVED_DESCRIPTIONS[p]}"
        for p in evolved_patterns
    ])

    conditioned_text = "\n".join([
        f"- {s.value}: {CONDITIONED_DESCRIPTIONS[s]}"
        for s in conditioned_symbols
    ])

    prompt = f"""Perform a semiotic analysis of this ARC puzzle:

{grid_description}

Identify:
1. Evolved (hardwired) visual patterns present
2. Conditioned (learned) symbolic associations
3. The reality model being simulated
4. How these combine to predict the transformation"""

    response = f"""## Semiotic Analysis

### Evolved Patterns (Hardwired)
{evolved_text}

### Conditioned Symbols (Learned)
{conditioned_text}

### Reality Model
{reality_model.value}: {REALITY_DESCRIPTIONS[reality_model]}

### Transformation Prediction
{transformation_hint}

The transformation follows from the reality model. The evolved patterns determine what stands out (salience), the conditioned symbols determine what it means (semantics), and the reality model determines what happens (dynamics)."""

    return {
        "prompt": prompt,
        "response": response,
        "metadata": {
            "type": "semiotic_scan",
            "evolved": [p.value for p in evolved_patterns],
            "conditioned": [s.value for s in conditioned_symbols],
            "reality_model": reality_model.value
        }
    }


def create_channel_conflict_example(
    grid_description: str,
    evolved_says: str,
    conditioned_says: str,
    resolution: str
) -> Dict[str, str]:
    """Create example showing nature vs nurture conflict resolution."""

    prompt = f"""This ARC puzzle shows a conflict between evolved and conditioned interpretations:

{grid_description}

Analyze the conflict and determine which channel should dominate."""

    response = f"""## Channel Conflict Analysis

### Evolved Pattern Detection Says:
{evolved_says}

### Conditioned Symbol Association Says:
{conditioned_says}

### Conflict Resolution:
{resolution}

When evolved patterns and conditioned symbols disagree, look for:
1. Which channel the examples consistently reward
2. Whether the transformation is physical (favor evolved) or semantic (favor conditioned)
3. Edge cases that disambiguate the rule"""

    return {
        "prompt": prompt,
        "response": response,
        "metadata": {
            "type": "channel_conflict"
        }
    }


def create_reality_model_inference_example(
    examples_description: str,
    reality_model: RealityModel,
    evidence: List[str]
) -> Dict[str, str]:
    """Create example teaching reality model inference from examples."""

    evidence_text = "\n".join([f"- {e}" for e in evidence])

    prompt = f"""Given these ARC training examples:

{examples_description}

What reality is this puzzle modeling? What real-world phenomenon does the transformation simulate?"""

    response = f"""## Reality Model Inference

### Identified Model: {reality_model.value}
{REALITY_DESCRIPTIONS[reality_model]}

### Evidence:
{evidence_text}

### Application:
Once the reality model is identified, apply its rules to the test input. The transformation isn't arbitrary - it follows the physics/logic of the reality being modeled.

Remember: ARC puzzles aren't games - they're compressed reality simulations. The transformation rule IS the simulation step."""

    return {
        "prompt": prompt,
        "response": response,
        "metadata": {
            "type": "reality_model_inference",
            "model": reality_model.value
        }
    }


# ═══════════════════════════════════════════════════════════════════════════
# CONCRETE EXAMPLES
# ═══════════════════════════════════════════════════════════════════════════

SEMIOTIC_EXAMPLES = [
    # Gravity example
    create_semiotic_scan_example(
        grid_description="Input shows colored blocks scattered across a black grid. Output shows the same blocks at the bottom of the grid, stacked.",
        evolved_patterns=[EvolvedPattern.SMALL_NUMBER, EvolvedPattern.PROXIMITY_GROUPING],
        conditioned_symbols=[ConditionedSymbol.DOWN_NEGATIVE, ConditionedSymbol.BLACK_BOUNDARY],
        reality_model=RealityModel.GRAVITY,
        transformation_hint="Objects fall down (gravity) until they hit the bottom or another object. This is physical simulation - blocks obey gravity."
    ),

    # Flood fill example
    create_semiotic_scan_example(
        grid_description="Input shows a red rectangle with a small blue square inside. Output shows the entire interior of the rectangle filled with blue.",
        evolved_patterns=[EvolvedPattern.ENCLOSURE, EvolvedPattern.EDGE_BOUNDARY],
        conditioned_symbols=[ConditionedSymbol.BLUE_CALM, ConditionedSymbol.RED_DANGER],
        reality_model=RealityModel.FLUID_FILL,
        transformation_hint="The inner color floods the enclosed space. Like water filling a container. The enclosure boundary contains the spread."
    ),

    # Classification example
    create_semiotic_scan_example(
        grid_description="Input shows objects of different colors scattered randomly. Output shows objects arranged in columns, grouped by color.",
        evolved_patterns=[EvolvedPattern.PROXIMITY_GROUPING, EvolvedPattern.SMALL_NUMBER],
        conditioned_symbols=[ConditionedSymbol.CENTER_IMPORTANT],
        reality_model=RealityModel.CLASSIFICATION,
        transformation_hint="Objects are sorted by color into groups. This is classification - like sorting laundry or organizing files."
    ),

    # Symmetry completion example
    create_semiotic_scan_example(
        grid_description="Input shows half of a pattern on the left side with the right side empty. Output completes the pattern with bilateral symmetry.",
        evolved_patterns=[EvolvedPattern.BILATERAL_SYMMETRY, EvolvedPattern.EDGE_BOUNDARY],
        conditioned_symbols=[],
        reality_model=RealityModel.ANALOGY,
        transformation_hint="The pattern is reflected to create bilateral symmetry. Our evolved symmetry detection expects completion - like seeing half a face, we know what the other half should be."
    ),

    # Attention/highlighting example
    create_semiotic_scan_example(
        grid_description="Input shows a complex pattern with one element in a different color. Output shows only that different element, everything else removed.",
        evolved_patterns=[EvolvedPattern.SMALL_NUMBER],
        conditioned_symbols=[ConditionedSymbol.ONE_UNIQUE, ConditionedSymbol.CENTER_IMPORTANT],
        reality_model=RealityModel.ATTENTION,
        transformation_hint="The unique element captures attention - it's the figure, everything else is ground. This models selective attention."
    ),

    # Growth example
    create_semiotic_scan_example(
        grid_description="Input shows a small colored region. Output shows the region expanded by one cell in all directions.",
        evolved_patterns=[EvolvedPattern.EDGE_BOUNDARY],
        conditioned_symbols=[ConditionedSymbol.BIGGER_MORE],
        reality_model=RealityModel.GROWTH,
        transformation_hint="The region grows outward like cellular growth or spreading fire. Each iteration expands the boundary."
    ),

    # Channel conflict example
    create_channel_conflict_example(
        grid_description="Input has two objects: one red and small, one blue and large. They are close together (proximity), but different colors.",
        evolved_says="These objects are a GROUP because they are close together (proximity principle).",
        conditioned_says="These objects are DIFFERENT because red vs blue signals different categories.",
        resolution="Check the training examples. If transformations treat them together despite color, evolved wins (spatial grouping). If transformations treat them separately despite proximity, conditioned wins (semantic categories). In ARC, color often defines category more strongly than spatial proximity."
    ),

    # Reality model inference
    create_reality_model_inference_example(
        examples_description="Example 1: Scattered dots → dots moved to bottom. Example 2: Different scattered dots → dots moved to bottom. Example 3: More dots at different positions → all at bottom.",
        reality_model=RealityModel.GRAVITY,
        evidence=[
            "All objects move downward consistently",
            "Objects stop at the bottom boundary",
            "Objects stack when they meet (no overlapping)",
            "Horizontal position is preserved (no lateral movement)"
        ]
    ),
]


def generate_training_file(output_path: str = "semiotic_training.jsonl"):
    """Generate the training JSONL file."""

    output = Path(output_path)

    with output.open("w", encoding="utf-8") as f:
        for example in SEMIOTIC_EXAMPLES:
            # Format for chat fine-tuning
            training_example = {
                "messages": [
                    {"role": "system", "content": "You are an expert at semiotic analysis of visual puzzles. You understand that visual patterns have both evolved (hardwired) and conditioned (learned) components, and that puzzles are compressed reality models."},
                    {"role": "user", "content": example["prompt"]},
                    {"role": "assistant", "content": example["response"]}
                ],
                "metadata": example["metadata"]
            }
            f.write(json.dumps(training_example, ensure_ascii=False) + "\n")

    print(f"Generated {len(SEMIOTIC_EXAMPLES)} semiotic training examples to {output_path}")


def main():
    generate_training_file("semiotic_training.jsonl")

    # Also generate a reference document
    with open("SEMIOTIC_REFERENCE.md", "w", encoding="utf-8") as f:
        f.write("# Semiotic Analysis Reference\n\n")

        f.write("## Evolved Patterns (Hardwired)\n\n")
        for pattern, desc in EVOLVED_DESCRIPTIONS.items():
            f.write(f"### {pattern.value}\n{desc}\n\n")

        f.write("## Conditioned Symbols (Learned)\n\n")
        for symbol, desc in CONDITIONED_DESCRIPTIONS.items():
            f.write(f"### {symbol.value}\n{desc}\n\n")

        f.write("## Reality Models\n\n")
        for model, desc in REALITY_DESCRIPTIONS.items():
            f.write(f"### {model.value}\n{desc}\n\n")

    print("Generated SEMIOTIC_REFERENCE.md")


if __name__ == "__main__":
    main()
