# ⭐ PLAYER RATING ENGINE — FULL DESIGN

This system converts MatchEvent data → Player Match Rating (0.0 – 10.0)

**It must:**
- Reward meaningful impact
- Avoid stat padding exploits
- Respect position & role
- Value clutch moments
- Stay realistic (no 10.0 for 1 tap-in)

## 1. RATING ENGINE OVERVIEW
```mermaid
graph TD
    Events[Match Events] --> Processor[Event Impact Processor]
    Processor --> Context[Context Adjustments]
    Context --> Consistency[Consistency & Involvement Layer]
    Consistency --> Penalties[Mistake & Error Penalties]
    Penalties --> Normalization[Normalization Curve]
    Normalization --> Final[Final Player Match Rating (0–10)]
```

## 2. STEP ONE — EVENT IMPACT AGGREGATION
Each MatchEvent already has a `TotalImpactScore`.
We now group them per player.

> **PlayerMatchImpact** = SUM(All Event TotalImpactScores for Player)

But we don’t stop there — because football isn’t just highlight reels.

## 3. STEP TWO — EVENT TYPE WEIGHT BALANCING
Not all actions should scale equally.

| Category | Weight Modifier | Why |
|---|---|---|
| Goals | ×1.4 | High value but not infinite |
| Assists | ×1.2 | Slightly less than goals |
| Key Passes | ×1.0 | Playmaking |
| Defensive Actions | ×1.1 | Often undervalued |
| GK Saves | ×1.3 | Game-changing |
| Turnovers | ×1.2 (negative) | Costly mistakes |
| Fouls in Dangerous Areas | ×1.3 (negative) | High risk |

**This prevents:**
- 1 goal = automatic 9.5
- 40 simple passes = exploit

## 4. STEP THREE — CONTEXT MULTIPLIERS

### ⏱ Time Importance
| Situation | Multiplier |
|---|---|
| Goal after 85’ to equalize | ×1.5 |
| Early goal (≤15’) | ×1.0 |
| Garbage time goal (team +3) | ×0.7 |

### 🎯 Position Expectation Modifier
Players get boosted for unexpected contributions.

| Action | If Out of Role | Multiplier |
|---|---|---|
| Defender scores | Yes | ×1.3 |
| Striker makes goal-line clearance | Yes | ×1.4 |
| GK assist | Yes | ×1.6 |

### 🧠 Match Importance
| Match Type | Multiplier |
|---|---|
| Friendly | ×0.8 |
| League | ×1.0 |
| Cup Knockout | ×1.2 |
| Final | ×1.4 |

## 5. STEP FOUR — INVOLVEMENT & CONSISTENCY SCORE
We now evaluate how active and consistent the player was.

### 📊 Involvement Score
Measures total meaningful actions:

> **InvolvementScore** =
> WeightedTouches
> \+ PressActions
> \+ DefensiveDuels
> \+ OffBallRuns

Players with too little involvement get a ceiling cap:

| Involvement Level | Max Rating Cap |
|---|---|
| Very Low | 6.8 |
| Low | 7.5 |
| Normal | No cap |
| High | Small bonus |

**Prevents:**
⚠️ Player scores 1 goal from 1 touch → unrealistic 9.8

### 📉 Consistency Check
Big swings (amazing + terrible) balance out.

If player has **Many positive AND many negative events** → rating stabilizes toward mid-high instead of extreme.

## 6. STEP FIVE — NEGATIVE EVENT PENALTIES
Mistakes matter. A lot.

| Event | Penalty Strength |
|---|---|
| Error leading to goal | VERY HIGH |
| Missed penalty | HIGH |
| Last man red card | VERY HIGH |
| Bad turnover before goal | HIGH |
| Frequent failed dribbles | MEDIUM |
| Caught offside repeatedly | LOW |

**Penalty formula:**
> NegativeImpact = SUM(NegativeEventScores × MistakeSeverityMultiplier)

*This subtracts from PlayerMatchImpact before final scaling.*

## 7. STEP SIX — MOMENTUM & CLUTCH FACTOR
Some players step up in key moments.

We detect:
- Events when team is losing/drawing late
- High pressure moments (finals, derbies)

> **ClutchScore** = SUM(HighPressureEventImpact × PressureMultiplier)

Adds a small boost that:
- Separates 7.8 from 8.4
- Doesn’t create unrealistic 10s

## 8. STEP SEVEN — RAW PERFORMANCE SCORE
Now we combine everything:

> **RawScore** =
> (AdjustedPositiveImpact
> \+ InvolvementBonus
> \+ ClutchScore)
> − NegativeImpact

This score is usually a wide range like: -5 → +25.
We must normalize it.

## 9. STEP EIGHT — NORMALIZATION CURVE (CRITICAL)
We map RawScore → 0.0–10.0 using a soft performance curve.

### Target Distribution
| Performance | Typical Rating |
|---|---|
| Poor | 5.0 – 5.9 |
| Average | 6.3 – 6.9 |
| Good | 7.0 – 7.6 |
| Very Good | 7.7 – 8.4 |
| Excellent | 8.5 – 9.2 |
| Legendary | 9.3 – 9.8 |

⚠️ 10.0 is almost impossible (hat-trick + dominant all-round game)

### Rating Anchors
| Scenario | Expected Rating |
|---|---|
| Invisible full match | 6.2 |
| Decent, mistake-free | 6.8 |
| 1 goal, quiet otherwise | 7.4 |
| Goal + all-round play | 8.2 |
| Brace + influence | 8.8 |
| Hat-trick + MOTM | 9.5+ |

## 10. STEP NINE — POSITION-SPECIFIC ADJUSTMENTS
Different roles are judged differently.

### 🧤 Goalkeepers
- Saves weighted heavily
- Conceding from unstoppable shots ≠ big penalty
- Distribution contributes slightly

### 🛡 Defenders
- Defensive duels matter more
- Clean sheet bonus (scaled by shots faced)
- Mistakes punished strongly

### 🎯 Midfielders
- Balance of attack + defense
- Key passes highly valued

### ⚡ Forwards
- Goals and shot quality matter most
- Off-ball runs and pressing contribute

## 11. FINAL RATING OUTPUT

> **FinalRating** =
> Clamp( Normalize(RawScore), 4.5, 9.9 )

We clamp to avoid:
- 3.2 ratings (too harsh)
- Too many 10s