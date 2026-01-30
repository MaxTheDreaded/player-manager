# Entities Layer Documentation

## Overview

The entities layer contains all the data structures that represent the game world. These structures define the core objects in the game including players, teams, matches, and competitions.

## Player Entity

The `Player` struct is the central entity in the game, representing the player character and all other players in the game world.

### Core Information
- `id`: Unique identifier for the player
- `name`: Player's full name
- `age`: Current age of the player
- `birth_date`: Date of birth
- `nationality`: Player's nationality
- `height/weight`: Physical characteristics
- `preferred_foot`: Dominant foot (Left, Right, Both)
- `primary_position`: Main position on the field
- `secondary_positions`: Additional positions the player can play

### Attributes
The player has three categories of attributes (0-100 scale):

#### Technical Attributes
- `dribbling`: Ability to beat opponents with the ball
- `passing`: Accuracy and vision in passing
- `shooting`: Ability to score goals
- `first_touch`: Control of the ball
- `tackling`: Defensive ability
- `crossing`: Delivery from wide areas

#### Physical Attributes
- `pace`: Speed and acceleration
- `stamina`: Endurance over the full match
- `strength`: Physical power and dueling ability
- `agility`: Ability to change direction quickly
- `jumping`: Aerial ability

#### Mental Attributes
- `composure`: Ability to perform under pressure
- `vision`: Seeing passes and opportunities
- `work_rate`: Effort in pressing and tracking back
- `determination`: Will to succeed
- `positioning`: Being in the right place at the right time
- `teamwork`: Working effectively with teammates

### Hidden Attributes
These affect gameplay but aren't directly visible to the player:
- `injury_proneness`: Likelihood of getting injured
- `consistency`: How consistent the player is
- `big_match_temperament`: Performance in important matches
- `professionalism`: Approach to training and preparation
- `potential_ceiling`: Maximum possible development
- `versatility`: Ability to play multiple positions
- `ambition`: Drive for success
- `loyalty`: Attachment to club
- `ego`: Self-regard and confidence

### Current State
- `fitness`: Short-term readiness (0-100)
- `fatigue`: Accumulated from training/matches (0-100)
- `form`: Average of last 5 match ratings
- `morale`: Mental state affecting performance (0-100)
- `sharpness`: Match-readiness (0-100)

### Reputation
- `local_reputation`: Recognition within their league/country (0-100)
- `international_reputation`: Global recognition (0-100)

### Contract Information
- `contract`: Details about current contract including wage, length, squad role, etc.

### Career Statistics
- `career_stats`: Historical performance data including appearances, goals, assists, etc.

### Relationships
- `relationships`: Map of relationship values with other entities (managers, teammates, etc.)

### Injury Status
- `injury_status`: Current injury information if applicable

## Team Entity

The `Team` struct represents a football club.

### Basic Information
- `id`: Unique identifier
- `name`: Club name
- `reputation`: Overall club reputation (0-100)
- `financial_power`: Economic strength (0-100)

### Operational Characteristics
- `youth_focus`: Emphasis on developing young players (0-100)
- `tactical_identity`: Preferred playing style
- `facilities_quality`: Training facilities quality (0-100)
- `medical_quality`: Medical staff quality (0-100)

### Manager Profile
- `manager_profile`: Information about the manager's preferences and style

### Squad
- `squad`: List of players in the team

### League Information
- `league_id`: ID of the league the team competes in
- `current_season_stats`: Current season performance

## Match Entity

The `Match` struct represents a single football match.

### Basic Information
- `id`: Unique identifier
- `home_team_id`, `away_team_id`: Identifiers for the competing teams
- `competition_id`: Competition this match is part of
- `scheduled_time`: When the match is scheduled
- `status`: Current status (Scheduled, InProgress, Finished, etc.)

### Results
- `halftime_score`: Score at halftime
- `fulltime_score`: Final score
- `events`: List of match events
- `player_ratings`: Ratings for each player
- `lineup`: Starting lineups and substitutions

## Competition Entity

The `Competition` struct represents leagues, cups, and tournaments.

### Basic Information
- `id`: Unique identifier
- `name`: Competition name
- `competition_type`: Type (League, Cup, Continental, etc.)
- `tier_level`: Competitive level (1 = top tier)

### Teams and Season
- `teams`: List of participating teams
- `current_season`: Information about the current season
- `fixtures`: Scheduled matches
- `standings`: Current league table

## Match Event Entity

The `MatchEvent` struct represents individual actions during a match.

### Event Information
- `id`: Unique identifier
- `match_id`: Match this event belongs to
- `minute`: When the event occurred
- `half`: Which half of the match
- `event_type`: Type of action (Goal, Tackle, etc.)
- `player_involved`: Player who performed the action
- `secondary_player`: Other player involved (if applicable)
- `pitch_zone`: Where on the field the event occurred
- `success`: Whether the action was successful

### Impact Calculation
- `base_impact`: Base value of the action
- `time_multiplier`: Impact of when the event occurred
- `position_multiplier`: Impact based on player position
- `difficulty_multiplier`: Impact based on difficulty
- `clutch_multiplier`: Impact based on match importance
- `total_impact_score`: Final calculated impact

## Design Principles

### Immutability and Safety
- Entities are designed to be safely shared between systems
- All attributes have appropriate bounds checking
- Serialization is supported for save/load functionality

### Extensibility
- New attributes can be added without breaking existing code
- Event types can be extended for new game mechanics
- Position types can be expanded for tactical depth

### Performance
- Efficient data structures for quick lookups
- Minimal allocations during gameplay
- Serialization optimized for save/load speed

## Usage Patterns

### Creating Players
Players are typically created during game initialization or when loading from save files. The constructor ensures all attributes are within valid ranges.

### Updating Attributes
Player attributes are updated through the Player Development Engine, which applies appropriate modifiers based on training, matches, and other factors.

### Relationship Management
Relationships are managed through the Social Engine, which updates values based on interactions and personality factors.

### Match Simulation
Matches are simulated by the Match Engine, which generates events and calculates ratings based on player attributes and match context.

This entities layer provides the foundation for all game systems, ensuring consistent data representation across the entire simulation.