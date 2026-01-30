use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;
use uuid::Uuid;

/// The TimeEngine controls the flow of time in the game world
/// It advances time in small segments, triggers scheduled events,
/// checks for random events, and pauses when user input is required
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEngine {
    /// Current in-game date and time
    pub current_date: DateTime<Utc>,
    /// Duration of each time tick (e.g., 1 hour)
    pub tick_duration: Duration,
    /// Queue of scheduled events ordered by priority and time
    pub event_queue: BinaryHeap<ScheduledEvent>,
    /// Whether the game is paused awaiting user input
    pub is_paused: bool,
    /// Reason for the pause
    pub pause_reason: Option<PauseReason>,
}

impl TimeEngine {
    /// Creates a new TimeEngine instance
    pub fn new(start_date: DateTime<Utc>) -> Self {
        TimeEngine {
            current_date: start_date,
            tick_duration: Duration::hours(1), // Default to 1-hour ticks
            event_queue: BinaryHeap::new(),
            is_paused: false,
            pause_reason: None,
        }
    }

    /// Advances time by one tick duration
    pub fn advance_time(&mut self) -> Result<(), TimeEngineError> {
        if self.is_paused {
            return Err(TimeEngineError::Paused);
        }

        // Advance the current date
        self.current_date = self.current_date + self.tick_duration;

        // Check for any events that should now be triggered
        self.process_scheduled_events();

        Ok(())
    }

    /// Adds a new event to the queue
    pub fn schedule_event(&mut self, event: ScheduledEvent) {
        self.event_queue.push(event);
    }

    /// Processes all events that are scheduled for the current time
    fn process_scheduled_events(&mut self) {
        let mut events_to_process = Vec::new();
        let current_time = self.current_date;

        // Collect all events that should happen at or before current time
        while let Some(event) = self.event_queue.peek() {
            if event.scheduled_time <= current_time {
                if let Some(event) = self.event_queue.pop() {
                    events_to_process.push(event);
                }
            } else {
                break;
            }
        }

        // Process the collected events
        for event in events_to_process {
            self.handle_event_priority(&event);
        }
    }

    /// Checks if there are any high-priority events requiring user input
    fn handle_event_priority(&mut self, event: &ScheduledEvent) {
        if event.requires_user_input {
            match event.priority {
                EventPriority::High => {
                    self.pause_game(PauseReason::HighPriorityEvent(event.clone()));
                }
                EventPriority::Medium => {
                    // Medium priority events might interrupt based on game settings
                    // For now, we'll treat them as notifications
                }
                EventPriority::Low => {
                    // Low priority events are just notifications in the feed
                }
            }
        }
    }

    /// Pauses the game and sets the reason
    fn pause_game(&mut self, reason: PauseReason) {
        self.is_paused = true;
        self.pause_reason = Some(reason);
    }

    /// Resumes the game after user input
    pub fn resume_game(&mut self) {
        self.is_paused = false;
        self.pause_reason = None;
    }

    /// Advances time until the next scheduled event
    pub fn advance_to_next_event(&mut self) -> Result<(), TimeEngineError> {
        if self.is_paused {
            return Err(TimeEngineError::Paused);
        }

        if let Some(next_event) = self.event_queue.peek() {
            let _time_to_advance = next_event.scheduled_time - self.current_date;
            self.current_date = next_event.scheduled_time;
            self.process_scheduled_events();
            Ok(())
        } else {
            Err(TimeEngineError::NoEventsScheduled)
        }
    }

    /// Gets the time until the next scheduled event
    pub fn time_until_next_event(&self) -> Option<Duration> {
        self.event_queue.peek().map(|event| {
            if event.scheduled_time > self.current_date {
                event.scheduled_time - self.current_date
            } else {
                Duration::zero()
            }
        })
    }

    /// Checks if a specific time has passed
    pub fn has_time_passed(&self, target_time: DateTime<Utc>) -> bool {
        self.current_date >= target_time
    }
}

/// Represents a scheduled event in the game
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScheduledEvent {
    /// Unique identifier for the event
    pub id: Uuid,
    /// When the event should occur
    pub scheduled_time: DateTime<Utc>,
    /// Type of event
    pub event_type: ScheduledEventType,
    /// Priority level (determines interruption behavior)
    pub priority: EventPriority,
    /// Whether this event requires user input
    pub requires_user_input: bool,
    /// Optional data payload for the event
    pub data: Option<serde_json::Value>,
}

impl ScheduledEvent {
    pub fn new(
        scheduled_time: DateTime<Utc>,
        event_type: ScheduledEventType,
        priority: EventPriority,
        requires_user_input: bool,
    ) -> Self {
        ScheduledEvent {
            id: Uuid::new_v4(),
            scheduled_time,
            event_type,
            priority,
            requires_user_input,
            data: None,
        }
    }
}

/// Compare events by scheduled time and priority for the heap
impl Eq for ScheduledEvent {}

impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // First compare by scheduled time (earlier events first)
        self.scheduled_time
            .cmp(&other.scheduled_time)
            .then_with(|| {
                // Then by priority (higher priority first)
                other.priority.cmp(&self.priority)
            })
    }
}

/// Different types of scheduled events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScheduledEventType {
    /// Match day event
    MatchDay(Uuid), // Match ID
    /// Training session
    TrainingSession,
    /// Transfer window opens
    TransferWindowOpen,
    /// Transfer window closes
    TransferWindowClose,
    /// Contract expires
    ContractExpires(Uuid), // Player ID
    /// Contract negotiation
    ContractNegotiation(Uuid), // Player ID
    /// Manager evaluation
    ManagerEvaluation,
    /// Season ends
    SeasonEnd,
    /// International break
    InternationalBreak,
    /// Media event
    MediaEvent,
    /// Personal event (injury, family, etc.)
    PersonalEvent(Uuid), // Player ID
    /// Random event
    RandomEvent,
}

/// Priority levels for events
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    /// High priority events that interrupt gameplay
    High,
    /// Medium priority events that may interrupt
    Medium,
    /// Low priority events that just appear in the feed
    Low,
}

/// Reasons why the game might be paused
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PauseReason {
    /// High priority event requiring user input
    HighPriorityEvent(ScheduledEvent),
    /// User requested pause
    UserRequested,
    /// Match day
    MatchDay,
    /// Transfer offer
    TransferOffer,
    /// Contract negotiation
    ContractNegotiation,
    /// Manager conversation
    ManagerConversation,
}

/// Errors that can occur in the TimeEngine
#[derive(Debug, thiserror::Error)]
pub enum TimeEngineError {
    #[error("Game is currently paused")]
    Paused,
    #[error("No events are currently scheduled")]
    NoEventsScheduled,
}