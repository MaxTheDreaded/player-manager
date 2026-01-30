use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub age: u8,
    pub birth_date: NaiveDate,
    pub nationality: String,
    pub height: u16, // in cm
    pub weight: u16, // in kg
    pub preferred_foot: Foot,
    pub primary_position: Position,
    pub secondary_positions: Vec<Position>,
    
    // Attributes (0-100 scale)
    pub technical: TechnicalAttributes,
    pub physical: PhysicalAttributes,
    pub mental: MentalAttributes,
    
    // Hidden attributes
    pub hidden: HiddenAttributes,
    
    // Current state
    pub fitness: f32,        // 0-100
    pub fatigue: f32,        // 0-100
    pub form: f32,           // 0-100 (avg of last 5 match ratings)
    pub morale: f32,         // 0-100
    pub sharpness: f32,      // 0-100
    
    // Reputation
    pub local_reputation: f32,      // 0-100
    pub international_reputation: f32, // 0-100
    
    // Contract info
    pub contract: Contract,
    
    // Career stats
    pub career_stats: CareerStats,
    
    // Relationships
    pub relationships: std::collections::HashMap<Uuid, f32>, // entity_id -> relationship_value
    
    // Injury status
    pub injury_status: Option<Injury>,
    
    // Form history for calculating form
    pub form_history: Vec<f32>,  // Last 5 match ratings for form calculation
    
    /// Track which tutorials have been seen
    #[serde(default)] 
    pub tutorial_state: std::collections::HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalAttributes {
    pub dribbling: u8,
    pub passing: u8,
    pub shooting: u8,
    pub first_touch: u8,
    pub tackling: u8,
    pub crossing: u8,
}

impl TechnicalAttributes {
    pub fn average(&self) -> f32 {
        let sum = self.dribbling as f32 +
                  self.passing as f32 +
                  self.shooting as f32 +
                  self.first_touch as f32 +
                  self.tackling as f32 +
                  self.crossing as f32;
        sum / 6.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalAttributes {
    pub pace: u8,
    pub stamina: u8,
    pub strength: u8,
    pub agility: u8,
    pub jumping: u8,
}

impl PhysicalAttributes {
    pub fn average(&self) -> f32 {
        let sum = self.pace as f32 +
                  self.stamina as f32 +
                  self.strength as f32 +
                  self.agility as f32 +
                  self.jumping as f32;
        sum / 5.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalAttributes {
    pub composure: u8,
    pub vision: u8,
    pub work_rate: u8,
    pub determination: u8,
    pub positioning: u8,
    pub teamwork: u8,
}

impl MentalAttributes {
    pub fn average(&self) -> f32 {
        let sum = self.composure as f32 +
                  self.vision as f32 +
                  self.work_rate as f32 +
                  self.determination as f32 +
                  self.positioning as f32 +
                  self.teamwork as f32;
        sum / 6.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiddenAttributes {
    pub injury_proneness: u8,      // 0-100
    pub consistency: u8,           // 0-100
    pub big_match_temperament: u8, // 0-100
    pub professionalism: u8,       // 0-100
    pub potential_ceiling: u8,     // 0-100
    pub versatility: u8,           // 0-100
    pub ambition: u8,              // 0-100
    pub loyalty: u8,               // 0-100
    pub ego: u8,                   // 0-100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub club_id: Uuid,
    pub wage: f32,
    pub length_years: u8,
    pub squad_role: SquadRole,
    pub release_clause: Option<f32>,
    pub performance_bonuses: Vec<Bonus>,
    pub contract_end_date: NaiveDate,
    pub league_strength: f32, // 0-100, affects reputation conversion
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bonus {
    pub condition: BonusCondition,
    pub amount: f32,
    pub achieved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BonusCondition {
    Goals(u32),
    Appearances(u32),
    CleanSheets(u32),
    LeaguePosition(u8),
    CupWin,
    IndividualAward(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareerStats {
    pub seasons_played: u8,
    pub total_appearances: u32,
    pub total_goals: u32,
    pub total_assists: u32,
    pub total_yellow_cards: u32,
    pub total_red_cards: u32,
    pub average_rating: f32,
    pub highest_rating: f32,
    pub season_stats: Vec<SeasonStats>,
    pub awards: Vec<Award>,
    pub trophies: Vec<Trophy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonStats {
    pub season_year: String, // e.g., "2023-24"
    pub appearances: u32,
    pub goals: u32,
    pub assists: u32,
    pub yellow_cards: u32,
    pub red_cards: u32,
    pub average_rating: f32,
    pub team_finish_position: Option<u8>,
    pub goals_conceded: u32, // For goalkeepers
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Award {
    pub name: String,
    pub season: String,
    pub competition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trophy {
    pub name: String,
    pub season: String,
    pub competition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Injury {
    pub injury_type: InjuryType,
    pub severity: InjurySeverity,
    pub weeks_remaining: u8,
    pub affected_attributes: Vec<AffectedAttribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InjuryType {
    MuscleStrain,
    LigamentSprain,
    Fracture,
    Concussion,
    TornLigament,
    BrokenBone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InjurySeverity {
    Minor,      // 1-2 weeks
    Moderate,   // 3-8 weeks
    Major,      // 9+ weeks
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectedAttribute {
    pub attribute: AttributeType,
    pub reduction_percentage: f32, // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttributeType {
    Technical(TechnicalAttribute),
    Physical(PhysicalAttribute),
    Mental(MentalAttribute),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TechnicalAttribute {
    Dribbling,
    Passing,
    Shooting,
    FirstTouch,
    Tackling,
    Crossing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhysicalAttribute {
    Pace,
    Stamina,
    Strength,
    Agility,
    Jumping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MentalAttribute {
    Composure,
    Vision,
    WorkRate,
    Determination,
    Positioning,
    Teamwork,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Position {
    GK,  // Goalkeeper
    RB,  // Right Back
    CB,  // Center Back
    LB,  // Left Back
    FB,  // Full Back (Right or Left)
    DM,  // Defensive Midfielder
    RM,  // Right Midfield
    CM,  // Center Midfield
    LM,  // Left Midfield
    AM,  // Attacking Midfielder
    RW,  // Right Wing
    LW,  // Left Wing
    CF,  // Center Forward
    SS,  // Secondary Striker
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Foot {
    Right,
    Left,
    Both,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SquadRole {
    KeyPlayer,
    FirstTeam,
    Rotation,
    Backup,
    Prospect,
}

impl SquadRole {
    pub fn ambition_factor(&self) -> f32 {
        match self {
            SquadRole::KeyPlayer => 0.5,   // Already fulfilled ambitions
            SquadRole::FirstTeam => 1.0,   // Satisfied but could want more
            SquadRole::Rotation => 2.0,    // Wants more playing time
            SquadRole::Backup => 3.0,      // Significantly unhappy
            SquadRole::Prospect => 2.5,    // Wants opportunity
        }
    }
}

// Additional missing entities

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub country: String,
    pub city: String,
    pub reputation: f32, // 0-100
    pub finances: Finances,
    pub squad: Vec<Uuid>, // Player IDs
    pub staff: Vec<Uuid>, // Staff member IDs
    pub youth_academy_level: u8, // 1-10
    pub facilities: Facilities,
    pub financial_power: f32, // Added financial power field
    pub youth_focus: f32,     // Added youth focus field
    pub facilities_quality: f32, // Added facilities quality field
    pub medical_quality: f32,    // Added medical quality field
    pub tactical_identity: String, // Added tactical identity field
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finances {
    pub balance: f32,
    pub weekly_wage_bill: f32,
    pub revenue_per_week: f32,
    pub debt: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Facilities {
    pub training_ground_quality: u8, // 1-10
    pub stadium_capacity: u32,
    pub stadium_quality: u8, // 1-10
    pub youth_facilities: u8, // 1-10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competition {
    pub id: Uuid,
    pub name: String,
    pub country: String,
    pub level: u8, // 1 = top tier, higher = lower tier
    pub teams: Vec<Uuid>, // Team IDs
    pub fixtures: Vec<Fixture>,
    pub standings: Vec<Standing>,
    pub competition_type: CompetitionType,
    pub season_start: NaiveDate,
    pub season_end: NaiveDate,
    pub current_season: CurrentSeason, // Added current season field
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompetitionType {
    League,
    Knockout,
    GroupAndKnockout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentSeason {
    pub is_active: bool,
    pub current_matchday: u32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fixture {
    pub id: Uuid,
    pub competition_id: Uuid,
    pub home_team: Uuid,
    pub away_team: Uuid,
    pub scheduled_date: NaiveDate,
    pub venue: Uuid, // Team ID (home team)
    pub status: MatchStatus,
    pub result: Option<MatchResult>,
    pub matchday: u32, // Added matchday field
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Standing {
    pub team_id: Uuid,
    pub position: u8,
    pub played: u8,
    pub won: u8,
    pub drawn: u8,
    pub lost: u8,
    pub goals_for: u32, // Changed to u32 to handle larger numbers
    pub goals_against: u32, // Changed to u32 to handle larger numbers
    pub points: u8,
    pub form: Vec<FormResult>, // Last 5 results
    pub goal_difference: i32, // Added goal difference field
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FormResult {
    Win,
    Draw,
    Loss,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub id: Uuid,
    pub competition_id: Uuid,
    pub home_team: Uuid,
    pub away_team: Uuid,
    pub date: NaiveDate,
    pub venue: Uuid, // Team ID (home team)
    pub status: MatchStatus,
    pub result: Option<MatchResult>,
    pub events: Vec<MatchEvent>,
    pub half_results: Option<(u8, u8)>, // HT score (home, away)
    pub player_ratings: std::collections::HashMap<Uuid, f32>, // Player ID to rating
    pub fulltime_score: Option<(u8, u8)>, // Final score (home, away)
    pub competition_type: CompetitionType, // Added competition type field
    pub lineup: MatchLineup, // Added lineup field
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MatchStatus {
    Scheduled,
    InProgress,
    Finished,
    Postponed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub home_score: u8,
    pub away_score: u8,
    pub winner: Option<Uuid>, // Team ID of winner, None if draw
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchEvent {
    pub event_type: EventType,
    pub minute: u8,
    pub team_id: Uuid,
    pub player_id: Uuid,
    pub description: String,
    pub rating_impact: Option<f32>, // Impact on player rating
    pub id: Uuid, // Added ID field
    pub match_id: Uuid, // Added match ID field
    pub half: MatchHalf, // Added half field
    pub player_involved: Uuid, // Added player involved field
    pub secondary_player: Option<Uuid>, // Added secondary player field
    pub pitch_zone: PitchZone, // Added pitch zone field
    pub total_impact_score: f32, // Added total impact score field
    pub base_impact: f32, // Added base impact field
    pub success: bool, // Added success field
    pub time_multiplier: f32, // Added time multiplier field
    pub position_multiplier: f32, // Added position multiplier field
    pub difficulty_multiplier: f32, // Added difficulty multiplier field
    pub clutch_multiplier: f32, // Added clutch multiplier field
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EventType {
    Goal,
    Assist,
    YellowCard,
    RedCard,
    SubstitutionIn,
    SubstitutionOut,
    Injury,
    Offside,
    PenaltyTaken,
    PenaltySaved,
    PenaltyMissed,
    OwnGoal,
    Save,
    TackleWon,
    TackleLost,
    ChanceCreated,
    ChanceMissed,
    FoulCommitted,
    FoulSuffered,
    SuccessfulDribble,
    UnsuccessfulDribble,
    KeyPass,
    ShotOnTarget,
    ShotOffTarget,
    CrossSuccessful,
    CrossUnsuccessful,
    PassSuccessful,
    PassUnsuccessful,
    AerialDuelWon,
    AerialDuelLost,
    Clearance,
    Interception,
    Block,
    Dispossessed,
    DuelWon,
    DuelLost,
    ClaimCross,
    PunchClear,
    SweeperClearance,
    GoalConceded,
    MissedBigChance,
    PenaltyWon,
    PenaltyConceded,
    DribbleSuccessful,
    ThroughBall,
    ReflexSave,
    OneOnOneSave,
    CrossSuccess,
    PassSuccess,
    DribbleSuccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PitchZone {
    DefensiveThird,
    MiddleThird,
    AttackingThird,
    LeftFlank,
    RightFlank,
    Center,
    FinalThird,
    Box,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchHalf {
    First,
    Second,
    ExtraFirst,
    ExtraSecond,
    Penalties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInMatch {
    pub player_id: Uuid,
    pub team_id: Uuid,
    pub position: Position,
    pub shirt_number: u8,
    pub rating: Option<f32>,
    pub events: Vec<MatchEvent>,
    pub minutes_played: u8,
    pub substitution_minute: Option<u8>,
    pub was_substituted_on: bool,
    pub was_substituted_off: bool,
    pub stats: PlayerMatchStats, // Added stats field
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMatchStats {
    pub tackles: u8,
    pub tackles_won: u8,
    pub interceptions: u8,
    pub passes_completed: u8,
    pub passes_attempted: u8,
    pub shots_on_target: u8,
    pub shots_off_target: u8,
    pub dribbles_successful: u8,
    pub dribbles_attempted: u8,
    pub aerials_won: u8,
    pub aerials_lost: u8,
    pub fouls_committed: u8,
    pub fouls_suffered: u8,
    pub offsides: u8,
    pub clearances: u8,
    pub blocks: u8,
    pub duels_won: u8,
    pub duels_lost: u8,
    pub saves: Option<u8>,
    pub goals: u8,
    pub assists: u8,
    pub yellow_cards: u8,
    pub red_cards: u8,
    pub minutes_played: u8,
    pub possession_time: f32, // Percentage of time with ball
    pub distance_covered: f32, // Meters
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchLineup {
    pub formation: Formation,
    pub players: Vec<PlayerInMatch>,
    pub tactics: Tactics,
    pub home_starting_xi: Vec<Uuid>, // Added home starting XI
    pub away_starting_xi: Vec<Uuid>, // Added away starting XI
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Formation {
    pub goalkeeper: Uuid,
    pub defenders: Vec<Uuid>,
    pub midfielders: Vec<Uuid>,
    pub forwards: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tactics {
    pub style: TacticalStyle,
    pub mentality: f32, // -1.0 (defensive) to 1.0 (attacking)
    pub tempo: f32,     // 0.0 (slow) to 1.0 (fast)
    pub width: f32,     // 0.0 (narrow) to 1.0 (wide)
    pub pressing_intensity: f32, // 0.0 (low) to 1.0 (high)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TacticalStyle {
    Possession,
    CounterAttack,
    HighPress,
    Defensive,
    Attacking,
    Flexible,
    Balanced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub date: NaiveDate,
    pub event_type: EventType,
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

// pub use event::*; // Removed unused re-export

pub mod event;