use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

use crate::entities::event::ScheduledEvent;
use crate::core::time_engine::EventPriority;

/// The EventEngine is the central nervous system of the game
/// It stores events in priority order, handles interruptions,
/// and delivers events to the appropriate systems
pub struct EventEngine {
    /// Queue of events ordered by priority and timing
    pub event_queue: VecDeque<QueuedEvent>,
    /// Registry of event handlers for different event types
    pub event_handlers: HashMap<String, Box<dyn EventHandler>>,
    /// History of processed events for debugging and replay
    pub event_history: Vec<ProcessedEvent>,
}

impl EventEngine {
    /// Creates a new EventEngine instance
    pub fn new() -> Self {
        EventEngine {
            event_queue: VecDeque::new(),
            event_handlers: HashMap::new(),
            event_history: Vec::new(),
        }
    }

    /// Registers an event handler for a specific event type
    pub fn register_handler(&mut self, event_type: String, handler: Box<dyn EventHandler>) {
        self.event_handlers.insert(event_type, handler);
    }

    /// Adds an event to the queue, maintaining priority order
    pub fn queue_event(&mut self, event: QueuedEvent) {
        // Find the correct position based on priority and time
        let pos = self.event_queue.iter()
            .position(|queued| {
                // Higher priority events come first
                queued.priority > event.priority ||
                // Same priority, earlier time comes first
                (queued.priority == event.priority && queued.timestamp < event.timestamp)
            });

        match pos {
            Some(index) => self.event_queue.insert(index, event),
            None => self.event_queue.push_back(event),
        }
    }

    /// Processes the next event in the queue
    pub fn process_next_event(&mut self) -> Result<Option<EventResult>, EventEngineError> {
        if let Some(queued_event) = self.event_queue.pop_front() {
            // Convert the event type to a string representation
            let event_type = format!("{:?}", queued_event.event.event_type);

            if let Some(handler) = self.event_handlers.get(&event_type) {
                let result = handler.handle(&queued_event.event)?;
                
                // Log the processed event
                self.event_history.push(ProcessedEvent {
                    event_id: queued_event.event.id,
                    processed_at: chrono::Utc::now(),
                    result: result.clone(),
                });
                
                Ok(Some(result))
            } else {
                Err(EventEngineError::NoHandlerFound(event_type))
            }
        } else {
            Ok(None)
        }
    }

    /// Processes all events in the queue
    pub fn process_all_events(&mut self) -> Result<Vec<EventResult>, EventEngineError> {
        let mut results = Vec::new();
        
        while let Some(result) = self.process_next_event()? {
            results.push(result);
        }
        
        Ok(results)
    }

    /// Checks if there are any high-priority events requiring user input
    pub fn has_high_priority_events(&self) -> bool {
        self.event_queue
            .iter()
            .any(|event| event.priority == EventPriority::High)
    }

    /// Gets all events that require user input
    pub fn get_user_input_events(&self) -> Vec<&QueuedEvent> {
        self.event_queue
            .iter()
            .filter(|_| true) // For now, return all events (since requires_user_input doesn't exist)
            .collect()
    }

    /// Clears all events from the queue
    pub fn clear_events(&mut self) {
        self.event_queue.clear();
    }
}

/// Represents an event that has been queued for processing
#[derive(Debug, Clone)]
pub struct QueuedEvent {
    /// The scheduled event
    pub event: ScheduledEvent,
    /// Timestamp when it was queued
    pub timestamp: u64,
    /// Priority of the event
    pub priority: EventPriority,
}

impl QueuedEvent {
    pub fn new(event: ScheduledEvent) -> Self {
        QueuedEvent {
            timestamp: chrono::Utc::now().timestamp() as u64,
            priority: EventPriority::Low, // Default priority since ScheduledEvent doesn't have priority
            event,
        }
    }
}

/// Trait that all event handlers must implement
pub trait EventHandler: Send + Sync {
    /// Handles an event and returns a result
    fn handle(&self, event: &ScheduledEvent) -> Result<EventResult, EventEngineError>;
}

/// Result of processing an event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventResult {
    /// Event was handled successfully
    Handled,
    /// Event requires user input
    NeedsUserInput(UserDecisionRequest),
    /// Event should be deferred to a later time
    Deferred(ScheduledEvent),
    /// Event failed to process
    Failed(String),
}

/// Request for user decision during event processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDecisionRequest {
    /// ID of the event requesting input
    pub event_id: Uuid,
    /// Type of decision required
    pub decision_type: DecisionType,
    /// Options available to the user
    pub options: Vec<DecisionOption>,
    /// Context information for the decision
    pub context: serde_json::Value,
}

/// Types of decisions that can be requested
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionType {
    /// Match day decisions
    MatchDayChoice,
    /// Transfer offer response
    TransferOfferResponse,
    /// Contract negotiation
    ContractNegotiation,
    /// Training focus selection
    TrainingFocusSelection,
    /// Manager conversation
    ManagerConversation,
    /// Media interview
    MediaInterview,
    /// Personal life choice
    PersonalLifeChoice,
}

/// An option in a user decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionOption {
    /// Unique identifier for the option
    pub id: Uuid,
    /// Text description of the option
    pub text: String,
    /// Potential consequences of choosing this option
    pub consequences: Vec<Consequence>,
    /// Required attributes or conditions to select this option
    pub requirements: Vec<Requirement>,
}

/// Potential consequence of a decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consequence {
    /// Type of consequence
    pub consequence_type: ConsequenceType,
    /// Value or impact of the consequence
    pub value: f32,
    /// Duration if applicable
    pub duration: Option<u32>, // in days
}

/// Types of consequences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsequenceType {
    /// Morale change
    MoraleChange,
    /// Reputation change
    ReputationChange,
    /// Attribute improvement
    AttributeImprovement(AttributeType),
    /// Relationship change
    RelationshipChange,
    /// Financial impact
    FinancialImpact,
    /// Playing time impact
    PlayingTimeImpact,
    /// Contract status change
    ContractStatusChange,
}

/// Requirement for selecting an option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Requirement {
    /// Minimum attribute value
    MinAttribute(AttributeType, u8),
    /// Minimum reputation
    MinReputation(f32),
    /// Specific relationship level
    RelationshipLevel(Uuid, f32),
    /// Contract status
    ContractStatus(ContractStatus),
}

/// Contract status types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractStatus {
    Active,
    ExpiringSoon,
    Expired,
    Negotiating,
}

/// Attribute types for requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttributeType {
    Technical(crate::entities::TechnicalAttribute),
    Physical(crate::entities::PhysicalAttribute),
    Mental(crate::entities::MentalAttribute),
}

/// Represents a processed event for history tracking
#[derive(Debug, Clone)]
pub struct ProcessedEvent {
    /// ID of the processed event
    pub event_id: Uuid,
    /// When it was processed
    pub processed_at: chrono::DateTime<chrono::Utc>,
    /// Result of processing
    pub result: EventResult,
}

/// Errors that can occur in the EventEngine
#[derive(Debug, thiserror::Error)]
pub enum EventEngineError {
    #[error("No handler found for event type: {0}")]
    NoHandlerFound(String),
    #[error("Event processing failed: {0}")]
    ProcessingFailed(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

// Mock implementations for event type string conversion
// In a real implementation, ScheduledEventType would implement ToString
impl std::fmt::Display for crate::core::time_engine::ScheduledEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            crate::core::time_engine::ScheduledEventType::MatchDay(_) => write!(f, "MatchDay"),
            crate::core::time_engine::ScheduledEventType::TrainingSession => write!(f, "TrainingSession"),
            crate::core::time_engine::ScheduledEventType::TransferWindowOpen => write!(f, "TransferWindowOpen"),
            crate::core::time_engine::ScheduledEventType::TransferWindowClose => write!(f, "TransferWindowClose"),
            crate::core::time_engine::ScheduledEventType::ContractExpires(_) => write!(f, "ContractExpires"),
            crate::core::time_engine::ScheduledEventType::ContractNegotiation(_) => write!(f, "ContractNegotiation"),
            crate::core::time_engine::ScheduledEventType::ManagerEvaluation => write!(f, "ManagerEvaluation"),
            crate::core::time_engine::ScheduledEventType::SeasonEnd => write!(f, "SeasonEnd"),
            crate::core::time_engine::ScheduledEventType::InternationalBreak => write!(f, "InternationalBreak"),
            crate::core::time_engine::ScheduledEventType::MediaEvent => write!(f, "MediaEvent"),
            crate::core::time_engine::ScheduledEventType::PersonalEvent(_) => write!(f, "PersonalEvent"),
            crate::core::time_engine::ScheduledEventType::RandomEvent => write!(f, "RandomEvent"),
        }
    }
}