// src/systems/social_system.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;



/// The SocialEngine tracks relationships between players and other entities
/// It influences morale, playing time, development, and career opportunities
pub struct SocialEngine;

impl SocialEngine {
    /// Creates a new SocialEngine instance
    pub fn new() -> Self {
        SocialEngine
    }

    /// Updates a relationship between two entities
    pub fn update_relationship(
        &self,
        relationships: &mut HashMap<Uuid, f32>,
        other_entity_id: Uuid,
        change: f32,
        personality_factors: &PersonalityFactors,
    ) -> f32 {
        // Get current relationship value
        let current_value = *relationships.get(&other_entity_id).unwrap_or(&50.0);
        
        // Apply personality modifiers
        let modified_change = self.apply_personality_modifiers(change, personality_factors);
        
        // Calculate new value with boundaries
        let new_value = (current_value + modified_change).max(0.0).min(100.0);
        
        // Update the relationship
        relationships.insert(other_entity_id, new_value);
        
        new_value
    }

    /// Applies personality-based modifiers to relationship changes
    fn apply_personality_modifiers(&self, change: f32, factors: &PersonalityFactors) -> f32 {
        // Apply loyalty modifier (loyal people have stronger relationships)
        let loyalty_modifier = 1.0 + (factors.loyalty as f32 / 200.0);
        
        // Apply ego modifier (high ego people have more volatile relationships)
        let ego_modifier = if change > 0.0 {
            1.0 + (factors.ego as f32 / 300.0)  // Positive changes amplified by ego
        } else {
            1.0 + (factors.ego as f32 / 200.0)  // Negative changes amplified by ego
        };
        
        // Apply teamwork modifier (team-oriented people build better relationships)
        let teamwork_modifier = 1.0 + (factors.teamwork as f32 / 250.0);
        
        change * loyalty_modifier * ego_modifier * teamwork_modifier
    }

    /// Calculates the impact of relationships on player morale
    pub fn calculate_relationship_morale_impact(
        &self,
        relationships: &HashMap<Uuid, f32>,
        relationship_types: &HashMap<Uuid, RelationshipType>,
    ) -> f32 {
        let mut total_impact = 0.0;
        let mut count = 0;
        
        for (entity_id, value) in relationships {
            if let Some(rel_type) = relationship_types.get(entity_id) {
                let weight = match rel_type {
                    RelationshipType::Manager => 0.3,      // Manager has high impact
                    RelationshipType::Teammate => 0.1,     // Teammates moderate impact
                    RelationshipType::Agent => 0.2,        // Agent has significant impact
                    RelationshipType::Family => 0.25,      // Family has high impact
                    RelationshipType::Media => 0.05,       // Media has low impact
                    RelationshipType::Fans => 0.15,        // Fans have moderate impact
                    &RelationshipType::Club => 0.2,        // Club has moderate impact
                };
                
                // Calculate impact based on relationship value (centered around 50)
                let impact = (value - 50.0) * weight;
                total_impact += impact;
                count += 1;
            }
        }
        
        if count > 0 {
            total_impact / count as f32
        } else {
            0.0
        }
    }

    /// Calculates the impact of relationships on playing time decisions
    pub fn calculate_relationship_playing_time_impact(
        &self,
        _player_id: Uuid,
        manager_relationship: f32,
        teammate_relationships: &HashMap<Uuid, f32>,
        manager_profile: &ManagerProfile,
    ) -> f32 {
        // Manager relationship has the biggest impact
        let manager_impact = manager_relationship * manager_profile.favoritism / 100.0;
        
        // Teammate relationships affect team chemistry
        let avg_teammate_relationship = if teammate_relationships.is_empty() {
            50.0
        } else {
            let sum: f32 = teammate_relationships.values().sum();
            sum / teammate_relationships.len() as f32
        };
        
        // Chemistry impact based on manager's team orientation
        let chemistry_impact = (avg_teammate_relationship - 50.0) * (manager_profile.youth_trust / 100.0) * 0.1;
        
        manager_impact + chemistry_impact
    }

    /// Determines the success chance of a social interaction
    pub fn calculate_interaction_success_chance(
        &self,
        initiator_relationship: f32,
        target_relationship: f32,
        interaction_type: InteractionType,
        personality_factors: &PersonalityFactors,
    ) -> f32 {
        // Base chance based on mutual relationship
        let base_chance = (initiator_relationship + target_relationship) / 2.0 / 100.0;
        
        // Apply interaction type modifier
        let interaction_modifier = match interaction_type {
            InteractionType::PositiveEncouragement => 1.2,
            InteractionType::ConstructiveFeedback => 1.0,
            InteractionType::SeriousRequest => 0.8,
            InteractionType::Conflict => 0.6,
            InteractionType::AdviceSeeking => 1.1,
        };
        
        // Apply personality modifiers
        let teamwork_modifier = personality_factors.teamwork as f32 / 100.0;
        let communication_modifier = personality_factors.communication as f32 / 100.0;
        
        let final_chance = base_chance * interaction_modifier * teamwork_modifier * communication_modifier;
        
        final_chance.clamp(0.1, 0.95)  // Ensure some chance of failure/success
    }

    /// Processes a social interaction between two entities
    pub fn process_interaction(
        &self,
        relationships: &mut HashMap<Uuid, f32>,
        initiator_id: Uuid,
        target_id: Uuid,
        interaction_type: InteractionType,
        personality_factors: &PersonalityFactors,
        success: bool,
    ) -> InteractionResult {
        let success_factor: f32 = if success { 1.0 } else { -0.5 };
        
        // Determine relationship changes based on interaction type and success
        let (initiator_change, target_change) = match interaction_type {
            InteractionType::PositiveEncouragement => {
                if success {
                    (2.0 * success_factor, 3.0 * success_factor)
                } else {
                    (-1.0, -2.0)
                }
            },
            InteractionType::ConstructiveFeedback => {
                if success {
                    (1.0 * success_factor, 2.0 * success_factor)
                } else {
                    (-2.0, -1.0)
                }
            },
            InteractionType::SeriousRequest => {
                if success {
                    (3.0 * success_factor, 1.0 * success_factor)
                } else {
                    (-3.0, 0.5)
                }
            },
            InteractionType::Conflict => {
                if success {  // "Success" in conflict might mean winning the argument
                    (1.0 * success_factor, -3.0 * success_factor.abs())
                } else {
                    (-2.0, -2.0)
                }
            },
            InteractionType::AdviceSeeking => {
                if success {
                    (2.0 * success_factor, 1.5 * success_factor)
                } else {
                    (-1.0, -0.5)
                }
            },
        };
        
        // Update relationships
        let new_initiator_rel = self.update_relationship(
            relationships,
            initiator_id,
            initiator_change,
            personality_factors,
        );
        
        let new_target_rel = self.update_relationship(
            relationships,
            target_id,
            target_change,
            personality_factors,
        );
        
        InteractionResult {
            initiator_new_relationship: new_initiator_rel,
            target_new_relationship: new_target_rel,
            success,
        }
    }

    /// Calculates the impact of relationships on transfer decisions
    pub fn calculate_relationship_transfer_impact(
        &self,
        player_relationships: &HashMap<Uuid, f32>,
        relationship_types: &HashMap<Uuid, RelationshipType>,
        current_club_id: Uuid,
    ) -> TransferInfluence {
        let mut club_loyalty = 0.0;
        let mut manager_influence = 0.0;
        let mut teammate_influence = 0.0;
        
        for (entity_id, value) in player_relationships {
            if let Some(rel_type) = relationship_types.get(entity_id) {
                match rel_type {
                    RelationshipType::Manager => {
                        manager_influence = *value;
                    },
                    RelationshipType::Teammate => {
                        teammate_influence += *value;
                    },
                    RelationshipType::Club if *entity_id == current_club_id => {
                        club_loyalty = *value;
                    },
                    _ => {}
                }
            }
        }
        
        // Average teammate influence
        let avg_teammate_influence = if player_relationships.values().any(|_| true) {
            teammate_influence / player_relationships.len() as f32
        } else {
            50.0
        };
        
        TransferInfluence {
            club_loyalty,
            manager_influence,
            teammate_influence: avg_teammate_influence,
        }
    }

    /// Gets relationship status description
    pub fn get_relationship_status(&self, value: f32) -> RelationshipStatus {
        if value >= 80.0 {
            RelationshipStatus::Excellent
        } else if value >= 60.0 {
            RelationshipStatus::Good
        } else if value >= 40.0 {
            RelationshipStatus::Neutral
        } else if value >= 20.0 {
            RelationshipStatus::Poor
        } else {
            RelationshipStatus::Terrible
        }
    }
}

/// Personality factors that affect social interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityFactors {
    pub loyalty: u8,         // 0-100, affects relationship stability
    pub ego: u8,             // 0-100, affects relationship volatility
    pub teamwork: u8,        // 0-100, affects ability to build relationships
    pub communication: u8,   // 0-100, affects interaction success
    pub trust: u8,           // 0-100, affects how relationships form
}

impl PersonalityFactors {
    pub fn new(loyalty: u8, ego: u8, teamwork: u8, communication: u8, trust: u8) -> Self {
        PersonalityFactors {
            loyalty: loyalty.min(100),
            ego: ego.min(100),
            teamwork: teamwork.min(100),
            communication: communication.min(100),
            trust: trust.min(100),
        }
    }
}

/// Types of relationships
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum RelationshipType {
    Manager,
    Teammate,
    Agent,
    Family,
    Media,
    Fans,
    Club,
}

/// Types of social interactions
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InteractionType {
    PositiveEncouragement,
    ConstructiveFeedback,
    SeriousRequest,
    Conflict,
    AdviceSeeking,
}

/// Result of a social interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionResult {
    pub initiator_new_relationship: f32,
    pub target_new_relationship: f32,
    pub success: bool,
}

/// Influence of relationships on transfer decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferInfluence {
    pub club_loyalty: f32,
    pub manager_influence: f32,
    pub teammate_influence: f32,
}

/// Status of a relationship
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RelationshipStatus {
    Excellent,
    Good,
    Neutral,
    Poor,
    Terrible,
}

/// Manager profile affecting relationship dynamics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagerProfile {
    pub favoritism: f32,        // 0-100, how much favorites matter
    pub youth_trust: f32,       // 0-100, how much team chemistry matters
    pub discipline: f32,        // 0-100, how relationships affect discipline
    pub communication_style: CommunicationStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationStyle {
    Direct,
    Diplomatic,
    Authoritative,
    Collaborative,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_update() {
        let engine = SocialEngine::new();
        let mut relationships = HashMap::new();
        
        let personality = PersonalityFactors::new(80, 50, 70, 60, 75);
        
        // Test positive relationship change
        let new_value = engine.update_relationship(
            &mut relationships,
            Uuid::new_v4(),
            10.0,
            &personality,
        );
        
        assert!(new_value > 50.0);  // Should be above neutral
        
        // Test negative relationship change
        let other_id = Uuid::new_v4();
        let new_value = engine.update_relationship(
            &mut relationships,
            other_id,
            -15.0,
            &personality,
        );
        
        assert!(new_value < 50.0);  // Should be below neutral
    }

    #[test]
    fn test_personality_modifiers() {
        let engine = SocialEngine::new();
        
        let high_loyalty = PersonalityFactors::new(90, 30, 60, 70, 80);
        let high_ego = PersonalityFactors::new(50, 90, 60, 70, 80);
        
        // Test that high loyalty amplifies positive changes
        let modified_high_loyalty = engine.apply_personality_modifiers(5.0, &high_loyalty);
        let modified_low_loyalty = engine.apply_personality_modifiers(5.0, &PersonalityFactors::new(30, 50, 60, 70, 80));
        
        assert!(modified_high_loyalty > modified_low_loyalty);
        
        // Test that high ego amplifies both positive and negative changes
        let modified_pos_ego = engine.apply_personality_modifiers(5.0, &high_ego);
        let modified_neg_ego = engine.apply_personality_modifiers(-5.0, &high_ego);
        
        assert!(modified_pos_ego > 5.0);  // Positive change amplified
        assert!(modified_neg_ego < -5.0); // Negative change amplified
    }

    #[test]
    fn test_relationship_status() {
        let engine = SocialEngine::new();
        
        assert_eq!(engine.get_relationship_status(90.0), RelationshipStatus::Excellent);
        assert_eq!(engine.get_relationship_status(70.0), RelationshipStatus::Good);
        assert_eq!(engine.get_relationship_status(50.0), RelationshipStatus::Neutral);
        assert_eq!(engine.get_relationship_status(30.0), RelationshipStatus::Poor);
        assert_eq!(engine.get_relationship_status(10.0), RelationshipStatus::Terrible);
    }

    #[test]
    fn test_interaction_success_chance() {
        let engine = SocialEngine::new();
        
        let personality = PersonalityFactors::new(70, 40, 80, 90, 75);
        
        // Test high relationship, positive interaction
        let chance = engine.calculate_interaction_success_chance(
            80.0,
            85.0,
            InteractionType::PositiveEncouragement,
            &personality,
        );
        
        assert!(chance > 0.7);  // Should be quite high
        
        // Test low relationship, serious request
        let chance = engine.calculate_interaction_success_chance(
            20.0,
            25.0,
            InteractionType::SeriousRequest,
            &personality,
        );
        
        assert!(chance < 0.5);  // Should be lower
    }
}