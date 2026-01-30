use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub id: Uuid,
    pub home_team_id: Uuid,
    pub away_team_id: Uuid,
    pub competition_id: Uuid,
    pub scheduled_time: DateTime<Utc>,
    pub status: MatchStatus,
    pub halftime_score: Option<(u8, u8)>,
    pub fulltime_score: Option<(u8, u8)>,
    pub events: Vec<MatchEvent>,
    pub player_ratings: std::collections::HashMap<Uuid, f32>, // player_id -> rating
    pub lineup: MatchLineup,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchEvent {
    pub id: Uuid,
    pub match_id: Uuid,
    pub minute: u8,           // 0-120
    pub half: MatchHalf,
    pub event_type: EventType,
    pub player_involved: Uuid,
    pub secondary_player: Option<Uuid>,
    pub pitch_zone: PitchZone,
    pub success: bool,
    pub base_impact: f32,
    pub time_multiplier: f32,
    pub position_multiplier: f32,
    pub difficulty_multiplier: f32,
    pub clutch_multiplier: f32,
    pub total_impact_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    // Attacking
    Goal,
    ShotOnTarget,
    ShotOffTarget,
    KeyPass,
    Assist,
    DribbleSuccess,
    DribbleFailed,
    CrossSuccess,
    CrossFailed,
    ThroughBall,
    ChanceCreated,
    PenaltyWon,
    FoulWon,
    
    // Defensive
    TackleWon,
    TackleLost,
    Interception,
    Block,
    Clearance,
    AerialDuelWon,
    AerialDuelLost,
    LastManTackle,
    GoalLineClearance,
    
    // Goalkeeper
    Save,
    ReflexSave,
    OneOnOneSave,
    ClaimCross,
    PunchClear,
    SweeperClearance,
    GoalConceded,
    
    // Transition
    BallRecovery,
    CounterAttackStart,
    CounterAttackInvolvement,
    TurnoverCommitted,
    TurnoverForced,
    
    // Discipline
    FoulCommitted,
    YellowCard,
    SecondYellow,
    RedCard,
    PenaltyConceded,
    
    // Off-ball
    PressSuccess,
    PressBroken,
    OffBallRun,
    SpaceCreated,
    MarkingError,
    TrackingBackStop,
    
    // Additional types for implementation
    PassSuccess,
    PassFailed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MatchStatus {
    Scheduled,
    InProgress,
    Finished,
    Postponed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MatchHalf {
    First,
    Second,
    ExtraTime,
    Penalties,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PitchZone {
    DefensiveThird,
    MiddleThird,
    FinalThird,
    Box,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchLineup {
    pub home_team_formation: Formation,
    pub away_team_formation: Formation,
    pub home_starting_xi: Vec<PlayerInMatch>,
    pub away_starting_xi: Vec<PlayerInMatch>,
    pub home_substitutes: Vec<PlayerInMatch>,
    pub away_substitutes: Vec<PlayerInMatch>,
    pub substitutions_made: Vec<Substitution>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInMatch {
    pub player_id: Uuid,
    pub position: Position,
    pub shirt_number: u8,
    pub rating: Option<f32>,
    pub stats: PlayerMatchStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMatchStats {
    pub goals: u8,
    pub assists: u8,
    pub shots_on_target: u8,
    pub shots_off_target: u8,
    pub tackles_won: u8,
    pub interceptions: u8,
    pub clearances: u8,
    pub saves: Option<u8>, // For goalkeepers
    pub passes_completed: u16,
    pub passes_attempted: u16,
    pub dribbles_successful: u8,
    pub dribbles_attempted: u8,
    pub yellow_cards: u8,
    pub red_cards: u8,
    pub minutes_played: u8,
}

impl Default for PlayerMatchStats {
    fn default() -> Self {
        PlayerMatchStats {
            goals: 0,
            assists: 0,
            shots_on_target: 0,
            shots_off_target: 0,
            tackles_won: 0,
            interceptions: 0,
            clearances: 0,
            saves: None,
            passes_completed: 0,
            passes_attempted: 0,
            dribbles_successful: 0,
            dribbles_attempted: 0,
            yellow_cards: 0,
            red_cards: 0,
            minutes_played: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Substitution {
    pub minute: u8,
    pub player_out: Uuid,
    pub player_in: Uuid,
    pub reason: SubstitutionReason,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubstitutionReason {
    Tactical,
    Injury,
    Fatigue,
    Performance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Formation {
    pub goalkeeper: u8,
    pub defenders: Vec<u8>, // e.g., [2, 1, 2] for 4-4-2
    pub midfielders: Vec<u8>,
    pub forwards: Vec<u8>,
}