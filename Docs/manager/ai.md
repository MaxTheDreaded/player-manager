# 🧠 MANAGER AI & SQUAD ROLE SYSTEM

Managers are not static. They:
- Choose lineups
- Set expectations
- Adjust tactics when results dip
- React to player performances
- Influence development and morale

## 1. MANAGER PROFILE
Each manager has a hidden profile that shapes decisions.

| Trait | Effect |
|---|---|
| Tactical Flexibility | Likelihood of changing formation |
| Youth Trust | Willingness to play young players |
| Favoritism | Bias toward certain players |
| Discipline | Reaction to poor form or attitude |
| Attacking vs Defensive Bias | Team play style |
| Rotation Tendency | Squad rotation frequency |

## 2. SQUAD ROLE SYSTEM
Every player is assigned a Squad Role that defines expectations.

| Role | Playing Time Expectation |
|---|---|
| Key Player | Must start most matches |
| First Team | Regular starter |
| Rotation | Splits time |
| Backup | Occasional appearances |
| Prospect | Rare appearances, development focus |

### Role Effects
| Area | Impact |
|---|---|
| Morale | Drops if expectations unmet |
| Development | More minutes = faster growth |
| Confidence | Increases with trust |
| Transfer Interest | Low role → higher desire to leave |

## 3. HOW MANAGER PICKS LINEUPS
Before each match, Manager AI evaluates:

| Factor | Weight |
|---|---|
| Player ability | High |
| Form | High |
| Fitness | High |
| Morale | Medium |
| Tactical fit | High |
| Recent performances | Medium |
| Squad rotation need | Variable |

> *A tired player in poor form may lose their spot even if highly rated.*

## 4. TACTICAL SYSTEM
Each manager has:
- Preferred Formation (e.g., 4-3-3)
- Secondary Formation
- Tactical Style (pressing, defensive, counter, possession)

## 5. TACTICAL ADAPTATION MECHANIC
If team underperforms, manager may:

| Trigger | Possible Change |
|---|---|
| Losing streak | Switch to defensive shape |
| Struggling to score | Add extra attacker |
| Leaking goals | Add extra defender |
| Key injury | Change formation |

> *This happens dynamically during the season.*

## 6. IMPACT OF TACTICAL CHANGES ON PLAYER
When tactics change:

### A. Position Fit Check
Player may now be:
- Perfect fit
- Playable
- Out of position

### B. Role Impact
| Situation | Outcome |
|---|---|
| Tactic favors your position | More starts |
| Your role removed (e.g. no wingers) | Reduced minutes |
| You are versatile | Retain importance |
| You can’t adapt | Lose squad status |

### C. Morale Effects
| Event | Morale |
|---|---|
| Losing starting role | Drop |
| Adapting successfully | Boost |
| Playing out of position | Slight drop (unless versatile trait high) |

## 7. PLAYER VERSATILITY SYSTEM
Players have a Positional Familiarity rating.

| Level | Effect |
|---|---|
| Natural | Full effectiveness |
| Competent | Slight penalty |
| Unfamiliar | Significant penalty |

> *Versatility becomes a career survival skill when tactics change.*

## 8. PLAYER–MANAGER INTERACTIONS
When role changes, events may trigger:

| Situation | Interaction Option |
|---|---|
| Dropped from team | Ask manager why |
| Out of position | Request preferred role |
| Good form but benched | Express concern |
| Tactical mismatch | Request role clarification |

**Manager response depends on:**
- Relationship
- Discipline trait
- Your recent form

## 9. MANAGER TRUST SYSTEM
Managers track hidden Trust Rating for each player.

**Trust increases with:**
- Good performances
- Professional behavior
- Training compliance

**Trust decreases with:**
- Poor form
- Complaints
- Low morale attitude

**High trust players:**
- Get more chances
- Survive bad form longer
- May play slightly out of position with less penalty

## 10. LONG-TERM CAREER IMPACT
Manager systems influence:

| Area | Outcome |
|---|---|
| Development | More minutes = faster growth |
| Transfers | Unhappy role → desire to leave |
| Reputation | Being key player raises profile |
| Morale | Stable role improves happiness |

> *A manager change can completely reset your career trajectory.*

## 11. DESIGN OUTCOMES

- [x] Playing time is earned, not guaranteed
- [x] Tactics make the world feel alive
- [x] Versatility becomes strategically valuable
- [x] Manager relationships matter
- [x] Career setbacks feel realistic
- [x] Different managers = different career paths

> This system now links: **Manager AI → Tactics → Playing Time → Morale → Development → Career Direction** making the world feel like it moves with or without the player.