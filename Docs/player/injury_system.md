# 🏥 INJURY & FITNESS SYSTEM

This system governs:
- Player fitness levels
- Fatigue accumulation
- Injury risk
- Injury types and recovery
- Long-term physical consequences

**It connects directly to:**
- ⚽ Match Engine
- 🏋️ Training System
- 📈 Player Development
- 😊 Morale & Form

## 1. FITNESS SYSTEM (SHORT-TERM CONDITION)
Fitness represents how physically ready the player is right now.

### Fitness Scale (Hidden Value → Shown as %)
| Fitness % | Meaning |
|---|---|
| 95–100 | Fully fit |
| 85–94 | Slight fatigue |
| 70–84 | Noticeable fatigue |
| 50–69 | Very tired |
| <50 | High injury risk & poor performance |

### How Fitness Changes
| Activity | Fitness Effect |
|---|---|
| Full match | Heavy fatigue |
| Sub appearance | Moderate fatigue |
| Intense training | Small fatigue |
| Light training | Minimal fatigue |
| Rest day | Recovery |
| Injury rehab | Gradual recovery |

### What Fitness Affects
| Area | Effect |
|---|---|
| Match Performance | Lower stamina, fewer involvements |
| Late Match Errors | More likely when tired |
| Injury Risk | Increases sharply under 75% fitness |
| Form Recovery | Slower when fatigued |

> Low fitness does NOT reduce attributes directly — it affects match simulation probabilities.

## 2. INJURY RISK SYSTEM
Every match and training session carries injury risk.

### Base Injury Risk Influenced By
| Factor | Effect |
|---|---|
| Low fitness | Major risk increase |
| High match intensity | Increased risk |
| Poor pitch/weather | Slight risk increase |
| Opposition aggression | Slight risk increase |
| Injury Proneness trait | Personal risk increase |
| Age | Older players higher risk |

**Injury Check Occurs During:**
- High-impact match actions (tackles, sprints)
- Overtraining
- Playing while fatigued

## 3. INJURY TYPES
Injuries are categorized by severity and body area.

### 🟢 Minor Injuries
| Type | Duration |
|---|---|
| Bruise | Few days |
| Muscle tightness | 3–7 days |

**Effect:**
- Slight training limitation
- Small morale drop

### 🟡 Moderate Injuries
| Type | Duration |
|---|---|
| Muscle strain | 2–6 weeks |
| Ankle sprain | 3–8 weeks |

**Effect:**
- No match play
- Fitness drops
- Morale impact

### 🔴 Major Injuries
| Type | Duration |
|---|---|
| Torn ligament | 4–8 months |
| Broken bone | 3–6 months |

**Effect:**
- Long layoff
- Possible permanent physical decline
- Confidence drop after return

## 4. LONG-TERM INJURY CONSEQUENCES
Severe injuries may permanently reduce:

| Attribute Type | Possible Impact |
|---|---|
| Pace | Reduced ceiling |
| Acceleration | Slower recovery |
| Stamina | Lower max |
| Natural Fitness | Slower recovery speed |

> Professionalism and medical staff quality reduce severity of long-term damage.

## 5. PSYCHOLOGICAL IMPACT OF INJURIES
Injuries don’t just affect the body.

| Situation | Confidence | Morale |
|---|---|---|
| Short injury | Small drop | Small drop |
| Long injury | Large drop | Moderate drop |
| Recurring injuries | Big drop | Frustration risk |

> Young players may struggle more mentally after long layoffs.

## 6. RECOVERY PROCESS
Recovery has phases:

| Phase | Description |
|---|---|
| Injured | No training or matches |
| Rehab | Light training only |
| Return to Training | Gradual fitness rebuild |
| Match Sharpness | Needs games to regain form |

### Match Sharpness (Separate from Fitness)
A player returning from injury:
- May have 95% fitness
- But still be "rusty"

**This temporarily reduces:**
- Form
- Involvement frequency

*Sharpness improves with match minutes.*

## 7. TRAINING LOAD & INJURY RISK
Training intensity affects both growth and risk.

| Training Intensity | Growth | Injury Risk |
|---|---|---|
| Light | Slow | Very Low |
| Normal | Balanced | Low |
| Intense | Fast | Medium |
| Very Intense | Very Fast | High |

> Manager personality influences default intensity.

## 8. NATURAL FITNESS ATTRIBUTE
Hidden attribute that affects:
- Fitness recovery speed
- Fatigue resistance
- Career longevity

> *High Natural Fitness = fewer injuries, longer peak.*

## 9. MATCH INJURY EVENTS
During match simulation:

**When a high-risk event occurs:**
1. Injury check triggered
2. Severity determined

**Player may:**
- Continue with reduced performance
- Be forced off
- Play through minor knock (risk worsens)

## 10. AGE & INJURY
| Age | Injury Trend |
|---|---|
| Teen | Fast recovery, lower severe risk |
| 20s | Peak durability |
| 30+ | Slower recovery, higher muscle injuries |
| 34+ | Frequent minor issues |

## 11. DESIGN OUTCOMES

- [x] Rotation and rest matter
- [x] Young players bounce back faster
- [x] Overplaying risks long-term damage
- [x] Big injuries feel dramatic
- [x] Comebacks feel meaningful
- [x] Physical decline feels natural, not scripted

> This system now ties physical realism into: **Training → Matches → Fatigue → Injury → Recovery → Form** making career management deeper and more emotional.