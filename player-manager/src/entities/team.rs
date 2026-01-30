use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{Player, SeasonStats};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub reputation: f32,          // 0-100
    pub financial_power: f32,     // 0-100
    pub youth_focus: f32,         // 0-100
    pub tactical_identity: TacticalStyle,
    pub facilities_quality: f32,  // 0-100
    pub medical_quality: f32,     // 0-100
    pub manager_profile: ManagerProfile,
    pub squad: Vec<Player>,
    pub league_id: Uuid,
    pub current_season_stats: SeasonStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagerProfile {
    pub tactical_flexibility: f32,  // 0-100
    pub youth_trust: f32,           // 0-100
    pub favoritism: f32,            // 0-100
    pub discipline: f32,             // 0-100
    pub attacking_bias: f32,        // 0-100
    pub rotation_tendency: f32,     // 0-100
    pub trust_ratings: std::collections::HashMap<Uuid, f32>, // player_id -> trust
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TacticalStyle {
    Defensive,
    Balanced,
    Attacking,
    CounterAttacking,
    Possession,
    Pressing,
}