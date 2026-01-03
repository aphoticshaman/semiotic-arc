"""
Asymmetric Ratcheting Loss Function

A loss function that enforces monotonic improvement in state transitions.
Key insight: Only accept modifications that improve utility above a threshold.

This implements counterfactual reasoning by explicitly evaluating utility
over hypothetical future states before committing to changes.
"""

from dataclasses import dataclass
from typing import Callable, Any, Tuple, Optional
import math


@dataclass
class RatchetConfig:
    """Configuration for asymmetric ratcheting."""
    epsilon: float = 0.0          # Acceptance threshold
    adaptive: bool = False        # Enable adaptive epsilon
    confidence_weight: float = 1.0  # Weight for confidence adjustment
    min_epsilon: float = -0.1     # Floor for adaptive epsilon
    max_epsilon: float = 1.0      # Ceiling for adaptive epsilon


class RatchetLoss:
    """
    Asymmetric ratcheting loss function.

    Mathematical definition:
        L_ratchet(m | s_t) = max(0, -Δ(s_t, m) + ε)

    Where:
        Δ(s_t, m) = U(W(s_t, m)) - U(s_t)
        U = utility function (from survival pressure, G4)
        W = world model (counterfactual generator)
        ε = acceptance threshold

    Properties:
        - Returns 0 when modification improves utility by at least ε
        - Returns positive loss otherwise
        - Hinge loss structure enables gradient-based learning
    """

    def __init__(self, config: Optional[RatchetConfig] = None):
        self.config = config or RatchetConfig()
        self._epsilon = self.config.epsilon

    def __call__(self, delta: float, confidence: float = 1.0) -> float:
        """
        Compute ratcheting loss.

        Args:
            delta: U(W(s,m)) - U(s), predicted utility change
            confidence: Model confidence in prediction (0-1)

        Returns:
            loss: 0 if delta >= epsilon, else epsilon - delta
        """
        epsilon = self._get_epsilon(confidence)
        return max(0.0, epsilon - delta)

    def _get_epsilon(self, confidence: float) -> float:
        """Get acceptance threshold, optionally adjusted for confidence."""
        if not self.config.adaptive:
            return self._epsilon

        # Lower confidence -> higher epsilon (more conservative)
        adjusted = self._epsilon + self.config.confidence_weight * (1 - confidence)
        return max(self.config.min_epsilon, min(self.config.max_epsilon, adjusted))

    def gradient(self, delta: float, confidence: float = 1.0) -> float:
        """
        Compute gradient of ratcheting loss w.r.t. delta.

        Returns:
            -1 if delta < epsilon (modification should improve)
            0 otherwise (already acceptable)
        """
        epsilon = self._get_epsilon(confidence)
        return -1.0 if delta < epsilon else 0.0


class RatchetDecision:
    """
    Asymmetric ratchet decision rule.

    Implements the core ratcheting mechanism:
        s_{t+1} = W(s_t, m*) if Δ(s_t, m*) > ε
                = s_t        otherwise

    This computationally instantiates counterfactual reasoning (G5)
    by explicitly evaluating utility over non-actual states.
    """

    def __init__(
        self,
        world_model: Callable[[Any, Any], Any],
        utility: Callable[[Any], float],
        config: Optional[RatchetConfig] = None
    ):
        """
        Args:
            world_model: W(s, m) -> s', predicts state after modification
            utility: U(s) -> R, evaluates state utility
            config: Ratcheting configuration
        """
        self.world_model = world_model
        self.utility = utility
        self.config = config or RatchetConfig()
        self.loss_fn = RatchetLoss(self.config)

    def evaluate(self, state: Any, modification: Any) -> Tuple[float, float, Any]:
        """
        Evaluate a candidate modification.

        Args:
            state: Current state s_t
            modification: Candidate modification m

        Returns:
            (delta, loss, predicted_state)
        """
        predicted_state = self.world_model(state, modification)
        current_utility = self.utility(state)
        predicted_utility = self.utility(predicted_state)
        delta = predicted_utility - current_utility
        loss = self.loss_fn(delta)
        return delta, loss, predicted_state

    def decide(
        self,
        state: Any,
        modification: Any,
        apply_fn: Callable[[Any, Any], Any]
    ) -> Tuple[bool, Any, dict]:
        """
        Make ratcheting decision.

        Args:
            state: Current state s_t
            modification: Candidate modification m
            apply_fn: Function to actually apply modification

        Returns:
            (accepted, new_state, info_dict)
        """
        delta, loss, predicted_state = self.evaluate(state, modification)

        accepted = delta > self.config.epsilon

        if accepted:
            new_state = apply_fn(state, modification)
        else:
            new_state = state

        info = {
            'delta': delta,
            'loss': loss,
            'epsilon': self.config.epsilon,
            'accepted': accepted,
            'predicted_utility': self.utility(predicted_state),
            'actual_utility': self.utility(new_state) if accepted else self.utility(state)
        }

        return accepted, new_state, info


class MonotonicityGuard:
    """
    Monitors ratcheting monotonicity guarantee.

    Theorem (from ratchet_formalization.tex):
        U(s_{t+1}) >= U(s_t) - δ_model

    Tracks violations to detect world model degradation.
    """

    def __init__(self, tolerance: float = 0.1):
        self.tolerance = tolerance  # δ_model bound
        self.history = []
        self.violations = []

    def check(self, utility_before: float, utility_after: float, accepted: bool) -> bool:
        """
        Check monotonicity guarantee.

        Returns:
            True if monotonicity holds within tolerance
        """
        self.history.append({
            'before': utility_before,
            'after': utility_after,
            'accepted': accepted
        })

        if accepted:
            # Should have U(s_{t+1}) >= U(s_t) - tolerance
            violation = utility_after < utility_before - self.tolerance
            if violation:
                self.violations.append(self.history[-1])
            return not violation
        else:
            # No modification, utility unchanged
            return True

    @property
    def violation_rate(self) -> float:
        """Fraction of decisions that violated monotonicity."""
        if not self.history:
            return 0.0
        return len(self.violations) / len(self.history)


# =============================================================================
# FACTORY FUNCTION
# =============================================================================

def create_ratchet_decision(
    world_model: Callable,
    utility: Callable,
    epsilon: float = 0.0,
    adaptive: bool = True
) -> RatchetDecision:
    """
    Factory function for creating a ratchet decision system.

    Usage:
        ratchet = create_ratchet_decision(
            world_model=predict_next_state,
            utility=evaluate_state,
            epsilon=0.01,
            adaptive=True
        )

        accepted, new_state, info = ratchet.decide(
            state=current_state,
            modification=candidate_action,
            apply_fn=apply_action
        )
    """
    config = RatchetConfig(
        epsilon=epsilon,
        adaptive=adaptive,
        confidence_weight=0.5,
        min_epsilon=-0.05,
        max_epsilon=0.5
    )
    return RatchetDecision(world_model, utility, config)


# =============================================================================
# TESTS
# =============================================================================

def test_ratchet_loss():
    """Test ratcheting loss function."""
    loss_fn = RatchetLoss(RatchetConfig(epsilon=0.1))

    # Improvement above threshold -> zero loss
    assert loss_fn(0.2) == 0.0, "Should accept large improvement"

    # Improvement below threshold -> positive loss
    assert loss_fn(0.05) == 0.05, "Should penalize small improvement"

    # Regression -> positive loss
    assert loss_fn(-0.1) == 0.2, "Should penalize regression"

    print("test_ratchet_loss: PASS")


def test_ratchet_decision():
    """Test ratcheting decision rule."""
    # Simple world model: modification adds delta to state
    world_model = lambda s, m: s + m
    utility = lambda s: s  # Utility = state value

    ratchet = RatchetDecision(world_model, utility, RatchetConfig(epsilon=0.0))

    # Positive modification should be accepted
    accepted, new_state, info = ratchet.decide(
        state=1.0,
        modification=0.5,
        apply_fn=lambda s, m: s + m
    )
    assert accepted, "Should accept positive modification"
    assert new_state == 1.5, "Should apply modification"

    # Negative modification should be rejected
    accepted, new_state, info = ratchet.decide(
        state=1.0,
        modification=-0.5,
        apply_fn=lambda s, m: s + m
    )
    assert not accepted, "Should reject negative modification"
    assert new_state == 1.0, "Should preserve state"

    print("test_ratchet_decision: PASS")


def test_monotonicity_guard():
    """Test monotonicity monitoring."""
    guard = MonotonicityGuard(tolerance=0.1)

    # Normal improvement
    assert guard.check(1.0, 1.5, accepted=True), "Should pass on improvement"

    # Small regression within tolerance
    assert guard.check(1.5, 1.45, accepted=True), "Should pass within tolerance"

    # Large regression violates monotonicity
    assert not guard.check(1.45, 1.0, accepted=True), "Should fail on large regression"

    assert len(guard.violations) == 1, "Should track one violation"

    print("test_monotonicity_guard: PASS")


if __name__ == "__main__":
    test_ratchet_loss()
    test_ratchet_decision()
    test_monotonicity_guard()
    print("\nAll tests passed.")
    print("\nRatcheting loss function ready for use.")
