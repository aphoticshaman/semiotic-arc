# Multi-Model Prompt Templates for ARC-AGI

Specialized prompts for different model capabilities in ARC-AGI analysis.

---

## VISUAL ANALYSIS PROMPTS

### Grid Pattern Library (for vision-capable models)

```
I'm working on ARC-AGI-3, an interactive reasoning benchmark where AI agents
play novel games on 64x64 grids. Humans score 100%, frontier AI scores 0%.

For each game frame:

1. OBJECTS: List every distinct object you see. Describe by:
   - Color (use terms like "orange", "blue", "pink", "black", "gray")
   - Shape (rectangle, square, irregular cluster)
   - Approximate size (e.g., 4x4 pixels, 2x2, large region)
   - Position (top-left, center, coordinates if possible)

2. PLAYER: Identify the likely player character
   - Usually a small (4x4 to 5x5) distinctive colored block
   - Often has multiple colors (e.g., orange with blue)

3. GOALS: Identify likely goal/exit/door
   - Often has distinctive border or bright color (yellow, pink)
   - May be locked/inaccessible initially

4. OBSTACLES: Identify walls and barriers
   - Usually black or dark gray
   - Block movement

5. INTERACTABLES: Identify items that can be collected or used
   - Keys, energy pills, buttons
   - Often small colored objects

6. HUD: Look at the bottom 4 rows of the grid
   - Energy bars?
   - Score displays?
   - Inventory indicators?

7. WIN HYPOTHESIS: Based on the layout, what do you think the win condition is?

Be as specific as possible.
```

---

### Action Effect Analysis

```
I'm showing you two consecutive frames from an ARC-AGI-3 game.

The action taken between Frame 1 and Frame 2 was: [ACTION]

Analyze the visual difference:

1. MOVEMENT: Did anything move? What direction? How many pixels?

2. APPEARANCE: Did anything new appear in Frame 2?

3. DISAPPEARANCE: Did anything from Frame 1 disappear?

4. COLOR CHANGES: Did any objects change color?

5. HUD CHANGES: Did the bottom rows (HUD area) change?

6. RULE INFERENCE: What rule does this action-effect pair suggest?
   Example: "ACTION1 moves the player up by 4 pixels"

7. BLOCKED?: If nothing changed, hypothesize why

Output as structured JSON:
{
  "action": "ACTION1",
  "effect": "movement_up",
  "player_delta": {"dx": 0, "dy": -4},
  "objects_changed": [...],
  "inferred_rule": "ACTION1 moves player up when not blocked"
}
```

---

## ADVERSARIAL REVIEW PROMPTS

### Strategy Critique

```
Review this approach for ARC-AGI-3:

[PASTE STRATEGY HERE]

For each section, analyze:

1. STRENGTHS: What's good about this approach?

2. WEAKNESSES: What's likely to fail? Where are the gaps?

3. MISSING: What critical elements are absent?

4. FAILURE MODES: How could an agent following this get stuck?

5. IMPROVEMENTS: Specific rewrites or additions?

Focus especially on:
- Does the cognitive stack make sense?
- Is the exploration/exploitation balance right?
- Will this generalize to unknown games?

Be harsh. This is for competition.
```

---

### Red Team Protocol

```
I'm competing with this strategy:

CURRENT APPROACH:
1. First 25-30 actions: Pure exploration
   - Test each action type systematically
   - Build world model (what does each action do?)
   - Find player position, goals, obstacles

2. Goal inference phase:
   - Identify win condition from patterns
   - Look for doors, keys, exits
   - Check HUD for score/energy mechanics

3. Exploitation phase:
   - Plan path to goal
   - Execute minimal action sequence
   - Adapt if blocked

RED TEAM THIS:

1. How does this strategy fail?

2. What game types break this approach?
   - Non-navigation games?
   - Games requiring precise timing?
   - Games with hidden state?

3. What's the fundamental limitation?

4. If you were designing a game to beat this agent, what would you make?

5. What would a winning approach look like instead?
```

---

## ALGORITHM DESIGN PROMPTS

### Pathfinding System

```
Design an optimal exploration and pathfinding system for ARC-AGI-3 games.

CONSTRAINTS:
- Grid: 64x64 cells
- Cell values: integers 0-15 (different object types)
- Movement: typically 4 pixels per action
- Actions: UP, DOWN, LEFT, RIGHT, INTERACT, CLICK(x,y), UNDO
- Some cells block movement (walls)
- Goal position may be UNKNOWN initially
- Action budget is LIMITED — efficiency matters

REQUIREMENTS:

1. EXPLORATION ALGORITHM:
   - Maximize map coverage
   - Learn action effects
   - Identify player, walls, goals
   - Minimize redundant actions

2. PATHFINDING WITH UNKNOWN GOAL:
   - Modified A* balancing:
     - Distance to goal candidates
     - Unexplored area bonus
     - Resource constraints

3. DYNAMIC REPLANNING:
   - What to do when blocked?
   - How to update path on new info?

4. EFFICIENCY OPTIMIZATION:
   - Score = optimal_actions / actual_actions
   - Minimize exploration while maintaining solvability

Provide pseudocode and complexity analysis.
```

---

### Explore/Exploit Tradeoff

```
In ARC-AGI-3:
- Score = optimal_actions / actual_actions (higher = better)
- Agent starts knowing NOTHING
- Must explore to learn, then exploit to win

ANALYSIS REQUEST:

1. EXPLORATION COST MODEL:
   Let E = exploration actions, X = exploitation actions
   Total = E + X

   If exploration insufficient, P(win) decreases
   If exploration excessive, efficiency drops

   Model this tradeoff mathematically.

2. OPTIMAL STOPPING:
   When should exploration end?
   - After testing all actions?
   - After confidence threshold reached?
   - After map coverage percent?

   Derive the optimal stopping rule.

3. INFORMATION VALUE:
   Each exploration action has expected information gain.
   Each exploitation action has expected goal progress.

   Formulate as bandit/MDP problem:
   - State = (knowledge, position)
   - Actions = explore vs exploit
   - Reward = -1 per action, +bonus on win

   What's the optimal policy?
```

---

## UNCONVENTIONAL THINKING PROMPTS

### Alternative Framings

```
ARC-AGI-3 is a new benchmark testing AI generalization.

THE BRUTAL FACTS:
- Humans score: 100%
- Frontier AI (GPT-4, Claude, etc.): 0%
- Interactive games on 64x64 grids
- Agents must discover rules and goals on the fly
- No instructions given — pure exploration

CURRENT AI APPROACHES (all fail):
- LLM-based agents that reason about game state
- Heuristic agents with systematic exploration
- World model builders

YOUR TASK:
Generate 10 unconventional hypotheses for:
1. Why the gap exists (what are humans doing that AI can't?)
2. How to close it (approaches no one is trying)

RULES:
- Be weird
- Be bold
- Ignore what's "reasonable"
- Think like a hacker, not an ML researcher
- Consider cross-domain analogies

I want ideas that would make a traditional ML researcher uncomfortable.
```

---

### Constraint-Consumption Analysis

```
We have a theory called the "Constraint-Consumption Obstruction":

Hard problems share a structure where constraints imposed by the problem
consume the degrees of freedom needed to solve it.

FOR ARC-AGI-3:

1. What constraints does the benchmark format impose on AI agents?
   - Text-based input? (grids as numbers)
   - Token-by-token reasoning?
   - Single-turn decision making?
   - No persistent memory across games?

2. What capabilities do those constraints CONSUME?
   - Spatial reasoning?
   - Temporal abstraction?
   - Analogical transfer?

3. The key question: How do you design an agent that ROUTES AROUND
   these constraints rather than fighting them head-on?

Think orthogonally.
```

---

*Prompt templates for multi-model ARC-AGI analysis*
