# 💾 SAVE / LOAD & DATA PERSISTENCE SYSTEM

**Goal:**
Allow a player career to span many seasons, survive app restarts, and later scale into a web or online game without redesigning everything.

> We’ll design this in layers, so you can start simple and grow big.

## 1. CORE SAVE PHILOSOPHY
We don’t just save stats. **We save the world state.**

**A save file must capture:**
- [x] The Player
- [x] The Current Club
- [x] The Season State
- [x] The Football World (clubs, leagues, transfers, narratives)

> *Think of it like a snapshot of an evolving universe.*

## 2. PLAYER SAVE DATA
This is the heart of the career.

### Player Profile
- Name
- Age / DOB
- Nationality
- Preferred foot
- Positions (with proficiency levels)

### Attributes
- Technical attributes
- Physical attributes
- Mental attributes
- Personality traits
- Potential ability (hidden or semi-hidden)

### Development
- Current ability rating
- Growth history (optional but powerful)
- Trait unlocks (e.g. Leadership, Big Game Player)

### Status
- Morale
- Fatigue
- Injury status + recovery timeline
- Match sharpness

### Career Stats
*Saved per season and all-time*
- Appearances
- Goals
- Assists
- Ratings
- Awards
- Trophies

### Contract
- Current club ID
- Squad role
- Wage
- Contract end date
- Promises made by manager

## 3. CLUB SAVE DATA
For every club in the playable world:
- Club ID
- Name
- Reputation level
- Financial strength
- Tactical style
- Manager profile
- Squad list (player IDs or generated squad strength value)
- League position
- Season objectives

> *You do NOT need to fully simulate every player. You can store squad strength metrics for non-user teams to keep it lightweight.*

## 4. WORLD STATE DATA
This makes the game feel alive.

### Season Info
- Current season year
- Current month/week
- Transfer window status (open/closed)
- Competition stage (league matchday, cup round)

### League Tables
For each league:
- Team standings
- Points
- Goals for/against
- Form

### Competition Progress
- Cup brackets
- Continental competitions
- Award races (Top scorer, Best player)

## 5. DYNAMIC SYSTEM STATES
These systems must persist because they evolve:

### Reputation & Fame
- Player reputation score
- Media popularity
- Fan popularity at current club

### Transfer Interest
For each interested club:
- Interest level (monitoring, scouting, shortlisted)
- Last evaluation date

### Relationships
- Manager relationship score
- Teammate chemistry levels
- Agent effectiveness rating

### Narrative Flags
These track career storylines:
- “Big club linked recently”
- “Transfer requested”
- “Returning from long injury”
- “Wonderkid status”

## 6. SAVE FILE STRUCTURE (SIMPLIFIED)
If using JSON or similar:

```json
{
  "player": { ... },
  "currentClubId": 12,
  "clubs": [ ... ],
  "season": { ... },
  "leagues": [ ... ],
  "competitions": [ ... ],
  "transferSystem": { ... },
  "relationships": { ... },
  "narratives": { ... }
}
```

> *Design it modular, so new systems can be added later without breaking old saves.*

## 7. AUTO-SAVE STRATEGY
To prevent loss and allow meaningful decisions:

**Auto-save at:**
- End of each match
- After major career events (transfer, contract signing)
- Season end

**Optional:**
- Manual save slots (like classic career games)

## 8. VERSIONING SYSTEM (VERY IMPORTANT)
As the game grows, the save structure will change.

**Add:**
`"saveVersion": 1.0`

**When loading:**
- If version is older → run a migration step
- *Example: Add new field like player.personalityTraits = [] if missing*

> *This prevents old careers from breaking.*

## 9. PREPARING FOR WEB / ONLINE VERSION
Design now so future multiplayer or cloud saves are easy.

### Use Unique IDs
Every entity should have an ID:
- Player ID
- Club ID
- League ID

> *Never rely on array positions.*

### Separate Systems
Keep logic separated:
- Match engine
- Transfer system
- Media system
- Save system

**So later, saves can move to:**
- Database (SQL / NoSQL)
- Cloud storage
- User accounts

## 10. DATA SIZE OPTIMIZATION
We don’t want bloated saves.

**Instead of storing every simulated match:**
- [x] Store results summaries
- [x] Store season stats
- [x] Store key events only

**For AI leagues:**
- Store standings, not full play-by-play

## 11. DATA INTEGRITY & ANTI-CORRUPTION
**Add:**
- Checksum or hash (optional advanced)
- Backup auto-save slot
- Safe-write system (write new file, then replace old)

> *Prevents career loss from crashes.*

## 12. DESIGN OUTCOMES

- [x] Multi-season careers feel continuous
- [x] Player history matters
- [x] Transfers and relationships persist
- [x] Media narratives evolve
- [x] Game can grow without breaking saves
- [x] Future web version is possible without redesign