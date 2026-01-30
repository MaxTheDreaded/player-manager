use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{Player, Team, Competition};

/// The main game state that holds all the data for a running game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    /// The player character
    pub player: Player,
    /// The current club the player is at
    pub current_club_id: Uuid,
    /// All clubs in the game world
    pub clubs: Vec<Team>,
    /// Current season information
    pub season: SeasonInfo,
    /// All leagues in the game
    pub leagues: Vec<Competition>,
    /// All competitions (leagues, cups, etc.)
    pub competitions: Vec<Competition>,
    /// Transfer system state
    pub transfer_system: TransferSystemState,
    /// Relationship values with other characters
    pub relationships: std::collections::HashMap<Uuid, f32>,
    /// Narrative flags tracking ongoing storylines
    pub narratives: std::collections::HashMap<String, bool>,
    /// Track which tutorials have been seen
    pub tutorial_state: std::collections::HashMap<String, bool>,
    /// Save version for compatibility
    pub save_version: String,
    /// Current game date
    pub current_date: DateTime<Utc>,
}

impl GameState {
    pub fn new(player: Player, current_club_id: Uuid) -> Self {
        GameState {
            player,
            current_club_id,
            clubs: Vec::new(),
            season: SeasonInfo::new(),
            leagues: Vec::new(),
            competitions: Vec::new(),
            transfer_system: TransferSystemState::new(),
            relationships: std::collections::HashMap::new(),
            narratives: std::collections::HashMap::new(),
            tutorial_state: std::collections::HashMap::new(),
            save_version: "1.0".to_string(),
            current_date: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonInfo {
    pub year: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub current_matchday: u8,
    pub is_active: bool,
    pub transfer_windows: TransferWindows,
}

impl SeasonInfo {
    pub fn new() -> Self {
        SeasonInfo {
            year: "".to_string(),
            start_date: Utc::now(),
            end_date: Utc::now() + chrono::Duration::days(365),
            current_matchday: 1,
            is_active: true,
            transfer_windows: TransferWindows::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferWindows {
    pub summer_start: DateTime<Utc>,
    pub summer_end: DateTime<Utc>,
    pub winter_start: Option<DateTime<Utc>>,
    pub winter_end: Option<DateTime<Utc>>,
}

impl TransferWindows {
    pub fn new() -> Self {
        TransferWindows {
            summer_start: Utc::now(),
            summer_end: Utc::now() + chrono::Duration::days(60),
            winter_start: None,
            winter_end: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferSystemState {
    pub interested_clubs: std::collections::HashMap<Uuid, TransferInterest>,
    pub active_offers: Vec<TransferOffer>,
    pub negotiation_history: Vec<TransferNegotiation>,
}

impl TransferSystemState {
    pub fn new() -> Self {
        TransferSystemState {
            interested_clubs: std::collections::HashMap::new(),
            active_offers: Vec::new(),
            negotiation_history: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferInterest {
    pub club_id: Uuid,
    pub interest_level: InterestLevel,
    pub last_evaluation_date: DateTime<Utc>,
    pub evaluation_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterestLevel {
    Monitoring,
    Scouting,
    Shortlisted,
    PreparingOffer,
    OfficialOffer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOffer {
    pub id: Uuid,
    pub buying_club_id: Uuid,
    pub target_player_id: Uuid,
    pub offered_wage: f32,
    pub contract_length_years: u8,
    pub transfer_fee: Option<f32>,
    pub offer_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferNegotiation {
    pub offer_id: Uuid,
    pub player_response: Option<PlayerResponse>,
    pub agent_response: Option<AgentResponse>,
    pub final_result: Option<NegotiationResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerResponse {
    Interested,
    NotInterested,
    LetAgentHandle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentResponse {
    Accepted,
    Rejected,
    Negotiating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NegotiationResult {
    Accepted,
    Rejected,
    Withdrawn,
}