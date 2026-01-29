# 🌟 DUAL REPUTATION SYSTEM

Your player now has two interconnected reputation values:

> **Local Reputation** → feeds into → **International Reputation**

This creates a natural “buzz → fame → global stardom” progression.

## 1. LOCAL REPUTATION

Represents how well-known and respected the player is within their league/country.

**Scale:** 0–100

| Range | Status |
|---|---|
| 0–20 | Unknown youth player |
| 21–40 | Emerging prospect |
| 41–60 | Recognized league player |
| 61–80 | League star |
| 81–100 | Domestic icon |

### How Local Reputation Changes

Updated after every match and weekly.

**Match-Based Changes**

| Performance | Local Rep Effect |
|---|---|
| 9.0+ rating | Big boost |
| 8.0–8.9 | Medium boost |
| 7.0–7.9 | Small boost |
| 6.5–6.9 | Neutral |
| 6.0–6.4 | Small drop |
| <6.0 | Medium drop |

**Modifiers:**
- Scoring winning goal → extra boost
- Big match (derby/final) → extra boost
- Being subbed off early due to poor play → extra drop

**Weekly Background Changes**

| Situation | Effect |
|---|---|
| Team winning streak | Gradual increase |
| Benched repeatedly | Slow decline |
| Injury long-term | Gradual decline |
| Media praise | Temporary boost |

**Local reputation also affects:**
- Fan support
- Starting chances
- Local transfer interest

## 2. INTERNATIONAL REPUTATION

Represents global football recognition.

**Scale:** 0–100

| Range | Status |
|---|---|
| 0–20 | Unknown outside league |
| 21–40 | Noticed abroad |
| 41–60 | Recognized talent |
| 61–80 | International star |
| 81–100 | Global superstar |

### How International Reputation Grows

International rep does **NOT** change directly per match.
Instead, it grows from sustained local reputation + major moments.

**Local → International Conversion**

Each week:
> InternationalRepGain = (LocalReputation × LeagueReputationFactor × MediaExposureFactor) / 100

*Higher-tier leagues convert local buzz into global fame faster.*

**Direct International Boosts**

| Event | Effect |
|---|---|
| Continental competition MOTM | Big boost |
| International tournament performance | Big boost |
| Winning major award | Huge boost |
| Transfer to bigger league | Medium boost |
| Highlight goals (long-range, solo) | Small boost |

**International Reputation Decay**

If player:
- Moves to weak league
- Rarely plays
- Has long poor form

*International reputation slowly drops.*

## 3. HOW BOTH SYSTEMS INTERACT

| Scenario | Result |
|---|---|
| Young player dominating small league | Local rep rises fast, international rises slowly |
| Star in top league | Both rise quickly |
| Bench player at big club | International stable, local drops |
| Veteran declining | Local drops first, international lingers |

> This creates realistic fame momentum.

## 4. EFFECTS OF EACH REPUTATION TYPE

**Local Reputation Influences**
- Starting XI chances
- Fan chants & support
- Local media coverage
- Domestic transfer interest

**International Reputation Influences**
- Big club transfer offers
- National team selection
- Award nominations
- Global sponsorship events (future feature)

## 5. DESIGN GOALS FOR CODING ASSISTANT

- [x] Local reputation = short-term, performance-driven
- [x] International reputation = long-term, prestige-driven
- [x] Fame spreads gradually, not instantly
- [x] Decline happens slowly unless dramatic drop