# UI and Save Systems Documentation

## Overview

The UI and Save systems provide the presentation layer and persistence functionality for the game. These systems handle user interaction and ensure game state can be saved and restored reliably.

## Console UI System

The `ConsoleUI` provides the text-based interface for the game, displaying information and collecting user input.

### Key Responsibilities
- Display game information in a readable format
- Present choices to the user during decision points
- Handle user input and validate selections
- Coordinate with other systems to process decisions
- Provide navigation between different views
- Format complex data for console display

### Main Interface Components

#### Weekly Status Display
- Shows player's current attributes and form
- Displays fitness, morale, and reputation levels
- Shows contract information and squad role
- Lists upcoming matches and fixtures
- Provides quick access to other screens

#### Player Profile View
- Comprehensive display of player attributes
- Technical, physical, and mental attribute breakdown
- Current status indicators (fitness, form, morale)
- Reputation levels and career statistics
- Contract details and relationship information

#### Match Report Display
- Final score and key match events
- Player's individual performance rating
- Statistical breakdown of player's contribution
- Context about match importance and circumstances
- Impact on player's form and reputation

#### League Table Display
- Current standings with points and statistics
- Form indicators for recent performance
- Head-to-head comparison with other teams
- Remaining fixtures and potential outcomes
- Qualification and relegation indicators

### Decision Handling
The UI system manages various types of decisions:

#### Training Focus Selection
- Presents available training options
- Explains potential benefits of each focus
- Processes player's selection
- Communicates choice to Training System

#### Match Day Decisions
- Pre-match preparation choices
- Tactical adjustment options
- Motivation and mindset selection
- In-match substitution requests

#### Transfer Offer Responses
- Displays details of transfer offers
- Shows pros and cons of each option
- Processes player's response
- Initiates negotiation process if needed

#### Contract Negotiation
- Presents contract terms and conditions
- Allows negotiation of key terms
- Processes counter-offers
- Manages agreement or rejection

#### Manager Conversation
- Facilitates communication with manager
- Presents options for discussing role
- Processes feedback and concerns
- Updates relationship values

### Navigation System
- Main menu with access to all game sections
- Context-sensitive options based on game state
- Quick access to frequently used screens
- Consistent navigation patterns throughout

## Save Manager System

The `SaveManager` handles saving and loading game states with support for multiple save slots and version migration.

### Key Responsibilities
- Serialize game state to persistent storage
- Deserialize saved game states
- Manage multiple save slots
- Handle version migration for compatibility
- Validate save file integrity
- Create backup copies of saves

### Save/Load Operations

#### Save Process
1. Serialize game state to JSON format
2. Apply any necessary transformations
3. Write to specified file path
4. Create backup if enabled
5. Update save metadata

#### Load Process
1. Read save file from storage
2. Validate file integrity
3. Deserialize game state
4. Apply version migration if needed
5. Update internal state

### Version Migration
The system handles compatibility across different game versions:

#### Migration Process
- Detect save file version
- Apply appropriate migration steps
- Update to current version format
- Preserve player progress and data
- Handle deprecated fields gracefully

#### Migration Examples
- Adding new fields with default values
- Updating data structure formats
- Renaming or reorganizing fields
- Removing obsolete data

### Save Slot Management
- Multiple save slots for different game states
- Metadata tracking for each save
- Quick save and load functionality
- Auto-save at key game moments
- Save validation and integrity checks

### File Management
- Automatic directory creation
- File locking during operations
- Error handling for disk issues
- Backup and recovery mechanisms

## Integration with Game Systems

### UI-System Communication
The UI system communicates with other systems through:

#### Data Queries
- Request current player information
- Get match results and statistics
- Retrieve league standings and fixtures
- Access relationship and reputation data

#### Action Processing
- Submit training focus selections
- Process transfer offer responses
- Handle contract negotiations
- Initiate match simulations

#### State Updates
- Receive notifications of game state changes
- Update displays based on new information
- Refresh screens when data changes
- Maintain consistency across views

### Save-System Integration
The Save Manager integrates with all systems:

#### Game State Capture
- Serialize all relevant game data
- Include system-specific state information
- Preserve relationships and progress
- Maintain consistency across systems

#### State Restoration
- Restore game state from saved data
- Initialize all systems with saved values
- Validate restored state integrity
- Resume game from saved point

## Design Principles

### Console UI Design

#### Accessibility
- Clear, readable text formatting
- Consistent layout and navigation
- Appropriate contrast and spacing
- Keyboard-based navigation

#### Information Hierarchy
- Most important information prominently displayed
- Detailed information available on demand
- Contextual information when relevant
- Summary views for quick assessment

#### User Experience
- Intuitive navigation and controls
- Clear feedback for user actions
- Helpful prompts and guidance
- Error prevention and recovery

### Save System Design

#### Reliability
- Robust error handling for all operations
- Data integrity validation
- Backup and recovery mechanisms
- Atomic operations to prevent corruption

#### Compatibility
- Version migration support
- Forward and backward compatibility
- Graceful degradation for missing data
- Clear version tracking

#### Performance
- Efficient serialization and deserialization
- Minimal overhead during gameplay
- Fast save/load operations
- Optimized file I/O operations

## Usage Patterns

### UI Interaction Flow
1. Display current game state information
2. Present available actions to user
3. Process user input and validate
4. Communicate decisions to game systems
5. Update display based on results
6. Return to main interface

### Save Operation Flow
1. Prepare game state for saving
2. Serialize to appropriate format
3. Write to storage with error handling
4. Update save metadata
5. Notify user of completion
6. Handle any errors gracefully

### Load Operation Flow
1. Read save file from storage
2. Validate file integrity and format
3. Deserialize game state
4. Apply version migration if needed
5. Initialize all systems with loaded data
6. Resume gameplay from saved state

These systems provide the essential user interaction and persistence functionality that makes the game accessible and enjoyable, while ensuring that players can continue their career progression across multiple sessions.