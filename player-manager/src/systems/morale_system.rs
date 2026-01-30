use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{Player, SquadRole};

/// The MoraleEngine tracks and updates player morale based on various factors
/// It influences performance consistency, development rate, and social interaction outcomes
pub struct MoraleEngine;

impl MoraleEngine {
    /// Creates a new MoraleEngine instance
    pub fn new() -> Self {
        MoraleEngine
    }

    /// Updates player morale based on various influencing factors
    pub fn update_player_morale(
        &self,
        player: &mut Player,
        match_rating: Option<f32>,           // Rating from last match, if applicable
        playing_time_minutes: Option<u8>,    // Minutes played in last match
        team_result: Option<GameResult>,     // Result of last match
        contract_status: ContractStatus,     // Current contract situation
        media_attention: MediaAttention,     // Level of media attention
        relationship_changes: &[(Uuid, f32)], // Changes in relationships
        days_since_last_match: u32,        // Days since last match
    ) {
        let mut morale_change = 0.0;

        // Apply match performance effect
        if let Some(rating) = match_rating {
            morale_change += self.calculate_match_performance_effect(rating, player.hidden.professionalism);
        }

        // Apply playing time effect
        if let Some(minutes) = playing_time_minutes {
            morale_change += self.calculate_playing_time_effect(minutes, &player.contract.squad_role);
        }

        // Apply team result effect
        if let Some(result) = team_result {
            morale_change += self.calculate_team_result_effect(result);
        }

        // Apply contract status effect
        morale_change += self.calculate_contract_effect(contract_status, &player.contract);

        // Apply media attention effect
        morale_change += self.calculate_media_effect(media_attention, player.hidden.ego);

        // Apply relationship changes
        for (_entity_id, change) in relationship_changes {
            morale_change += self.calculate_relationship_effect(*change, player.hidden.loyalty);
        }

        // Apply time effect (morale drifts toward baseline when not playing regularly)
        if days_since_last_match > 7 {
            morale_change += self.calculate_time_drift_effect(days_since_last_match);
        }

        // Apply morale change with boundaries
        player.morale = (player.morale + morale_change).max(0.0).min(100.0);
    }

    /// Calculates morale change based on match performance
    fn calculate_match_performance_effect(&self, rating: f32, professionalism: u8) -> f32 {
        // Base performance effect
        let base_effect = match rating {
            r if r >= 9.0 => 5.0,   // Excellent performance
            r if r >= 8.0 => 3.0,   // Very good
            r if r >= 7.0 => 1.5,   // Good
            r if r >= 6.0 => 0.5,   // Average
            r if r >= 5.0 => -1.0,  // Poor
            _ => -3.0,               // Terrible
        };

        // Apply professionalism modifier
        // Professional players are less affected by single match results
        let professionalism_modifier = 1.0 - (professionalism as f32 / 200.0);

        base_effect * professionalism_modifier
    }

    /// Calculates morale change based on playing time
    fn calculate_playing_time_effect(&self, minutes: u8, squad_role: &SquadRole) -> f32 {
        let expected_minutes = match squad_role {
            SquadRole::KeyPlayer => 85.0,
            SquadRole::FirstTeam => 70.0,
            SquadRole::Rotation => 45.0,
            SquadRole::Backup => 15.0,
            SquadRole::Prospect => 5.0,
        };

        let difference = minutes as f32 - expected_minutes;
        
        // Calculate satisfaction based on meeting expectations
        if difference >= 0.0 {
            // Met or exceeded expectations
            let excess_satisfaction = (difference / expected_minutes).min(0.5) * 3.0;
            excess_satisfaction.max(0.0)
        } else {
            // Fell short of expectations
            let shortfall_dissatisfaction = (difference / expected_minutes).max(-1.0) * 5.0;
            shortfall_dissatisfaction.min(0.0)
        }
    }

    /// Calculates morale change based on team result
    fn calculate_team_result_effect(&self, result: GameResult) -> f32 {
        match result {
            GameResult::Win => 2.0,
            GameResult::Draw => 0.5,
            GameResult::Loss => -1.5,
        }
    }

    /// Calculates morale change based on contract status
    fn calculate_contract_effect(&self, status: ContractStatus, contract: &crate::entities::Contract) -> f32 {
        match status {
            ContractStatus::Active => 0.0,
            ContractStatus::ExpiringSoon => {
                // Depends on player's squad role (which affects ambition)
                let ambition_factor = contract.squad_role.ambition_factor();
                let loyalty_factor = -2.0; // Simplified loyalty factor
                ambition_factor + loyalty_factor
            },
            ContractStatus::Expired => -5.0,
            ContractStatus::Renewed => 3.0,
            ContractStatus::Negotiating => {
                // Uncertainty affects morale
                let uncertainty = -1.0;
                let potential_opportunity = contract.squad_role.ambition_factor() * 1.5;
                uncertainty + potential_opportunity
            },
        }
    }

    /// Calculates morale change based on media attention
    fn calculate_media_effect(&self, attention: MediaAttention, ego: u8) -> f32 {
        let ego_factor = ego as f32 / 50.0; // Normalize to 0-2 scale
        
        match attention {
            MediaAttention::Positive => 1.5 * ego_factor,
            MediaAttention::Neutral => 0.0,
            MediaAttention::Negative => -1.0 * ego_factor,
            MediaAttention::IntensePositive => 3.0 * ego_factor,
            MediaAttention::IntenseNegative => -4.0 * ego_factor,
        }
    }

    /// Calculates morale change based on relationship changes
    fn calculate_relationship_effect(&self, relationship_change: f32, loyalty: u8) -> f32 {
        let loyalty_factor = loyalty as f32 / 100.0;
        relationship_change * loyalty_factor
    }

    /// Calculates morale drift when player hasn't played in a while
    fn calculate_time_drift_effect(&self, days: u32) -> f32 {
        // Morale drifts toward a baseline (e.g., 50) when not actively engaged
        let drift_rate = (days as f32 / 7.0) * 0.5; // 0.5 per week
        -drift_rate // Always negative, morale decreases without activity
    }

    /// Calculates the effect of morale on performance
    pub fn calculate_morale_performance_modifier(&self, morale: f32) -> f32 {
        // Morale affects performance multiplicatively
        // At 100 morale: 1.1x performance
        // At 50 morale: 1.0x performance (baseline)
        // At 0 morale: 0.7x performance
        if morale >= 50.0 {
            1.0 + (morale - 50.0) / 500.0 // Range: 1.0 to 1.1
        } else {
            1.0 - (50.0 - morale) / 166.67 // Range: 0.7 to 1.0
        }
    }

    /// Calculates the effect of morale on development rate
    pub fn calculate_morale_development_modifier(&self, morale: f32) -> f32 {
        // Higher morale leads to better development
        // At 100 morale: 1.2x development
        // At 50 morale: 1.0x development (baseline)
        // At 0 morale: 0.6x development
        if morale >= 50.0 {
            1.0 + (morale - 50.0) / 250.0 // Range: 1.0 to 1.2
        } else {
            1.0 - (50.0 - morale) / 125.0 // Range: 0.6 to 1.0
        }
    }

    /// Calculates the effect of morale on injury proneness
    pub fn calculate_morale_injury_modifier(&self, morale: f32) -> f32 {
        // Lower morale increases injury risk
        // At 100 morale: 0.8x injury chance
        // At 50 morale: 1.0x injury chance (baseline)
        // At 0 morale: 1.5x injury chance
        if morale >= 50.0 {
            1.0 - (morale - 50.0) / 250.0 // Range: 0.8 to 1.0
        } else {
            1.0 + (50.0 - morale) / 100.0 // Range: 1.0 to 1.5
        }
    }
}

// SquadRoleExt removed as it was unused

/// Result of a game
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GameResult {
    Win,
    Draw,
    Loss,
}

/// Level of media attention
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MediaAttention {
    Positive,
    Neutral,
    Negative,
    IntensePositive,  // Major media event
    IntenseNegative,  // Controversy
}

/// Status of player's contract
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ContractStatus {
    Active,
    ExpiringSoon,     // Less than 6 months remaining
    Expired,
    Renewed,          // Recently renewed
    Negotiating,      // In negotiation
}

/// Personality traits that affect how a player responds to situations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityProfile {
    /// How much the player is affected by team success/failure
    pub team_connection: f32,        // 0.0 to 1.0
    
    /// How much the player craves recognition
    pub recognition_need: f32,       // 0.0 to 1.0
    
    /// How resilient the player is to setbacks
    pub resilience: f32,             // 0.0 to 1.0
    
    /// How much the player values loyalty to the club
    pub club_loyalty: f32,           // 0.0 to 1.0
    
    /// How ambitious the player is
    pub ambition_level: f32,         // 0.0 to 1.0
    
    /// How much the player is affected by pressure
    pub pressure_sensitivity: f32,   // 0.0 to 1.0
}

impl PersonalityProfile {
    pub fn new(
        team_connection: f32,
        recognition_need: f32,
        resilience: f32,
        club_loyalty: f32,
        ambition_level: f32,
        pressure_sensitivity: f32,
    ) -> Self {
        PersonalityProfile {
            team_connection: team_connection.max(0.0).min(1.0),
            recognition_need: recognition_need.max(0.0).min(1.0),
            resilience: resilience.max(0.0).min(1.0),
            club_loyalty: club_loyalty.max(0.0).min(1.0),
            ambition_level: ambition_level.max(0.0).min(1.0),
            pressure_sensitivity: pressure_sensitivity.max(0.0).min(1.0),
        }
    }
}

/// The PersonalityEngine handles personality-driven behaviors and reactions
pub struct PersonalityEngine;

impl PersonalityEngine {
    pub fn new() -> Self {
        PersonalityEngine
    }

    /// Determines how a player reacts to a specific situation based on their personality
    pub fn determine_reaction(
        &self,
        personality: &PersonalityProfile,
        situation: &SituationType,
        _intensity: f32,  // How intense the situation is (0.0 to 1.0)
    ) -> ReactionOutcome {
        match situation {
            SituationType::TeamSuccess => {
                if personality.team_connection > 0.6 {
                    ReactionOutcome::PositiveEngagement
                } else {
                    ReactionOutcome::Indifferent
                }
            },
            SituationType::TeamFailure => {
                if personality.resilience > 0.5 {
                    ReactionOutcome::ConstructiveResponse
                } else if personality.team_connection > 0.7 {
                    ReactionOutcome::Disappointed
                } else {
                    ReactionOutcome::BlameOthers
                }
            },
            SituationType::PersonalAchievement => {
                if personality.recognition_need > 0.6 {
                    ReactionOutcome::Celebratory
                } else {
                    ReactionOutcome::Humble
                }
            },
            SituationType::ContractDispute => {
                if personality.ambition_level > 0.7 && personality.club_loyalty < 0.4 {
                    ReactionOutcome::DemandTransfer
                } else if personality.club_loyalty > 0.7 {
                    ReactionOutcome::PatientNegotiation
                } else {
                    ReactionOutcome::SeekCompromise
                }
            },
            SituationType::PressureSituation => {
                if personality.pressure_sensitivity > 0.6 {
                    ReactionOutcome::Choke
                } else {
                    ReactionOutcome::RiseToChallenge
                }
            },
            SituationType::RelationshipConflict => {
                if personality.team_connection > 0.6 {
                    ReactionOutcome::SeekResolution
                } else {
                    ReactionOutcome::Withdraw
                }
            },
        }
    }

    /// Calculates how likely a player is to engage positively with teammates
    pub fn calculate_teammate_engagement_chance(
        &self,
        personality: &PersonalityProfile,
        teammate_personality: &PersonalityProfile,
    ) -> f32 {
        // Compatibility based on similar traits
        let compatibility = (1.0 - (personality.team_connection - teammate_personality.team_connection).abs()) * 0.3
            + (1.0 - (personality.club_loyalty - teammate_personality.club_loyalty).abs()) * 0.2
            + 0.5; // Base chance
        
        compatibility.min(1.0)
    }

    /// Calculates how a player handles stress based on personality
    pub fn calculate_stress_response(
        &self,
        personality: &PersonalityProfile,
        stress_level: f32,  // 0.0 to 1.0
    ) -> StressResponse {
        let effective_resilience = personality.resilience * (1.0 - personality.pressure_sensitivity * 0.5);
        
        if stress_level < 0.3 {
            StressResponse::Calm
        } else if stress_level < 0.6 {
            if effective_resilience > 0.6 {
                StressResponse::Managing
            } else {
                StressResponse::Struggling
            }
        } else {
            if effective_resilience > 0.75 {
                StressResponse::HandlingWell
            } else if effective_resilience > 0.4 {
                StressResponse::Coping
            } else {
                StressResponse::Overwhelmed
            }
        }
    }
}

/// Types of situations that trigger personality reactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SituationType {
    TeamSuccess,
    TeamFailure,
    PersonalAchievement,
    ContractDispute,
    PressureSituation,
    RelationshipConflict,
}

/// Possible reactions to situations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReactionOutcome {
    PositiveEngagement,
    ConstructiveResponse,
    Disappointed,
    BlameOthers,
    Celebratory,
    Humble,
    DemandTransfer,
    PatientNegotiation,
    SeekCompromise,
    Choke,
    RiseToChallenge,
    SeekResolution,
    Withdraw,
    Indifferent,
}

/// How a player responds to stress
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StressResponse {
    Calm,
    Managing,
    Struggling,
    HandlingWell,
    Coping,
    Overwhelmed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_morale_performance_modifier() {
        let engine = MoraleEngine::new();
        
        // Test high morale
        assert!((engine.calculate_morale_performance_modifier(100.0) - 1.1).abs() < 0.01);
        
        // Test neutral morale
        assert!((engine.calculate_morale_performance_modifier(50.0) - 1.0).abs() < 0.01);
        
        // Test low morale
        assert!((engine.calculate_morale_performance_modifier(0.0) - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_morale_development_modifier() {
        let engine = MoraleEngine::new();
        
        // Test high morale
        assert!((engine.calculate_morale_development_modifier(100.0) - 1.2).abs() < 0.01);
        
        // Test neutral morale
        assert!((engine.calculate_morale_development_modifier(50.0) - 1.0).abs() < 0.01);
        
        // Test low morale
        assert!((engine.calculate_morale_development_modifier(0.0) - 0.6).abs() < 0.01);
    }

    #[test]
    fn test_morale_injury_modifier() {
        let engine = MoraleEngine::new();
        
        // Test high morale
        assert!((engine.calculate_morale_injury_modifier(100.0) - 0.8).abs() < 0.01);
        
        // Test neutral morale
        assert!((engine.calculate_morale_injury_modifier(50.0) - 1.0).abs() < 0.01);
        
        // Test low morale
        assert!((engine.calculate_morale_injury_modifier(0.0) - 1.5).abs() < 0.01);
    }

    #[test]
    fn test_personality_reaction() {
        let engine = PersonalityEngine::new();
        
        let profile = PersonalityProfile::new(0.8, 0.3, 0.7, 0.9, 0.4, 0.2);
        
        // Test team success reaction
        let reaction = engine.determine_reaction(&profile, &SituationType::TeamSuccess, 0.8);
        assert_eq!(reaction, ReactionOutcome::PositiveEngagement);
        
        // Test pressure situation reaction
        let reaction = engine.determine_reaction(&profile, &SituationType::PressureSituation, 0.9);
        assert_eq!(reaction, ReactionOutcome::RiseToChallenge);
    }

    #[test]
    fn test_stress_response() {
        let engine = PersonalityEngine::new();
        
        let profile = PersonalityProfile::new(0.5, 0.5, 0.8, 0.5, 0.5, 0.1);
        
        // Test low stress
        let response = engine.calculate_stress_response(&profile, 0.2);
        assert_eq!(response, StressResponse::Calm);
        
        // Test high stress with high resilience
        let response = engine.calculate_stress_response(&profile, 0.8);
        assert_eq!(response, StressResponse::HandlingWell);
    }
}