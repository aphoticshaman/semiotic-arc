# Anti-Cheat Failure Modes by Game Genre (2025)

## Executive Summary

Anti-cheat systems operate in a perpetual arms race. This document catalogs systemic failure modes across the top 10 game genres, identifying patterns useful for building human-like game agents for research purposes.

---

## Universal Failure Modes (Cross-Genre)

### 1. Kernel vs User-Mode Gap
- **User-mode anti-cheat** (VAC, older systems): Can't see kernel-level activity
- **Kernel-mode anti-cheat** (Vanguard, BattlEye, EAC): Introduces system instability, compatibility issues

### 2. DMA Hardware Cheats
Direct Memory Access cards read game memory from a second PC via PCIe. No code runs on gaming PC.
- **Detection Status**: Increasingly detected via PCIe slot dumps, transfer speed monitoring
- **Failure Mode**: Only effective against systems actively scanning PCIe topology at launch

### 3. BYOVD (Bring Your Own Vulnerable Driver)
Loading legitimate but vulnerable signed drivers to gain kernel access.
- **700+ exploitable drivers** catalogued in LOLDrivers project
- **Detection Gap**: Signed drivers are trusted by default

### 4. External/Hardware Cheats
- Mouse/keyboard firmware modifications
- Screen capture + CV on separate device
- Hardware input injection
- **Failure Mode**: No software can detect hardware modifications reliably

### 5. Behavioral Mimicry
Modern approaches incorporate:
- Pseudorandom delays
- Human-like aim acceleration curves
- Intentional misses
- **Detection Gap**: Hard to distinguish from skilled players

---

## Genre-Specific Analysis

### 1. FPS/Shooter

| Anti-Cheat Type | Key Failure Modes |
|-----------------|-------------------|
| Kernel-mode (Vanguard) | Driver conflicts, DMA detection bypassable |
| User-mode (VAC) | External tools invisible, hours-to-bypass after updates |

**Key Insight**: Non-kernel anti-cheats are structurally inferior. Kernel anti-cheats trade security for stability.

---

### 2. Battle Royale

| Challenge | Notes |
|-----------|-------|
| Scale | Massive player bases make manual review impossible |
| False positives | Automation has high error rates |
| DMA | Actively targeted but not eliminated |

**Key Insight**: Automation has high false positive rates at scale.

---

### 3. MOBA

| Approach | Status |
|----------|--------|
| Kernel AC | Effective but not omniscient |
| Scripts | Pseudorandom delays evade behavioral detection |
| Hardware mods | Completely bypass software detection |

**Key Insight**: Hardware and external approaches remain blind spots.

---

### 4. MMO

| Detection Focus | Evasion |
|-----------------|---------|
| Play patterns | Random delays, imperfect pathing |
| Superhuman reactions | Gaussian noise on timings |
| 24/7 uptime | Scheduled breaks, varied sessions |

**Key Insight**: MMO anti-cheat is behavioral-first. Pattern disruption is the primary evasion vector.

---

### 5. Turn-Based (Strategy, Card Games)

| Approach | Notes |
|----------|-------|
| Server-authoritative | Client-side less effective |
| Bot detection | Relies on temporal patterns |
| Pattern focus | Inhuman reaction times, perfect sequencing |

**Key Insight**: Server-side validation makes client-side manipulation less effective.

---

## Detection Vector Summary

| Detection Vector | Evasion Approach | Effectiveness |
|------------------|------------------|---------------|
| Memory scanning | External hardware | High |
| Process injection | External tools | High |
| Signature matching | Obfuscation | Medium |
| Behavioral analysis | Humanization | Medium-High |
| Hardware fingerprinting | ID spoofing | Medium |

---

## Principles for Human-Like Game Agents

### DO:
1. **Timing variance**: Add gaussian noise to all action timings (σ = 50-200ms)
2. **Imperfect pathing**: Path efficiency ~0.3-0.4, not 0.9+
3. **Mouse dynamics**: Neuromotor patterns (acceleration/deceleration curves)
4. **Session variance**: Randomize play duration, take breaks
5. **Intentional errors**: Occasional misclicks, suboptimal plays

### DON'T:
1. Fixed angles (0, ±π/2, ±π are tells)
2. Consistent timing between actions
3. Direct A→B pathing
4. 24/7 uptime
5. Perfect play (suspiciously good is detectable)

---

## Sources

### Academic/Research
- [AntiCheatPT: Transformer-Based Detection](https://arxiv.org/html/2508.06348v1)
- [Deep Learning Anti-cheat for Minecraft](https://link.springer.com/chapter/10.1007/978-3-031-81713-7_17)
- [If It Looks Like a Rootkit: Critical Examination of Kernel Anti-Cheat](https://dl.acm.org/doi/fullHtml/10.1145/3664476.3670433)

### Industry
- [Fortnite Anti-Cheat Update Feb 2025](https://www.fortnite.com/news/fortnite-anti-cheat-update-february-27-2025)
- [Riot Vanguard x LoL Retrospective](https://www.leagueoflegends.com/en-us/news/dev/dev-vanguard-x-lol-retrospective/)
- [RICOCHET Anti-Cheat Season 01](https://www.callofduty.com/blog/2025/12/call-of-duty-ricochet-anti-cheat-update-season-01)

---

*Compiled for ARC-AGI research on human-like agent behavior*
