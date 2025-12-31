//! Prior Bank: Amortized inference via compiled priors
//!
//! Layer 3 of the architecture.
//!
//! The "Core Knowledge" exception is infinitely elastic.
//! We compile game priors, not memorize tasks.
//! Inference becomes O(1) lookup.

use crate::dsl::RuleClass;
use crate::Abstraction;
use hashbrown::HashMap;
use std::hash::{Hash, Hasher};

/// Distribution over rule classes
#[derive(Clone, Debug)]
pub struct HypothesisDistribution {
    /// Map from rule class to probability
    probs: HashMap<RuleClass, f32>,
    /// Cached entropy
    cached_entropy: Option<f32>,
}

impl HypothesisDistribution {
    pub fn new() -> Self {
        Self {
            probs: HashMap::new(),
            cached_entropy: None,
        }
    }

    pub fn uniform(classes: &[RuleClass]) -> Self {
        let p = 1.0 / classes.len() as f32;
        let mut dist = Self::new();
        for c in classes {
            dist.probs.insert(c.clone(), p);
        }
        dist
    }

    pub fn singleton(class: RuleClass) -> Self {
        let mut dist = Self::new();
        dist.probs.insert(class, 1.0);
        dist
    }

    /// Get probability of a rule class
    pub fn prob(&self, class: &RuleClass) -> f32 {
        *self.probs.get(class).unwrap_or(&0.0)
    }

    /// Set probability of a rule class
    pub fn set(&mut self, class: RuleClass, prob: f32) {
        self.probs.insert(class, prob);
        self.cached_entropy = None;
    }

    /// Normalize to sum to 1
    pub fn normalize(&mut self) {
        let sum: f32 = self.probs.values().sum();
        if sum > 0.0 {
            for p in self.probs.values_mut() {
                *p /= sum;
            }
        }
        self.cached_entropy = None;
    }

    /// Shannon entropy H = -Σ p log p
    pub fn entropy(&self) -> f32 {
        if let Some(e) = self.cached_entropy {
            return e;
        }

        let h: f32 = self
            .probs
            .values()
            .filter(|&&p| p > 0.0)
            .map(|&p| -p * p.log2())
            .sum();

        h
    }

    /// Confidence = 1 - normalized entropy
    pub fn confidence(&self) -> f32 {
        let n = self.probs.len() as f32;
        if n <= 1.0 {
            return 1.0;
        }
        let max_entropy = n.log2();
        1.0 - (self.entropy() / max_entropy).min(1.0)
    }

    /// Most likely rule class
    pub fn argmax(&self) -> RuleClass {
        self.probs
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(c, _)| c.clone())
            .unwrap_or(RuleClass::Unknown)
    }

    /// Update with observation (Bayesian update)
    pub fn update(&mut self, observation: &Observation) {
        for (class, prob) in self.probs.iter_mut() {
            let likelihood = observation.likelihood(class);
            *prob *= likelihood;
        }
        self.normalize();
    }
}

/// Observation for Bayesian updating
pub struct Observation {
    /// What changed after an action
    pub change_type: ChangeType,
    /// How many cells changed
    pub cells_changed: u32,
    /// Pattern of change
    pub change_pattern: ChangePattern,
}

impl Observation {
    /// Likelihood of this observation given rule class
    pub fn likelihood(&self, class: &RuleClass) -> f32 {
        // TODO: Learn these from data
        // For now, simple heuristics
        match (class, &self.change_type) {
            (RuleClass::Navigation(_), ChangeType::Movement) => 0.9,
            (RuleClass::Navigation(_), ChangeType::Toggle) => 0.1,
            (RuleClass::Toggle(_), ChangeType::Toggle) => 0.9,
            (RuleClass::Toggle(_), ChangeType::Movement) => 0.1,
            _ => 0.5,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ChangeType {
    Movement,
    Toggle,
    Fill,
    None,
}

#[derive(Clone, Debug)]
pub enum ChangePattern {
    SingleCell,
    Cross,
    Neighborhood,
    Row,
    Column,
    Scattered,
}

/// The Prior Bank
///
/// Stores compiled priors for fast lookup.
/// This is where the "Core Knowledge" lives.
pub struct PriorBank {
    /// Fast lookup by abstraction hash
    lookup_table: HashMap<u64, HypothesisDistribution>,

    /// Default prior when no match
    default_prior: HypothesisDistribution,

    /// All known rule classes
    known_classes: Vec<RuleClass>,
}

impl PriorBank {
    pub fn new() -> Self {
        use crate::dsl::{NavigationRule, ToggleRule, PatternRule};

        // Initialize with known archetypes
        let known_classes = vec![
            RuleClass::Navigation(NavigationRule::SimpleWalls),
            RuleClass::Navigation(NavigationRule::KeyDoor),
            RuleClass::Navigation(NavigationRule::HealthHazard),
            RuleClass::Toggle(ToggleRule::SingleCell),
            RuleClass::Toggle(ToggleRule::Neighborhood3x3),
            RuleClass::Toggle(ToggleRule::CrossPattern),
            RuleClass::PatternFill(PatternRule::MatchTemplate),
            RuleClass::PatternFill(PatternRule::Mirror),
        ];

        Self {
            lookup_table: HashMap::new(),
            default_prior: HypothesisDistribution::uniform(&known_classes),
            known_classes,
        }
    }

    /// Lookup prior distribution for an abstraction
    pub fn lookup(&self, abstraction: &Abstraction) -> HypothesisDistribution {
        let hash = hash_abstraction(abstraction);

        self.lookup_table
            .get(&hash)
            .cloned()
            .unwrap_or_else(|| self.infer_prior(abstraction))
    }

    /// Infer prior from abstraction features
    fn infer_prior(&self, abstraction: &Abstraction) -> HypothesisDistribution {
        let mut dist = self.default_prior.clone();

        // Use features to adjust prior
        // More objects → more likely to be object manipulation
        if abstraction.object_count > 5 {
            if let Some(p) = dist.probs.get_mut(&RuleClass::Toggle(
                crate::dsl::ToggleRule::SingleCell,
            )) {
                *p *= 2.0;
            }
        }

        // High symmetry → more likely pattern matching
        if !abstraction.has_symmetry.is_empty() {
            if let Some(p) = dist.probs.get_mut(&RuleClass::PatternFill(
                crate::dsl::PatternRule::Mirror,
            )) {
                *p *= 2.0;
            }
        }

        dist.normalize();
        dist
    }

    /// Add learned prior to bank
    pub fn add_prior(&mut self, abstraction: &Abstraction, dist: HypothesisDistribution) {
        let hash = hash_abstraction(abstraction);
        self.lookup_table.insert(hash, dist);
    }

    /// Load priors from file (trained data)
    pub fn load(&mut self, _path: &str) -> Result<(), std::io::Error> {
        // TODO: Implement loading from trained prior bank
        Ok(())
    }

    /// Save priors to file
    pub fn save(&self, _path: &str) -> Result<(), std::io::Error> {
        // TODO: Implement saving
        Ok(())
    }
}

fn hash_abstraction(a: &Abstraction) -> u64 {
    use std::collections::hash_map::DefaultHasher;

    let mut hasher = DefaultHasher::new();
    a.color_histogram.hash(&mut hasher);
    a.object_count.hash(&mut hasher);
    a.bounding_box.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hypothesis_distribution() {
        let classes = vec![
            RuleClass::Unknown,
            RuleClass::Navigation(crate::dsl::NavigationRule::SimpleWalls),
        ];
        let dist = HypothesisDistribution::uniform(&classes);

        assert!((dist.prob(&RuleClass::Unknown) - 0.5).abs() < 0.001);
        assert!(dist.entropy() > 0.0);
    }
}
