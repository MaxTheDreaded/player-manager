# 🎮 CORE GAME LOOP DESIGN
**Text-Based Football Career Simulation**

This document defines how time flows, how the player experiences their career, and how all systems connect into one continuous loop.

## 1. HIGH-LEVEL GAME FLOW

```mermaid
graph TD
    Start[Career Creation] --> Season[Season Loop]
    Season --> Daily[Dynamic Time Simulation (Day-by-Day)]
    Daily --> Events[Events, Matches, Decisions]
    Events --> Review[Season End Review]
    Review --> Transfer[Transfer Window]
    Transfer --> Next[Next Season]
```

> The game is continuous and event-driven, not turn-based.

## 2. CAREER START
**Player Creation**

User selects:
- Name
- Nationality
- Starting age (15–17 typical)
- Preferred position

System generates:
- Visible attributes
- Hidden attributes (personality, potential, etc.)
- Starting club (youth or small professional club)

## 3. SEASON LOOP STRUCTURE
Each season runs as a continuous timeline rather than weekly turns.

> Season Start → Daily Simulation Loop → Matches occur on scheduled days → Season Ends

## 4. DYNAMIC TIME SIMULATION SYSTEM
Time advances automatically in small internal segments within each day.

### Internal Day Segments (Not always shown)

| Segment | Possible Events |
|---|---|
| Morning | Training updates, fitness changes |
| Midday | Manager talks, media stories |
| Afternoon | Transfers, lineup hints |
| Evening | Social interactions, morale updates |

*The system continuously checks for events.*

### CAREER FEED SYSTEM
As time passes, a scrolling feed shows updates:

**Examples:**
- Training improvement reports
- Media reactions
- Team form updates
- Transfer rumors
- League results

> These create immersion without requiring user input.

### USER PAUSE CONTROL
At any moment, the user can pause time and open:

1. Talk to Manager
2. Talk to Teammates
3. Review Training
4. View Player Profile
5. Check Standings
6. Resume Time

### AUTOMATIC PAUSE EVENTS
Time stops automatically when a decision is required.

**High-Priority Events**
- Match Day
- Transfer Offers
- Contract Negotiations
- Manager Role Talks
- Major Injury

**Medium-Priority Events**
- Teammate interactions
- Media questions
- Training focus conflicts

*Low-priority events stay in the feed without pausing.*

## 5. MATCH DAY FLOW
When a match day event triggers:

### Pre-Match Screen
Displays:
- Opponent
- Team form
- Your status (Starting / Bench / Not Selected)

**Small optional morale choice:**
- Motivate self
- Stay calm

### Match Simulation
The Match Engine runs automatically.

**User sees:**
- Key commentary
- Personal involvement highlights

### Post-Match Review
Shows:
- Match rating
- Key contributions
- Form change
- Morale change
- Local reputation change

> This feeds directly into development and career progression.

## 6. BETWEEN-MATCH LIFE
As time continues:
- Systems updating in background
- Training & attribute growth
- Morale changes
- Team standings
- Player form
- Reputation spread (local → international)

## 7. TRAINING INTERACTION LOOP
Manager assigns training focus automatically.

**If player disagrees:**
- User may request change
- Manager may accept, refuse, or compromise

> Alignment improves growth and morale. Conflict hurts morale.

## 8. TEAMMATE SOCIAL SYSTEM
User can pause time and interact with teammates.

**Interaction Types**
- Ask for advice or mentorship
- Build friendship
- Encourage teammate
- Try to convince teammate to stay
- Negative confrontations (optional)

**Relationships affect:**
- Morale
- Team chemistry
- Persuasion success

## 9. MANAGER & MEDIA INTERACTIONS
**Triggered by:**
- Form streaks
- Role disputes
- Big performances

**Choices influence:**
- Manager relationship
- Playing time
- Morale
- Reputation growth

## 10. REPUTATION UPDATES

**Local Reputation**
- Updates frequently from performances and news.

**International Reputation**
- Grows gradually from sustained local fame and big moments.

> Both influence transfers, awards, and career opportunities.

## 11. SEASON END FLOW
When season concludes:

**Season Summary Screen**
- Appearances, goals, assists
- Average rating
- Team finish
- Awards
- Reputation changes

## 12. TRANSFER WINDOW LOOP
Time continues dynamically, but transfer events are frequent.

**Possible interruptions:**
- Transfer rumors
- Official offers
- Contract negotiations
- Teammate transfer talks

> User decisions shape next season’s club and career path.

## 13. LONG-TERM CAREER LOOP
Over multiple seasons:

| Early Career | Mid Career | Late Career |
|---|---|---|
| Development focus | Peak performance | Managing decline |
| Fewer media events | Global recognition | Legacy & mentoring |
| Small clubs | Big transfers | Leadership role |

*The loop repeats until retirement.*

## 14. CORE DESIGN PRINCIPLES

- [x] Time flows naturally, not in turns
- [x] Player interrupts world when desired
- [x] World interrupts player when necessary
- [x] Matches are highlights, not the only focus
- [x] Relationships and reputation evolve slowly
- [x] Career feels lived-in, not menu-driven