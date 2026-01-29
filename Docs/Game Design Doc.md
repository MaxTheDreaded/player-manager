# ⚽ GAME DESIGN DOCUMENT
**Working Title:** From Boots to Ballon d’Or

**Genre:** Text-Based Football Career Simulation
**Initial Platform:** Console / Terminal
**Future Platform:** Web Application
**Game Length:** Multi-season career (15–20 in-game years)
**Target Audience:** Simulation-focused players, accessible to dedicated casual players

## 1. CORE GAME VISION

The player controls the career of a single footballer, beginning as a teenager and progressing season by season through training, matches, transfers, relationships, and life events. The goal is to become one of the greatest players in the world.

### Design Pillars

- **Long-Term Career Journey** – Development over years, not quick success
- **Meaningful Systems** – Every choice influences growth and opportunities
- **Match Realism Through Events** – Performances driven by contextual match actions
- **Accessible Depth** – Deep mechanics presented through simple menus

## 2. GAME FLOW STRUCTURE

### Time System

- Game progresses in weekly turns
- A season contains:
  - League matches
  - Cup matches
  - International breaks
- Career spans approx. ages 15–35+

## 3. CORE GAMEPLAY LOOP (WEEKLY)

### Status Review
- Fitness
- Morale
- Form
- Upcoming matches

### Player Decisions
- Choose training focus
- Optional extra activities (rest, media, social)
- Conversations (coach, agent, teammates)

### Match Simulation (if scheduled)
- Player match events generated
- Performance rating calculated

### Consequences Applied
- Attribute growth or decline
- Fatigue changes
- Injury checks
- Reputation shifts

### Random/Scripted Events Triggered
- Transfers
- Media
- Personal life
- Team dynamics

## 4. PLAYER PROFILE SYSTEM

### 4.1 Basic Info
- Name
- Age
- Nationality
- Height / Weight
- Preferred Foot
- Primary & Secondary Positions

### 4.2 Attributes (0–100 Scale)

**Technical**
- Dribbling, Passing, Shooting, First Touch, Tackling, Crossing

**Physical**
- Pace, Stamina, Strength, Agility, Jumping

**Mental**
- Composure, Vision, Work Rate, Determination, Positioning, Teamwork

**Hidden Attributes**
- Injury Proneness, Consistency, Big Match Temperament, Professionalism, Potential Ceiling

## 5. PLAYER DEVELOPMENT SYSTEM

### Age Phases

| Age | Development Trend |
|---|---|
| 15–18 | Fast technical growth |
| 19–23 | Strong physical growth |
| 24–28 | Peak performance |
| 29–32 | Slow decline, mental peak |
| 33+ | Physical decline phase |

**Growth depends on:**
- Training focus
- Match performance
- Coach quality
- Determination
- Potential ceiling
- Fatigue & injuries

## 6. TRAINING SYSTEM

Each week player selects one focus:

| Focus | Main Effect | Risk |
|---|---|---|
| Technical | Improves technical stats | Low fatigue |
| Physical | Improves physical stats | Higher fatigue |
| Tactical | Improves positioning & awareness | Medium fatigue |
| Mental | Improves composure & decisions | Low fatigue |
| Rest | Recovers fitness & reduces injury risk | No growth |

> **Note:** Overtraining → Fatigue ↑ → Injury risk ↑ → Match performance ↓

## 7. FITNESS, FATIGUE & FORM

| System | Function |
|---|---|
| Fitness | Short-term readiness (0–100) |
| Fatigue | Accumulates from training/matches |
| Form | Average of last 5 match ratings |
| Sharpness | Reduced when not playing matches |

> Low fitness or high fatigue negatively affects match events.

## 8. INJURY SYSTEM

**Injury risk influenced by:**
- Fatigue
- Injury Proneness
- Training intensity
- Match frequency

**Injury types:**
- Minor (1–2 weeks)
- Moderate (1–2 months)
- Major (6+ months, possible attribute loss)

## 9. MATCH SIMULATION OVERVIEW

Matches are simulated through player-specific match events rather than full team simulation.

**Each match produces:**
- A list of actions performed by the player
- Contextual information (time, scoreline, pressure)
- A final match rating based on those events

## 10. MATCH EVENT–BASED RATING SYSTEM

### Starting Rating
Every player begins a match with a baseline rating of **6.0**

### 10.1 Base Action Values
Each possible match action has a base value representing average importance.

**Positive Examples**

| Action | Base Value |
|---|---|
| Goal | +1.20 |
| Assist | +0.90 |
| Key Pass | +0.45 |
| Shot on Target | +0.25 |
| Successful Dribble | +0.15 |
| Tackle | +0.25 |
| Interception | +0.20 |
| Block | +0.40 |
| Goal-line Clearance | +0.80 |
| Save (GK) | +0.35 |

**Negative Examples**

| Action | Base Value |
|---|---|
| Error Leading to Goal | −1.50 |
| Missed Big Chance | −0.70 |
| Foul in Dangerous Area | −0.40 |
| Yellow Card | −0.30 |
| Red Card | −2.00 |

### 10.2 Context Multiplier

**Score Impact**

| Situation | Multiplier |
|---|---|
| Gives team the lead | ×1.4 |
| Equalizer | ×1.3 |
| Extends lead | ×1.1 |
| Game already decided | ×0.7 |

**Time Impact**

| Time | Multiplier |
|---|---|
| 0–30 min | ×1.0 |
| 31–60 min | ×1.1 |
| 61–85 min | ×1.25 |
| 86+ min | ×1.4 |

### 10.3 Position Responsibility Modifier

Actions are more valuable when performed by a player whose role does not normally prioritize that action.

| Action Type | ST | W | CM | DM | FB | CB | GK |
|---|---|---|---|---|---|---|---|
| Scoring | 1.0 | 1.1 | 1.2 | 1.4 | 1.6 | 1.8 | — |
| Creating | 1.0 | 1.0 | 1.1 | 1.2 | 1.3 | 1.5 | — |
| Defensive Stop | 1.4 | 1.2 | 1.0 | 0.9 | 0.9 | 0.8 | — |

### 10.4 Difficulty Modifier

Harder actions gain bonuses:
- Under pressure ×1.2
- Long range ×1.3
- Weak foot ×1.2
- Last-man tackle ×1.3

### 10.5 Diminishing Returns

Repeated similar actions give less impact:
- **Diminishing Factor** reduces value as action count increases.
- Prevents unrealistic rating inflation.
- Negative mistakes have **lighter diminishing** to ensure errors still hurt.

### 10.6 Final Rating Formula

> **Final Rating** = 6.0 + Sum of (All Event Impacts)
> Clamped between 3.0 and 10.0

*Soft compression above 9.2 ensures 10.0 ratings are extremely rare.*

## 11. MORALE & PERSONALITY

**Morale influenced by:**
- Playing time
- Team performance
- Media
- Contracts
- Relationships

> Low morale reduces performance and development.

## 12. RELATIONSHIP SYSTEM

**Tracked values with:**
- Coach
- Teammates
- Agent
- Fans

> Impacts playtime, transfers, and contracts.

## 13. TRANSFER & CONTRACT SYSTEM

**Clubs show interest based on:**
- Reputation
- Form
- Age
- Performance

**Contracts include:**
- Wage
- Squad role
- Length
- Bonuses

> Squad role affects morale and playing time expectations.

## 14. REPUTATION SYSTEM

| Type | Impact |
|---|---|
| Domestic | Local league movement |
| International | National team & elite clubs |

> Built from performances, awards, and big matches.

## 15. EVENTS SYSTEM

**Random and performance-triggered events:**
- Media praise or criticism
- Position retraining
- Lifestyle choices
- Sponsorship opportunities

> Choices affect morale and hidden attributes.

## 16. NATIONAL TEAM SYSTEM

- Call-ups based on reputation and form.
- International tournaments give large reputation boosts.

## 17. AWARDS & CAREER MILESTONES

**Tracked:**
- Goals
- Assists
- Appearances
- Trophies
- Individual awards

## 18. SAVE SYSTEM

**Must support:**
- Long-term persistence
- Multiple save files
- Storage of all stats, relationships, and hidden traits

## 19. CONSOLE UI PRINCIPLES

- Number-based menus
- Clear weekly dashboard
- No clutter
- Tooltips for stat explanations

## 20. DIFFICULTY MODES

| Mode | Effect |
|---|---|
| Casual | Fewer injuries, slower fatigue |
| Standard | Balanced |
| Hardcore | Faster decline, harsher form effects |

## 21. FUTURE WEB VERSION EXPANSION

- Graphs
- Match commentary feed
- News system
- Visual player profile
- League tables

> Simulation engine remains unchanged.

## 22. IMPLEMENTATION GUIDELINES FOR AI CODING ASSISTANT

- Systems must be modular (training, matches, events separate)
- All ratings must be event-driven, not hardcoded per position
- Use scalable data structures to support web version later
- Keep simulation probability-driven, not scripted outcomes
- Ensure rare elite performances remain rare