// src/entities/event.rs
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub date: NaiveDate,
    pub event_type: crate::entities::EventType, // Use the full path to avoid conflicts
    pub description: String,
    pub affected_entities: Vec<Uuid>, // IDs of entities affected by this event
    pub importance: f32, // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledEvent {
    pub id: Uuid,
    pub scheduled_time: NaiveDate,
    pub event_type: ScheduledEventType,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduledEventType {
    MatchDay,
    TransferWindowStart,
    TransferWindowEnd,
    ContractExpiry,
    YouthIntake,
    InternationalBreak,
    PreseasonStart,
    SeasonEnd,
}