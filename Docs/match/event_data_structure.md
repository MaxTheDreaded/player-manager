# 🎮 MATCH EVENT DATA STRUCTURE — FINAL SPEC

This system represents everything that happens during a match in structured, scorable form.

It powers:
- Match ratings
- Commentary/logs
- Highlights
- Stats tracking
- Player development feedback

Events are atomic football actions that occur in sequence and are influenced by player attributes.

## 1. MATCH EVENT ROOT OBJECT
```text
MatchEvent
├── EventID
├── MatchContext
├── TimeContext
├── EventType
├── PlayersInvolved
├── Location
├── Outcome
├── ImpactScores
├── TacticalContext
└── Metadata
```

## 2. MATCH CONTEXT
Identifies where the event belongs.

| Field | Type | Description |
|---|---|---|
| EventID | Unique ID | Unique per event |
| MatchID | ID | Parent match |
| CompetitionID | ID | League/Cup |
| HomeTeamID | ID | |
| AwayTeamID | ID | |

## 3. TIME CONTEXT

| Field | Type | Description |
|---|---|---|
| Minute | Int (0–120) | Match minute |
| Second | Int (0–59) | Optional |
| Half | Enum | First / Second / ExtraTime / Penalties |
| GameState | Enum | Drawing / Winning / Losing |
| GoalDifference | Int | Team goal difference at moment |

> **Used for impact weighting**
> Example: Late equalizer = higher impact than early goal.

## 4. EVENT TYPE (CORE OF SYSTEM)
Each event must be one of these categories.

### 🟢 ATTACKING EVENTS
- Goal
- ShotOnTarget
- ShotOffTarget
- KeyPass
- Assist
- DribbleSuccess
- DribbleFailed
- CrossSuccess
- CrossFailed
- ThroughBall
- ChanceCreated
- PenaltyWon
- FoulWon

### 🔵 DEFENSIVE EVENTS
- TackleWon
- TackleLost
- Interception
- Block
- Clearance
- AerialDuelWon
- AerialDuelLost
- LastManTackle
- GoalLineClearance

### 🟣 GOALKEEPER EVENTS
- Save
- ReflexSave
- OneOnOneSave
- ClaimCross
- PunchClear
- SweeperClearance
- GoalConceded

### 🟡 TRANSITION EVENTS
- BallRecovery
- CounterAttackStart
- CounterAttackInvolvement
- TurnoverCommitted
- TurnoverForced

### 🔴 DISCIPLINE EVENTS
- FoulCommitted
- YellowCard
- SecondYellow
- RedCard
- PenaltyConceded

### ⚪ OFF-BALL & INTANGIBLE
- PressSuccess
- PressBroken
- OffBallRun
- SpaceCreated
- MarkingError
- TrackingBackStop

> These are crucial for rewarding players who contribute without touching the ball much.

## 5. PLAYERS INVOLVED

| Field | Type | Description |
|---|---|---|
| PrimaryPlayerID | Player | Main actor |
| SecondaryPlayerID | Player | Opponent or assister |
| TertiaryPlayerID | Player | Optional (e.g., pre-assist) |
| TeamID | ID | Team performing action |
| OppositionTeamID | ID | |

## 6. LOCATION

| Field | Type | Description |
|---|---|---|
| PitchZone | Enum | DefensiveThird / MiddleThird / FinalThird / Box |
| XCoordinate | Float (0–1) | Optional for future visuals |
| YCoordinate | Float (0–1) | |

> Location affects difficulty and impact.

## 7. OUTCOME

| Field | Type | Description |
|---|---|---|
| Success | Boolean | Did the action succeed? |
| LedToShot | Boolean | Did it create a shot? |
| LedToGoal | Boolean | Did it result in goal? |
| PossessionRetained | Boolean | |

## 8. IMPACT SCORES (FOR RATINGS ENGINE)
These are not final ratings, just event weights.

| Field | Type | Purpose |
|---|---|---|
| BaseImpact | Float | Raw importance of event type |
| TimeMultiplier | Float | Late-game boosts |
| PositionMultiplier | Float | Out-of-role bonus |
| DifficultyMultiplier | Float | Based on location & pressure |
| ClutchMultiplier | Float | Based on match importance |
| TotalImpactScore | Float | Final event value for rating system |

### HOW IMPACT WORKS (Conceptual)
> **TotalImpactScore** =
> BaseImpact
> × TimeMultiplier
> × PositionMultiplier
> × DifficultyMultiplier
> × ClutchMultiplier

*This prevents even huge moments from instantly creating a “10/10” rating.*

## 9. TACTICAL CONTEXT (OPTIONAL BUT FUTURE-PROOF)

| Field | Description |
|---|---|
| TeamMentality | Defensive / Balanced / Attacking |
| PressureLevel | Low / Medium / High |
| DefensiveLineHeight | Deep / Normal / High |

> Allows smarter simulation later.

## 10. METADATA

| Field | Description |
|---|---|
| Weather | Rain, Dry, Snow |
| AttendancePressure | Crowd impact factor |
| RivalryMatch | Boolean |
| MatchImportance | Friendly / League / Cup / Final |

> Feeds into clutch and pressure calculations.

## ⚙️ SYSTEM RULES FOR DEVELOPERS

1. **Events are sequential**
    - Each match is a timeline of MatchEvent objects
2. **Ratings come AFTER**
    - Events feed the Player Rating Engine, not the other way around
3. **Attributes influence probability, NOT scores**
    - Example: High Finishing → higher chance of Goal event
    - But Goal event impact is calculated independently
4. **Position multipliers reward unusual contributions**
    - Defender scoring → higher multiplier
    - Striker making goal-line clearance → higher multiplier
5. **Not all events are visible**
    - Some (PressSuccess, SpaceCreated) exist only for ratings realism