# ARC-AGI-3 Architecture Insights

## What is ARC-AGI-3?

- "Atari on steroids" - interactive web-based mini games
- NOT static puzzles like ARC-AGI-2
- Games are NOT explained by design (discovering mechanics IS the test)
- API at `https://three.arcprize.org`
- Actions: RESET, ACTION1-7 (6=click with x,y coords, 1-4=directional)

---

## Key Insights from Testing

### Game Mechanics Discovery
1. Click interactions move objects between locations
2. Alternating clicks controls flow/traffic routing
3. Goals involve routing objects into target zones (like assembly lines)
4. Some levels are just alternating clicks between nodes
5. Temperature annealing via prompt injection WORKS
6. Code should be actuator only - model makes ALL decisions

### Observed Patterns
1. Clicking specific colors can score, but score resets on wrong clicks
2. Pure alternating heuristics yield 0 score (there's deeper structure)
3. Frame changes after scoring - must re-identify targets

---

## Architecture Principles

```
[ARC-AGI-3 API] <---> [ACTUATOR (code)]
                           |
                           v
                    [Rate Limiter]
                           |
                           v
                    [Frame Parser]
                           |
                           v
                    [LLM Decision Engine]
                           ^
                           |
                    [Memory/AAR System]
                           |
                    [Persistent Learning:
                     - successful_sequences
                     - failed_hypotheses
                     - meta_insights]
```

**Core Principle**: Code NEVER decides gameplay. Code only:
1. Captures frames
2. Sends state to model
3. Executes model's action
4. Records outcome
5. Updates memory

---

## Challenges

### 1. Game Mechanics Unknown
- No documentation on what actions do in each game
- Blind exploration wastes actions
- **Solution**: Systematic mechanics discovery via controlled tests

### 2. Architecture Evolution
- Original approach had hardcoded game logic (if-else for colors)
- Problem: Code making too many decisions
- **Solution**: Actuator-only architecture where model decides

### 3. Rate Limiting
- API may have rate limits
- **Solution**: MIN_INTERVAL = 0.5s between calls

---

## Strategies That Work

### For LLM Agents
1. **Semiotic mega-prompts** with:
   - Identity priming (S-tier game-playing AI)
   - Zero-sum protocol (never random actions)
   - Temperature annealing via prompt
   - Meta-strategy hunting

2. **Memory-augmented learning**:
   - Persistent JSON storage across sessions
   - Successful sequence replay
   - Failed hypothesis tracking

3. **Progressive discovery**:
   - Map action→effect relationships first
   - Identify invariants (what never changes)
   - Find goal patterns before optimizing

---

## Multi-Model Coordination

Different models excel at different tasks:

| Model Type | Best For |
|------------|----------|
| Vision models | Raw grid pattern analysis |
| Reasoning models | Game mechanics inference |
| Fast models | Action execution |
| Large models | Strategy planning |

---

## Conclusion

The semiotic framework provides the right conceptual foundation:
- Games are reality simulations
- Visual patterns are evolved + conditioned
- Mechanics must be discovered, not assumed

The bottleneck is **execution fidelity** - getting models to consistently output valid actions based on correct reasoning.

---

*Compiled from ARC-AGI-3 research sessions*
