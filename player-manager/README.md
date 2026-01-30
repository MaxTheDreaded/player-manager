# From Boots to Ballon d'Or - Football Career Simulator

A text-based football career simulation game built in Rust, where you control the career of a single footballer from teenage years to becoming one of the greatest players in the world.

## Overview

This game simulates a complete football career with:
- Player development over 15-20 in-game years
- Weekly training and match decisions
- Transfer market and contract negotiations
- Reputation system with local and international recognition
- Match simulation with event-based ratings
- Team relationships and morale management
- Multiple competitions and career milestones

## Architecture

The game follows a modular architecture with clear separation between systems:

- **Entities**: Core data structures (Player, Team, Match, Competition)
- **Core**: Foundational systems (Time Engine, Event Engine, Game State)
- **Systems**: Game mechanics (Development, Morale, Match, Reputation, etc.)
- **UI**: Console interface for user interaction
- **Save**: Persistence system for saving/loading games

## Features

### Player Development
- Attributes grow based on training focus, match performance, and age
- Age phases affect development rates (fast growth for teens, peak in 20s, decline in 30s)
- Hidden attributes influence development and personality

### Match Simulation
- Event-based system where player-specific actions generate ratings
- Context-sensitive calculations (time, score, pressure, position)
- Realistic rating system with diminishing returns

### Reputation System
- Dual reputation: Local (league/country) and International (global)
- Local reputation gradually converts to international
- Reputation affects transfer interest and career opportunities

### Transfer Market
- Clubs scout and show interest based on performance
- Transfer windows with negotiation periods
- Contract negotiations with multiple terms

### Social System
- Relationships with teammates, manager, and staff
- Relationship effects on morale and playing time
- Team chemistry and dynamics

## Getting Started

### Prerequisites
- Rust 2021 edition
- Cargo package manager

### Building
```bash
cargo build
```

### Running
```bash
cargo run
```

### Testing
```bash
cargo test
```

## Project Structure

```
src/
├── entities/          # Core data structures
├── core/             # Foundation systems
├── systems/          # Game mechanics
├── ui/               # User interface
├── save/             # Persistence
└── utils/            # Utility functions
tests/                # Integration tests
docs/                 # Documentation
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License.

## Acknowledgments

This game is inspired by classic football management simulators and aims to provide an engaging single-player career experience with deep simulation mechanics.