# Core Systems Documentation

## Overview

The core systems provide the foundational functionality for the game, managing time flow, event processing, and game state. These systems operate independently of the game-specific mechanics and provide the infrastructure for all other systems to interact.

## Time Engine

The `TimeEngine` controls the flow of time in the game world and manages the scheduling and execution of events.

### Key Responsibilities
- Advance time in small segments (e.g., hourly ticks)
- Trigger scheduled events when their time arrives
- Check for random events that may occur
- Pause the game when user input is required
- Maintain synchronization between game time and real-world time

### Core Components

#### Time Advancement
- `tick_duration`: Duration of each time segment (default: 1 hour)
- `current_date`: Current in-game date and time
- `advance_time()`: Method to advance time by one tick

#### Event Queue
- `event_queue`: Priority queue of scheduled events
- `schedule_event()`: Add an event to the queue
- `process_scheduled_events()`: Process all events that should occur at the current time

#### Pause System
- `is_paused`: Flag indicating if the game is paused
- `pause_reason`: Reason for the current pause
- `pause_game()`/`resume_game()`: Methods to control game flow

### Event Priorities
Events are categorized by priority:
- **High**: Interrupt gameplay, require immediate user input
- **Medium**: May interrupt based on game settings
- **Low**: Appear in notification feed without interrupting

### Time Management
The Time Engine handles various time-related aspects:
- Weekly training cycles
- Match scheduling
- Transfer window periods
- Contract expiration dates
- Season progression

## Event Engine

The `EventEngine` serves as the central nervous system of the game, managing all events and their processing.

### Key Responsibilities
- Store events in priority order
- Interrupt time when required
- Deliver events to appropriate systems
- Handle user input requests
- Maintain event history for debugging

### Core Components

#### Event Queue
- `event_queue`: Binary heap of events ordered by priority and timing
- `queue_event()`: Add an event to the queue
- `process_next_event()`: Process the next event in the queue

#### Event Handlers
- `event_handlers`: Registry mapping event types to handler functions
- `register_handler()`: Register a handler for a specific event type
- `handle_event_priority()`: Process events based on their priority

#### Event Processing
- `process_all_events()`: Process all events in the queue
- `has_high_priority_events()`: Check for events requiring user input
- `get_user_input_events()`: Retrieve events that need user decisions

### Event Types
The system handles various event types:
- Match events (match day, lineup decisions)
- Career events (transfer offers, contract renewals)
- Social events (teammate conversations, manager meetings)
- Manager events (training changes, tactical discussions)
- Media events (interviews, rumors)
- Personal events (injuries, family matters)

## Game State

The `GameState` holds all the data for a running game and serves as the central repository for game information.

### Core Components

#### Player Character
- `player`: The main player character with all their attributes and information
- `current_club_id`: ID of the club the player currently plays for

#### Game World
- `clubs`: List of all clubs in the game world
- `season`: Current season information
- `leagues`: All leagues in the game
- `competitions`: All competitions (leagues, cups, etc.)

#### Systems State
- `transfer_system`: State of the transfer market
- `relationships`: Relationship values with other characters
- `narratives`: Narrative flags tracking ongoing storylines

#### Technical Information
- `save_version`: Version of the save format
- `current_date`: Current in-game date

### State Management
The GameState provides methods for:
- Saving and loading game data
- Updating game world information
- Managing relationships and narratives
- Handling version migration

## Integration Between Core Systems

### Time-Event Integration
The Time Engine and Event Engine work closely together:
- Time Engine advances time and triggers scheduled events
- Event Engine processes events and may pause time for user input
- Events can schedule future events through the Time Engine
- Both systems maintain synchronization

### Event-Game State Integration
- Events modify the GameState when processed
- GameState provides context for event processing
- Event results update GameState information
- Save/load operations involve both systems

## Design Principles

### Modularity
- Each core system operates independently
- Clear interfaces between systems
- Easy to extend or modify individual components
- Minimal coupling between systems

### Robustness
- Proper error handling for all operations
- Validation of input data
- Recovery mechanisms for system failures
- Consistent state management

### Performance
- Efficient data structures for time operations
- Optimized event processing
- Minimal allocations during gameplay
- Fast lookup operations

## Usage Patterns

### Game Loop Integration
The core systems integrate with the main game loop:
1. Time Engine advances time
2. Events are processed by Event Engine
3. GameState is updated based on events
4. UI presents information to the player
5. User input is processed
6. Loop repeats

### Event Scheduling
Systems schedule events using the Time Engine:
- Match events scheduled for match days
- Training events scheduled weekly
- Transfer events scheduled during windows
- Contract events scheduled for expiration dates

### State Persistence
The GameState is saved and loaded using the Save Manager:
- All game data serialized to JSON
- Version migration for compatibility
- Multiple save slots supported
- Auto-save functionality

These core systems provide the reliable foundation upon which all game-specific mechanics are built, ensuring smooth gameplay and proper state management throughout the player's career.