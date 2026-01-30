use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::entities::{SeasonStats, MatchStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competition {
    pub id: Uuid,
    pub name: String,
    pub competition_type: CompetitionType,
    pub tier_level: u8, // 1 = top tier, higher = lower tier
    pub teams: Vec<Uuid>, // team IDs
    pub current_season: SeasonInfo,
    pub fixtures: Vec<Fixture>,
    pub standings: Vec<Standing>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompetitionType {
    League,
    DomesticCup,
    Continental,
    YouthLeague,
    Knockout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonInfo {
    pub year: String, // e.g., "2023-24"
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub current_matchday: u8,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fixture {
    pub id: Uuid,
    pub home_team_id: Uuid,
    pub away_team_id: Uuid,
    pub matchday: u8,
    pub scheduled_date: chrono::DateTime<chrono::Utc>,
    pub status: MatchStatus,
    pub result: Option<(u8, u8)>, // (home_goals, away_goals)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Standing {
    pub team_id: Uuid,
    pub position: u8,
    pub points: u32,
    pub played: u32,
    pub won: u32,
    pub drawn: u32,
    pub lost: u32,
    pub goals_for: u32,
    pub goals_against: u32,
    pub goal_difference: i32,
    pub form: Vec<FormResult>, // Last 5 matches
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormResult {
    Win,
    Draw,
    Loss,
}