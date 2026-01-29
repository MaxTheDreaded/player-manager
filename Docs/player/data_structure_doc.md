# ⚽ PLAYER DATA STRUCTURE — FINAL SPECIFICATION

This structure represents the core persistent entity of the game.
All systems (training, matches, transfers, morale, events, aging) read from and modify this model.

It must be:
- Persistent across seasons
- Modular and extensible
- Separated into visible vs hidden systems

## 1. PLAYER ROOT OBJECT
```text
Player
├── Identity
├── Career
├── Attributes
├── HiddenAttributes
├── PhysicalState
├── FormAndPerformance
├── PersonalityAndMorale
├── Relationships
├── Contract
├── Reputation
├── Traits
├── History
└── Metadata
```

## 2. IDENTITY

Basic personal information. Mostly static.

| Field | Type | Notes |
|---|---|---|
| PlayerID | Unique ID | Persistent identifier |
| FirstName | String | |
| LastName | String | |
| DateOfBirth | Date | Used to calculate age |
| Nationality | String | |
| HeightCm | Int | Affects aerial ability |
| WeightKg | Int | Affects physical duels |
| PreferredFoot | Enum (Left/Right/Both) | |
| PrimaryPosition | Enum | e.g., ST, CM, CB |
| SecondaryPositions | List<Enum> | |

## 3. CAREER INFO

Tracks the player’s professional journey.

| Field | Type | Description |
|---|---|---|
| CurrentClubID | ID | Reference to club |
| CareerStartYear | Int | First pro year |
| Retired | Boolean | Career ended |
| YearsPro | Int | Auto-calculated |
| SquadRole | Enum | Prospect / Rotation / Starter / Star |

## 4. VISIBLE ATTRIBUTES (0–100 SCALE)

These define footballing ability and are shown to the player.

### TECHNICAL

| Attribute | Description |
|---|---|
| Dribbling | Ability to beat players with the ball |
| FirstTouch | Quality of receiving the ball |
| Passing | General pass accuracy |
| Crossing | Accuracy of wide deliveries |
| Finishing | Scoring ability in box |
| LongShots | Shooting from distance |
| Heading | Aerial ability (attack & defense) |
| Technique | Ability to execute difficult skills |
| Tackling | Ground defensive ability |
| Marking | Ability to track opponents |
| BallControl | Ability to retain ball under pressure |

### MENTAL

| Attribute | Description |
|---|---|
| Decisions | Quality of choices made in play |
| Composure | Performance under pressure |
| Vision | Ability to spot opportunities |
| Anticipation | Reading the game |
| Positioning | Defensive spatial awareness |
| OffTheBall | Attacking movement |
| Teamwork | Cooperation with teammates |
| WorkRate | Effort level |
| Determination | Drive to improve and compete |
| Leadership | Influence on teammates |
| Concentration | Reduces late-match errors |
| Flair | Likelihood of creative/risky play |
| Bravery | Willingness in duels |
| Aggression | Intensity in challenges |

### PHYSICAL

| Attribute | Description |
|---|---|
| Acceleration | Short burst speed |
| Pace | Top sprint speed |
| Agility | Turning ability |
| Balance | Stability in contact |
| Strength | Physical power |
| Jumping | Aerial reach |
| Stamina | In-match endurance |
| NaturalFitness | Between-match recovery speed |
| InjuryResistance | Resistance to injuries |

### GOALKEEPING (ONLY IF GK)

- Reflexes
- Handling
- GKPositioning
- AerialReach
- Distribution
- OneOnOnes
- Communication

## 5. HIDDEN ATTRIBUTES (NOT SHOWN TO PLAYER)

These shape long-term development, personality, and match consistency.

### Development & Performance

| Attribute | Purpose |
|---|---|
| Potential | Maximum attribute ceiling |
| Professionalism | Training growth speed |
| Consistency | Reduces performance variance |
| BigMoment | Boost in high-stakes matches |
| Adaptability | Settling into new environments |

### Physical Risk & Longevity

| Attribute | Purpose |
|---|---|
| InjuryProneness | Injury likelihood |
| RecoveryDiscipline | Fitness recovery rate |
| Durability | Resistance to aging decline |

### Personality & Mentality

| Attribute | Purpose |
|---|---|
| Ambition | Desire for bigger clubs |
| Loyalty | Willingness to stay at club |
| Temperament | Emotional control in matches |
| MediaHandling | Reaction to media events |
| Ego | Reaction to squad competition |
| LeadershipPresence | Off-field influence |

## 6. PHYSICAL STATE (WEEKLY CHANGES)

| Field | Range | Description |
|---|---|---|
| Fitness | 0–100 | Match readiness |
| Fatigue | 0–100 | Injury risk factor |
| Sharpness | 0–100 | Match rhythm |
| Injured | Boolean | Injury status |
| InjuryType | Enum | Type of injury |
| InjuryWeeksRemaining | Int | Recovery time |

## 7. FORM & PERFORMANCE TRACKING

| Field | Description |
|---|---|
| LastMatchRating | Most recent match rating |
| FormRating | Avg rating (last 5 games) |
| SeasonStats | Season totals |
| CareerStats | Lifetime totals |

## 8. PERSONALITY & MORALE

| Field | Range |
|---|---|
| Morale | 0–100 |
| Confidence | 0–100 |
| HappinessAtClub | 0–100 |
| MediaPressure | 0–100 |

## 9. RELATIONSHIPS (−100 to +100)

Values tracked separately for:
- Coach
- Teammates
- Agent
- Fans

## 10. CONTRACT

Includes:
- WagePerWeek
- ContractYearsRemaining
- SquadRole
- ReleaseClause
- PromisedPlayingTime

## 11. REPUTATION

Tracks:
- DomesticReputation
- InternationalReputation
- MarketValue

## 12. TRAITS

Special modifiers earned over time (e.g., Big Game Player, Injury Resistant, Leader).

## 13. HISTORY

- Match history (recent)
- Trophies
- Awards
- Transfers

## 14. METADATA

- LastUpdatedWeek
- DataVersion

## IMPLEMENTATION RULES

- Visible attributes = football skill
- Hidden attributes = personality, growth, mentality
- Attributes must influence event probabilities, not outcomes directly
- Structure must be modular for future web expansion