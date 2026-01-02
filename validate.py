#!/usr/bin/env python3
"""
SEMIOTIC ANNOTATION VALIDATOR

Validates submissions against schema.
Reports errors, warnings, quality score.

Usage:
    python validate.py data/example_001.json
    python validate.py data/  # validate all
"""

import json
import sys
import os
from pathlib import Path

# Required fields at each level
REQUIRED_ROOT = ["task_id", "perception", "transformation", "reasoning"]
REQUIRED_PERCEPTION = ["objects"]
REQUIRED_OBJECT = ["id", "color", "shape", "semantic_role"]
REQUIRED_TRANSFORMATION = ["operations", "rule_natural_language"]
REQUIRED_REASONING = ["trace", "key_insight"]

VALID_SHAPES = ["point", "line", "rectangle", "square", "L-shape", "T-shape",
                "cross", "diagonal", "irregular", "composite"]
VALID_ROLES = ["agent", "target", "obstacle", "boundary", "marker",
               "background", "path", "container", "contained"]
VALID_OPERATIONS = ["move", "copy", "delete", "create", "resize", "recolor",
                    "rotate", "reflect", "fill", "extend", "crop",
                    "complete_pattern", "apply_mask", "conditional"]
VALID_RELATIONS = ["above", "below", "left_of", "right_of", "inside",
                   "contains", "adjacent", "aligned_h", "aligned_v",
                   "same_color", "same_shape", "touching", "separated"]


class ValidationResult:
    def __init__(self, filename):
        self.filename = filename
        self.errors = []
        self.warnings = []
        self.score = 100

    def error(self, msg):
        self.errors.append(msg)
        self.score -= 20

    def warn(self, msg):
        self.warnings.append(msg)
        self.score -= 5

    def passed(self):
        return len(self.errors) == 0

    def report(self):
        status = "PASS" if self.passed() else "FAIL"
        print(f"\n{'='*60}")
        print(f"[{status}] {self.filename}")
        print(f"Score: {max(0, self.score)}/100")

        if self.errors:
            print(f"\nERRORS ({len(self.errors)}):")
            for e in self.errors:
                print(f"  - {e}")

        if self.warnings:
            print(f"\nWARNINGS ({len(self.warnings)}):")
            for w in self.warnings:
                print(f"  - {w}")

        if self.passed() and not self.warnings:
            print("  Clean submission!")

        return self.passed()


def validate_object(obj, result):
    """Validate a single object entry"""
    for field in REQUIRED_OBJECT:
        if field not in obj:
            result.error(f"Object missing required field: {field}")

    if "id" in obj and not obj["id"]:
        result.error("Object has empty id")

    if "shape" in obj and obj["shape"] not in VALID_SHAPES:
        result.warn(f"Unknown shape: {obj['shape']}")

    if "semantic_role" in obj and obj["semantic_role"] not in VALID_ROLES:
        result.warn(f"Unknown semantic_role: {obj['semantic_role']}")

    if "color" in obj:
        if not isinstance(obj["color"], int) or obj["color"] < 0 or obj["color"] > 9:
            result.error(f"Color must be integer 0-9, got: {obj['color']}")


def validate_relation(rel, object_ids, result):
    """Validate a spatial relation"""
    if "subject" in rel and rel["subject"] not in object_ids:
        result.warn(f"Relation references unknown object: {rel['subject']}")

    if "object" in rel and rel["object"] not in object_ids:
        result.warn(f"Relation references unknown object: {rel['object']}")

    if "predicate" in rel and rel["predicate"] not in VALID_RELATIONS:
        result.warn(f"Unknown relation predicate: {rel['predicate']}")


def validate_operation(op, result):
    """Validate a transformation operation"""
    if "type" in op and op["type"] not in VALID_OPERATIONS:
        result.warn(f"Unknown operation type: {op['type']}")


def validate_file(filepath):
    """Validate a single annotation file"""
    result = ValidationResult(filepath)

    # Load JSON
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            data = json.load(f)
    except json.JSONDecodeError as e:
        result.error(f"Invalid JSON: {e}")
        return result
    except Exception as e:
        result.error(f"Could not read file: {e}")
        return result

    # Check root fields
    for field in REQUIRED_ROOT:
        if field not in data:
            result.error(f"Missing required field: {field}")

    # Validate task_id
    if "task_id" in data:
        if not data["task_id"] or not isinstance(data["task_id"], str):
            result.error("task_id must be non-empty string")

    # Validate perception
    if "perception" in data:
        perc = data["perception"]

        for field in REQUIRED_PERCEPTION:
            if field not in perc:
                result.error(f"perception missing: {field}")

        object_ids = set()
        if "objects" in perc:
            if not isinstance(perc["objects"], list):
                result.error("perception.objects must be array")
            else:
                for obj in perc["objects"]:
                    validate_object(obj, result)
                    if "id" in obj:
                        object_ids.add(obj["id"])

                if len(perc["objects"]) == 0:
                    result.warn("No objects defined")

        # Validate relations reference valid objects
        if "relations" in perc:
            for rel in perc["relations"]:
                validate_relation(rel, object_ids, result)

    # Validate transformation
    if "transformation" in data:
        trans = data["transformation"]

        for field in REQUIRED_TRANSFORMATION:
            if field not in trans:
                result.error(f"transformation missing: {field}")

        if "operations" in trans:
            if not isinstance(trans["operations"], list):
                result.error("transformation.operations must be array")
            else:
                for op in trans["operations"]:
                    validate_operation(op, result)

                if len(trans["operations"]) == 0:
                    result.warn("No operations defined")

        if "rule_natural_language" in trans:
            rule = trans["rule_natural_language"]
            if len(rule) < 10:
                result.warn("rule_natural_language too short - be descriptive")

    # Validate reasoning
    if "reasoning" in data:
        reas = data["reasoning"]

        for field in REQUIRED_REASONING:
            if field not in reas:
                result.error(f"reasoning missing: {field}")

        if "trace" in reas:
            if not isinstance(reas["trace"], list):
                result.error("reasoning.trace must be array")
            elif len(reas["trace"]) < 2:
                result.warn("reasoning.trace should have multiple steps")

        if "key_insight" in reas:
            if len(reas["key_insight"]) < 20:
                result.warn("key_insight too short - explain the 'aha' moment")

    # Quality bonuses
    if "metadata" in data:
        result.score += 5  # Bonus for metadata
        if "difficulty" in data["metadata"]:
            result.score += 2

    if "patterns" in data and data["patterns"]:
        result.score += 5  # Bonus for pattern analysis

    if "hypotheses_considered" in data.get("reasoning", {}):
        if len(data["reasoning"]["hypotheses_considered"]) > 1:
            result.score += 5  # Bonus for showing rejected hypotheses

    result.score = min(100, max(0, result.score))
    return result


def main():
    if len(sys.argv) < 2:
        print("Usage: python validate.py <file.json or directory>")
        sys.exit(1)

    target = sys.argv[1]

    if os.path.isdir(target):
        files = list(Path(target).glob("*.json"))
        if not files:
            print(f"No JSON files found in {target}")
            sys.exit(1)
    else:
        files = [Path(target)]

    results = []
    for f in files:
        if f.name.startswith("."):
            continue
        result = validate_file(str(f))
        result.report()
        results.append(result)

    # Summary
    passed = sum(1 for r in results if r.passed())
    total = len(results)

    print(f"\n{'='*60}")
    print(f"SUMMARY: {passed}/{total} passed")

    if passed < total:
        sys.exit(1)


if __name__ == "__main__":
    main()
