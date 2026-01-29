# 📈 PLAYER PROGRESSION & ATTRIBUTE GROWTH SYSTEM

This system governs:
- Attribute improvement
- Development speed
- Peaks and decline
- Influence of training, matches, morale, and personality

**It must feel realistic, gradual, and influenced by player choices.**

## 1. CORE DEVELOPMENT CONCEPTS
Every player has:

| Factor | Role |
|---|---|
| Current Ability (CA) | How good the player is now |
| Potential Ability (PA) | Maximum ceiling |
| Development Rate | How fast attributes grow |
| Age Curve | Determines growth vs decline |
| Training Focus | Directs attribute growth |
| Match Experience | Accelerates development |
| Morale & Personality | Modify growth efficiency |

## 2. CURRENT ABILITY (CA)
CA is the sum influence of all visible attributes.
- Increases with development
- Stabilizes during peak years
- Declines with age

> The player never sees CA directly — it’s used internally.

## 3. POTENTIAL ABILITY (PA)
PA is the maximum CA a player can reach.

**Influenced by:**
- Hidden Potential attribute
- Personality traits (Ambition, Professionalism)
- Injuries
- Career environment (club quality, training facilities)

> PA is not a hard wall — but growth slows dramatically near it.

## 4. AGE-BASED DEVELOPMENT CURVE
Different ages have different growth behavior.

| Age | Development Pattern |
|---|---|
| 15–18 | Rapid growth phase |
| 19–23 | Strong but slowing growth |
| 24–27 | Peak years (minimal growth, max performance) |
| 28–31 | Stability, slight physical decline |
| 32+ | Increasing physical decline, mental growth possible |

## 5. TRAINING IMPACT ON GROWTH
Training produces daily micro-growth, applied weekly.

**Growth Formula Concept:**
> Attribute Growth =
> Base Age Growth Rate
> × Training Focus Modifier
> × Personality Modifier
> × Morale Modifier
> × Coaching Quality Modifier

**Training Focus Rules:**
| Scenario | Result |
|---|---|
| Aligned with manager plan | Maximum growth efficiency |
| Player requested change (accepted) | High morale boost + good growth |
| Manager rejects request | Morale drop → slower growth |

## 6. PERSONALITY & HIDDEN ATTRIBUTE EFFECTS
Hidden traits strongly influence development.

| Trait | Effect |
|---|---|
| Professionalism | Boosts training effectiveness |
| Ambition | Pushes growth closer to PA |
| Work Rate | Improves physical & stamina growth |
| Consistency | Reduces performance volatility |
| Injury Proneness | Slows physical development |

> *A lazy but talented player may never reach full PA.*

## 7. MORALE EFFECT ON DEVELOPMENT
Morale acts as a development multiplier.

| Morale State | Growth Effect |
|---|---|
| Very High | +15% growth speed |
| Good | +5% |
| Neutral | Normal |
| Low | -10% |
| Very Low | -20% |

> Morale connects matches, relationships, and training into one loop.

## 8. MATCH EXPERIENCE IMPACT
Playing real matches accelerates development, especially for young players.

| Match Involvement | Growth Bonus |
|---|---|
| Full match | High |
| Sub appearance | Medium |
| Unused sub | Very low |
| Not selected | None |

**Good performances give extra positional attribute growth.**
*Example: Defender high rating → defensive attributes grow faster*

## 9. FORM VS DEVELOPMENT
- **Form** = short-term performance trend
- **Development** = long-term attribute growth

**However:**
- Good form streak → small temporary development boost
- Bad form streak → slows growth slightly

## 10. ATTRIBUTE-SPECIFIC GROWTH RULES
Different attribute types grow differently.

### 🟢 Physical
- Grow fast in youth
- Decline earlier
- Affected by training intensity & injuries

### 🔵 Technical
- Grow steadily with training + match use
- Peak mid-career
- Slower decline

### 🟣 Mental
- Grow slowly but longer
- Can improve even in early 30s
- Influenced by experience & mentorship

## 11. INJURY IMPACT
Injuries can:
- Pause growth temporarily
- Permanently reduce physical ceilings (severe injuries)
- Hurt morale → secondary growth impact

## 12. CLUB ENVIRONMENT EFFECTS
Club factors influence development speed:

| Factor | Effect |
|---|---|
| Training Facilities | Higher cap on growth speed |
| Coaching Quality | Improves training efficiency |
| Playing Time | Critical for youth development |
| Team Reputation | Affects morale & ambition |

## 13. DECLINE SYSTEM (AGING)
Decline is gradual and attribute-specific.

| Attribute Type | Decline Start |
|---|---|
| Pace/Acceleration | Late 20s |
| Stamina | Early 30s |
| Strength | Early 30s |
| Technical | Mid 30s |
| Mental | Very late decline |

> Good professionalism slows decline.

## 14. WEEKLY DEVELOPMENT UPDATE CYCLE
Each in-game week:
1. Apply training growth
2. Apply match experience bonus
3. Apply morale modifier
4. Apply personality modifiers
5. Apply age curve modifier
6. Apply injury penalties (if any)
7. Check if near PA → slow growth

*This produces small but meaningful changes over time.*

## 15. DESIGN OUTCOMES

- [x] Wonderkids grow fast but need game time
- [x] Poor mentality can waste high potential
- [x] Good morale matters long-term
- [x] Training choices feel meaningful
- [x] Career peaks feel natural
- [x] Aging feels realistic, not sudden
- [x] Late bloomers are possible but rare