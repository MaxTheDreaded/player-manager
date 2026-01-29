# 🔥 FORM & CONFIDENCE SYSTEM

This system determines how well a player performs right now, independent of long-term ability.

A player with great attributes can still play badly if:
- Out of form
- Low confidence
- Mentally affected

## 1. TWO SEPARATE BUT LINKED STATES
| System | Type | Affects |
|---|---|---|
| Form | Performance trend | Match sharpness & impact frequency |
| Confidence | Psychological belief | Risk-taking & success under pressure |

> They influence each other, but are not the same.

## 2. FORM SYSTEM
Form reflects recent on-pitch performances.

### Form Scale (Hidden Numeric Value)
Displayed to player as:
- 🔥 Excellent Form
- 🙂 Good Form
- 😐 Average Form
- 😟 Poor Form
- ❄️ Very Poor Form

### How Form Changes
| Event | Effect |
|---|---|
| High match rating | Increase form |
| Low match rating | Decrease form |
| Team winning streak | Slight boost |
| Not playing regularly | Gradual decay |
| Returning from injury | Temporary drop |

> Form is a rolling average of last performances with gradual decay over time.

### What Form Affects
Form modifies how often a player influences a match.

**Involvement Modifier =**
> Base Involvement × Form Multiplier

| Form Level | Effect |
|---|---|
| Excellent | +20% more key involvements |
| Good | +10% |
| Average | Normal |
| Poor | -10% |
| Very Poor | -20% |

> *A player in hot form just “keeps being in the right place.”*

## 3. CONFIDENCE SYSTEM
Confidence reflects belief and composure.
It influences success probability of actions, not involvement.

### Confidence Scale
| Level | Description |
|---|---|
| Very High | Fearless, decisive |
| High | Self-assured |
| Normal | Balanced |
| Low | Hesitant |
| Very Low | Nervous, error-prone |

### How Confidence Changes
| Trigger | Effect |
|---|---|
| Scoring goal | Big boost |
| Missing big chance | Drop |
| Praise from manager | Boost |
| Criticism | Drop |
| Good match rating | Small boost |
| Bad match rating | Small drop |
| Media hype | Boost |
| Transfer rejection | Drop |

### What Confidence Affects
Confidence modifies Action Difficulty Outcomes.

**Example:**
- High confidence striker → more likely to score 1v1
- Low confidence defender → more likely to mistime tackles

**Action Success Chance =**
> Base Skill Chance × Confidence Modifier

| Confidence | Modifier |
|---|---|
| Very High | +10% success |
| High | +5% |
| Normal | Normal |
| Low | -5% |
| Very Low | -12% |

## 4. FORM ↔ CONFIDENCE INTERACTION
They influence each other:

| Situation | Result |
|---|---|
| Good form streak | Slowly raises confidence |
| Bad form streak | Slowly lowers confidence |
| Confidence boost event | Can kickstart form recovery |

**But they are not identical:**
- A confident player can still be out of form
- A player in form may still be nervous in big moments

## 5. PERSONALITY INFLUENCE
Hidden traits shape emotional swings.

| Trait | Effect |
|---|---|
| Consistency | Reduces form swings |
| Pressure | Affects confidence in big matches |
| Professionalism | Faster recovery from poor form |
| Temperament | More emotional reactions |

> *Two players with same ratings can react very differently.*

## 6. RECOVERY MECHANICS
Players don’t stay down forever.

| Recovery Source | Effect |
|---|---|
| Good training week | Small form recovery |
| Mentor support | Confidence boost |
| Easy match performance | Confidence rebuild |
| Rest | Helps morale → helps form |

## 7. MATCH PERFORMANCE INTEGRATION
During match simulation:

| System | Influences |
|---|---|
| Form | How often player gets involved |
| Confidence | Whether actions succeed |

**So:**
- Form = frequency
- Confidence = quality

## 8. STREAK MECHANICS
To create emotional momentum:

### Hot Streak
**Multiple good games in a row:**
- Confidence rises faster
- Reputation gains slightly boosted

### Slump
**Multiple bad games:**
- Confidence drops faster
- Manager may question role

## 9. USER VISIBILITY
User doesn’t see numbers. They see:
- “You are in excellent form lately”
- “Your confidence is low after recent performances”

> *This keeps immersion high.*

## 10. DESIGN OUTCOMES

- [x] Players feel streaky like real life
- [x] Psychology matters without micromanagement
- [x] Young players are more volatile
- [x] Mentorship and morale feel meaningful
- [x] Comeback stories are possible
- [x] Slumps feel tense but recoverable

> This system now connects **Matches → Form → Confidence → Future Matches** making performances feel alive, not static.