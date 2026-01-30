# Game Systems Documentation

## Overview

The game systems implement the core simulation mechanics that drive the football career experience. Each system handles specific aspects of gameplay while maintaining independence and clear interfaces for integration.

## Player Development Engine

The `PlayerDevelopmentEngine` manages how players grow, decline, and maintain their attributes over time.

### Key Responsibilities
- Apply training effects to player attributes
- Process match performance impact on development
- Manage attribute growth curves based on age phases
- Apply morale effects to development rate
- Handle hidden attribute influences
- Process fatigue and injury effects

### Age-Based Development
The system implements different development patterns for each age phase:
- **15-18 (Young)**: Fast technical growth, high potential
- **19-23 (Physical Peak)**: Strong physical growth, technical refinement
- **24-28 (Peak Performance)**: Maintenance of peak attributes
- **29-32 (Aging Phase)**: Gradual decline, mental peak
- **33+ (Late Career)**: Physical decline, experience-based play

### Training Integration
- Processes training focus effects on attributes
- Applies training effectiveness based on various factors
- Implements diminishing returns to prevent stat padding
- Considers player's determination and professionalism

### Match Performance Impact
- Uses match ratings to influence development
- Applies performance-based modifiers
- Considers consistency and form trends
- Balances positive and negative reinforcement

### Hidden Attribute Influence
- Injury proneness affects development consistency
- Potential ceiling limits maximum growth
- Consistency affects rate of improvement
- Professionalism influences training effectiveness

## Morale Engine

The `MoraleEngine` tracks and updates player morale based on various factors and influences performance consistency.

### Key Responsibilities
- Update player morale based on match performance
- Process playing time effects on morale
- Apply team result impacts
- Handle contract status effects
- Process media attention impacts
- Calculate relationship effects
- Apply time-based drift effects

### Morale Impact Calculations
- **Performance Factor**: Based on match ratings and form
- **Playing Time Factor**: Based on squad role and actual minutes
- **Team Result Factor**: Based on match outcomes
- **Contract Factor**: Based on contract status and negotiations
- **Media Factor**: Based on media attention and player ego
- **Relationship Factor**: Based on relationships with teammates and staff
- **Time Factor**: Morale drifts toward baseline when inactive

### Performance Influence
- Morale affects match performance through modifiers
- High morale improves consistency and development
- Low morale reduces performance and increases injury risk
- Morale influences decision-making in critical situations

## Match Engine

The `MatchEngine` simulates football matches and produces player ratings using event-based calculations.

### Key Responsibilities
- Generate match events based on player attributes and form
- Simulate match flow and timeline
- Calculate player ratings from match events
- Process match context (time, score, importance)
- Handle position-specific event impacts
- Apply difficulty and pressure modifiers

### Event Generation
- Determines which team has possession
- Selects players to participate in actions
- Generates appropriate events based on position
- Considers player attributes and form
- Applies random variation for realism

### Rating Calculation
- Aggregates event impacts for each player
- Applies involvement and consistency checks
- Implements diminishing returns for stat padding
- Uses normalization curves to maintain realistic ratings
- Considers position-specific performance factors

### Context Modifiers
- **Time Importance**: Late-game events have higher impact
- **Score Impact**: Goals in tight games are more valuable
- **Position Responsibility**: Unexpected contributions are rewarded
- **Difficulty**: Harder actions receive bonuses
- **Clutch Factor**: Important moments have extra weight

## Reputation Engine

The `ReputationEngine` manages both local and international reputation systems and their conversion.

### Key Responsibilities
- Update local reputation based on match performance
- Convert local reputation to international reputation
- Handle reputation decay for inactive players
- Calculate transfer interest based on reputation
- Process award and achievement impacts
- Manage reputation spread through leagues

### Dual Reputation System
- **Local Reputation**: Immediate recognition in current league
- **International Reputation**: Global recognition and prestige
- Conversion rate depends on league strength and match importance
- International reputation grows more slowly but lasts longer

### Transfer Impact
- Higher reputation attracts more transfer interest
- International reputation opens doors to bigger clubs
- Reputation affects negotiation leverage
- Awards and achievements provide reputation boosts

### Decay Mechanisms
- International reputation decays if player isn't performing
- Local reputation can decline with poor form
- Inactive players lose reputation over time
- Consistent performance maintains reputation levels

## Social Engine

The `SocialEngine` tracks relationships between players and other entities and influences various game aspects.

### Key Responsibilities
- Track relationship values with managers, teammates, agents
- Process social interactions and their outcomes
- Calculate relationship impact on morale
- Influence playing time decisions
- Affect transfer and contract negotiations
- Process team chemistry effects

### Relationship Types
- **Manager**: Affects playing time and development opportunities
- **Teammates**: Influences team chemistry and morale
- **Agent**: Affects transfer negotiations and career decisions
- **Family**: Influences personal morale and decisions
- **Media**: Affects public perception and pressure
- **Fans**: Influences motivation and club loyalty

### Interaction Processing
- Determines success chance of social interactions
- Processes relationship changes based on personality
- Calculates morale impact from relationships
- Influences decision-making in various situations

## Training System

The `TrainingSystem` manages player training focus and its effects on development.

### Key Responsibilities
- Process manager-assigned training focus
- Compare with player preferred focus
- Generate morale effects based on alignment
- Apply training effects to attributes
- Calculate fatigue from training intensity
- Assess injury risk from training

### Focus Alignment
- **Perfect Alignment**: Maximum development and morale benefits
- **Complementary Focus**: Moderate benefits
- **Misaligned**: Reduced benefits and morale penalties
- **No Preference**: Neutral alignment

### Training Effects
- Different focuses improve different attribute categories
- Effectiveness depends on coach quality and facilities
- Intensity affects both improvement and fatigue
- Professionalism influences training effectiveness

## Competition Engine

The `CompetitionEngine` manages leagues, cups, standings, and schedules.

### Key Responsibilities
- Generate fixtures for competitions
- Update standings after matches
- Handle season progression
- Manage team performance tracking
- Process competition results
- Handle promotion/relegation

### Standings Management
- Points allocation based on match results
- Goal difference and goals scored tiebreakers
- Form tracking for recent performance
- Position assignment based on ranking

### Season Management
- Schedule generation for entire season
- Matchday progression tracking
- Season finale processing
- Award and trophy distribution

## Transfer Engine

The `TransferEngine` manages transfer interest, offers, and negotiations.

### Key Responsibilities
- Evaluate transfer interest based on player performance
- Generate transfer offers for interested clubs
- Process player responses to offers
- Handle contract negotiations
- Calculate transfer fees and wages
- Manage contract renewal offers

### Interest Evaluation
- Based on player attributes and potential
- Considers age and contract situation
- Factors in positional needs
- Accounts for financial capacity
- Includes reputation and form considerations

### Offer Generation
- Calculates appropriate wage offers
- Determines contract length based on age
- Estimates transfer fees based on value
- Considers club reputation and financial power
- Factors in player's career stage

## Integration and Coordination

### System Dependencies
- Player Development depends on Training and Match systems
- Morale integrates with Social and Match systems
- Reputation connects with Match, Competition, and Transfer systems
- Social relationships influence Transfer and Contract decisions

### Data Flow
- Match results update Player Development and Reputation
- Training focus affects Player Development and Morale
- Social interactions influence Morale and Transfer decisions
- Competition results affect Reputation and Team dynamics

### Performance Optimization
- Efficient data structures for quick lookups
- Batch processing where possible
- Caching of frequently accessed calculations
- Lazy evaluation for non-critical updates

These game systems work together to create a rich, interconnected simulation where player decisions and performance have meaningful consequences across multiple aspects of the game, creating an authentic football career experience.