// src/systems/development_system.rs
// Removed unused serde imports
// Morale system cleanup
// Removed unused HashMap

use crate::entities::{
    Player, HiddenAttributes
};
use crate::systems::training_system::TrainingFocus;

/// The PlayerDevelopmentEngine handles growth, decline, and form
/// It applies training effects, match performance impact, manages attribute 
/// growth curves based on age, applies morale effects, and handles hidden attributes
pub struct PlayerDevelopmentEngine;

impl PlayerDevelopmentEngine {
    /// Creates a new PlayerDevelopmentEngine instance
    pub fn new() -> Self {
        PlayerDevelopmentEngine
    }

    /// Updates player attributes based on training, match performance, and time passed
    pub fn update_player_attributes(
        &self,
        player: &mut Player,
        training_focus: TrainingFocus,
        match_performance: Option<f32>, // Rating from last match, if applicable
        days_passed: u32,
    ) {
        // Calculate age-based development factors
        let age_factor = self.calculate_age_development_factor(player.age);
        
        // Calculate training effectiveness
        let training_effectiveness = self.calculate_training_effectiveness(
            training_focus, 
            &player.hidden, 
            player.mental.determination
        );
        
        // Calculate match performance impact if applicable
        let performance_factor = match match_performance {
            Some(rating) => self.normalize_performance(rating),
            None => 0.0, // No match performance to consider
        };
        
        // Apply attribute changes based on age phase
        match player.age {
            15..=18 => self.update_young_development(player, training_effectiveness, performance_factor, age_factor, days_passed),
            19..=23 => self.update_physical_peak_development(player, training_effectiveness, performance_factor, age_factor, days_passed),
            24..=28 => self.update_peak_maintenance(player, training_effectiveness, performance_factor, age_factor, days_passed),
            29..=32 => self.update_aging_phase(player, training_effectiveness, performance_factor, age_factor, days_passed),
            _ => self.update_late_career_decline(player, training_effectiveness, performance_factor, age_factor, days_passed),
        }
        
        // Apply fatigue and injury effects
        self.apply_fatigue_effects(player);
        self.apply_injury_effects(player);
        
        // Update form based on recent performances
        if let Some(rating) = match_performance {
            self.update_form(player, rating);
        }
    }

    /// Calculates age-based development factor
    fn calculate_age_development_factor(&self, age: u8) -> f32 {
        match age {
            15..=18 => 1.2, // Fast growth
            19..=23 => 1.0, // Normal growth
            24..=28 => 0.8, // Slower growth
            29..=32 => 0.6, // Declining
            _ => 0.4,       // Significant decline
        }
    }

    /// Calculates how effective training is based on focus and player traits
    fn calculate_training_effectiveness(
        &self,
        training_focus: TrainingFocus,
        hidden_attrs: &HiddenAttributes,
        determination: u8,
    ) -> f32 {
        let base_effectiveness = match training_focus {
            TrainingFocus::Technical => 0.9,
            TrainingFocus::Physical => 0.9,
            TrainingFocus::Tactical => 0.7,
            TrainingFocus::Mental => 0.75,
            TrainingFocus::Rest => 0.0, // No growth during rest
        };
        
        // Apply player-specific modifiers
        let determination_modifier = (determination as f32) / 100.0;
        let potential_modifier = (hidden_attrs.potential_ceiling as f32) / 100.0;
        
        base_effectiveness * determination_modifier * potential_modifier
    }

    /// Normalizes performance rating to 0-1 scale
    fn normalize_performance(&self, rating: f32) -> f32 {
        // Convert 0-10 rating to 0-1 scale centered around 6.0 (average)
        ((rating - 6.0) / 4.0).max(-1.0).min(1.0)
    }

    /// Updates attributes for young players (ages 15-18)
    fn update_young_development(
        &self,
        player: &mut Player,
        training_effectiveness: f32,
        performance_factor: f32,
        age_factor: f32,
        days_passed: u32,
    ) {
        // Young players focus on technical growth
        self.apply_attribute_growth(
            player,
            training_effectiveness,
            performance_factor,
            age_factor,
            days_passed,
            GrowthCategory::Technical,
        );
    }

    /// Updates attributes for physical peak players (ages 19-23)
    fn update_physical_peak_development(
        &self,
        player: &mut Player,
        training_effectiveness: f32,
        performance_factor: f32,
        age_factor: f32,
        days_passed: u32,
    ) {
        // Focus on physical and technical growth
        self.apply_attribute_growth(
            player,
            training_effectiveness,
            performance_factor,
            age_factor,
            days_passed,
            GrowthCategory::Physical,
        );
    }

    /// Updates attributes for peak maintenance players (ages 24-28)
    fn update_peak_maintenance(
        &self,
        player: &mut Player,
        training_effectiveness: f32,
        performance_factor: f32,
        age_factor: f32,
        days_passed: u32,
    ) {
        // Maintain attributes with slight decline, focus on mental growth
        self.apply_attribute_growth(
            player,
            training_effectiveness * 0.7, // Reduced growth rate
            performance_factor,
            age_factor,
            days_passed,
            GrowthCategory::Mental,
        );
    }

    /// Updates attributes for aging players (ages 29-32)
    fn update_aging_phase(
        &self,
        player: &mut Player,
        training_effectiveness: f32,
        performance_factor: f32,
        age_factor: f32,
        days_passed: u32,
    ) {
        // Slower growth, focus on mental attributes
        self.apply_attribute_growth(
            player,
            training_effectiveness * 0.5, // Further reduced growth rate
            performance_factor,
            age_factor,
            days_passed,
            GrowthCategory::Mental,
        );
    }

    /// Updates attributes for late career players (ages 33+)
    fn update_late_career_decline(
        &self,
        player: &mut Player,
        training_effectiveness: f32,
        performance_factor: f32,
        age_factor: f32,
        days_passed: u32,
    ) {
        // Apply decline with minimal growth
        self.apply_attribute_growth(
            player,
            training_effectiveness * 0.3, // Minimal growth rate
            performance_factor,
            age_factor * 0.8, // Additional decline factor
            days_passed,
            GrowthCategory::None,
        );
    }

    /// Applies attribute growth based on training focus and other factors
    fn apply_attribute_growth(
        &self,
        player: &mut Player,
        training_effectiveness: f32,
        performance_factor: f32,
        age_factor: f32,
        days_passed: u32,
        category: GrowthCategory,
    ) {
        let time_factor = (days_passed as f32) / 7.0; // Normalize to weekly updates
        
        // Calculate base growth amount
        let base_growth = training_effectiveness * age_factor * time_factor;
        
        // Apply growth based on training focus and category
        match category {
            GrowthCategory::Technical => {
                self.increase_technical_attributes(player, base_growth * (1.0 + performance_factor));
            }
            GrowthCategory::Physical => {
                self.increase_physical_attributes(player, base_growth * (1.0 + performance_factor));
            }
            GrowthCategory::Mental => {
                self.increase_mental_attributes(player, base_growth * (1.0 + performance_factor));
            }
            GrowthCategory::None => {
                // Just apply decay factors
                self.apply_natural_decline(player, base_growth * 0.1);
            }
        }
    }

    /// Increases technical attributes
    fn increase_technical_attributes(&self, player: &mut Player, growth_amount: f32) {
        // Apply growth with diminishing returns
        let growth = self.apply_diminishing_returns(growth_amount, player.technical.average());
        
        // Distribute growth among technical attributes
        player.technical.dribbling = self.cap_attribute(
            player.technical.dribbling as f32 + growth * 0.15
        ) as u8;
        player.technical.passing = self.cap_attribute(
            player.technical.passing as f32 + growth * 0.20
        ) as u8;
        player.technical.shooting = self.cap_attribute(
            player.technical.shooting as f32 + growth * 0.18
        ) as u8;
        player.technical.first_touch = self.cap_attribute(
            player.technical.first_touch as f32 + growth * 0.17
        ) as u8;
        player.technical.tackling = self.cap_attribute(
            player.technical.tackling as f32 + growth * 0.15
        ) as u8;
        player.technical.crossing = self.cap_attribute(
            player.technical.crossing as f32 + growth * 0.15
        ) as u8;
    }

    /// Increases physical attributes
    fn increase_physical_attributes(&self, player: &mut Player, growth_amount: f32) {
        // Apply growth with diminishing returns
        let growth = self.apply_diminishing_returns(growth_amount, player.physical.average());
        
        // Distribute growth among physical attributes
        player.physical.pace = self.cap_attribute(
            player.physical.pace as f32 + growth * 0.20
        ) as u8;
        player.physical.stamina = self.cap_attribute(
            player.physical.stamina as f32 + growth * 0.25
        ) as u8;
        player.physical.strength = self.cap_attribute(
            player.physical.strength as f32 + growth * 0.20
        ) as u8;
        player.physical.agility = self.cap_attribute(
            player.physical.agility as f32 + growth * 0.18
        ) as u8;
        player.physical.jumping = self.cap_attribute(
            player.physical.jumping as f32 + growth * 0.17
        ) as u8;
    }

    /// Increases mental attributes
    fn increase_mental_attributes(&self, player: &mut Player, growth_amount: f32) {
        // Apply growth with diminishing returns
        let growth = self.apply_diminishing_returns(growth_amount, player.mental.average());
        
        // Distribute growth among mental attributes
        player.mental.composure = self.cap_attribute(
            player.mental.composure as f32 + growth * 0.18
        ) as u8;
        player.mental.vision = self.cap_attribute(
            player.mental.vision as f32 + growth * 0.20
        ) as u8;
        player.mental.work_rate = self.cap_attribute(
            player.mental.work_rate as f32 + growth * 0.17
        ) as u8;
        player.mental.determination = self.cap_attribute(
            player.mental.determination as f32 + growth * 0.20
        ) as u8;
        player.mental.positioning = self.cap_attribute(
            player.mental.positioning as f32 + growth * 0.15
        ) as u8;
        player.mental.teamwork = self.cap_attribute(
            player.mental.teamwork as f32 + growth * 0.10
        ) as u8;
    }

    /// Applies natural decline to attributes
    fn apply_natural_decline(&self, player: &mut Player, decline_amount: f32) {
        // Apply gradual decline to all attributes
        player.technical.dribbling = (player.technical.dribbling as f32 - decline_amount * 0.1).max(1.0) as u8;
        player.technical.passing = (player.technical.passing as f32 - decline_amount * 0.1).max(1.0) as u8;
        player.technical.shooting = (player.technical.shooting as f32 - decline_amount * 0.1).max(1.0) as u8;
        player.technical.first_touch = (player.technical.first_touch as f32 - decline_amount * 0.1).max(1.0) as u8;
        player.technical.tackling = (player.technical.tackling as f32 - decline_amount * 0.1).max(1.0) as u8;
        player.technical.crossing = (player.technical.crossing as f32 - decline_amount * 0.1).max(1.0) as u8;
        
        player.physical.pace = (player.physical.pace as f32 - decline_amount * 0.15).max(1.0) as u8;
        player.physical.stamina = (player.physical.stamina as f32 - decline_amount * 0.1).max(1.0) as u8;
        player.physical.strength = (player.physical.strength as f32 - decline_amount * 0.1).max(1.0) as u8;
        player.physical.agility = (player.physical.agility as f32 - decline_amount * 0.15).max(1.0) as u8;
        player.physical.jumping = (player.physical.jumping as f32 - decline_amount * 0.1).max(1.0) as u8;
        
        player.mental.composure = (player.mental.composure as f32 - decline_amount * 0.05).max(1.0) as u8;
        player.mental.vision = (player.mental.vision as f32 - decline_amount * 0.05).max(1.0) as u8;
        player.mental.work_rate = (player.mental.work_rate as f32 - decline_amount * 0.1).max(1.0) as u8;
        player.mental.determination = (player.mental.determination as f32 - decline_amount * 0.05).max(1.0) as u8;
        player.mental.positioning = (player.mental.positioning as f32 - decline_amount * 0.05).max(1.0) as u8;
        player.mental.teamwork = (player.mental.teamwork as f32 - decline_amount * 0.05).max(1.0) as u8;
    }

    /// Applies fatigue effects to player attributes
    fn apply_fatigue_effects(&self, player: &mut Player) {
        // Higher fatigue reduces performance
        let _fatigue_penalty = player.fatigue / 100.0 * 0.15; // Up to 15% penalty
        
        // Apply fatigue penalty to all attributes temporarily
        // These are applied during match simulation, not permanently
        player.fatigue = player.fatigue.min(100.0).max(0.0);
    }

    /// Applies injury effects to player attributes
    fn apply_injury_effects(&self, player: &mut Player) {
        // Check if there's an injury without holding a reference
        if player.injury_status.is_some() {
            // Clone the injury to avoid borrowing issues
            let injury_clone = player.injury_status.clone();

            if let Some(injury) = injury_clone {
                // Apply temporary attribute reductions based on injury
                for affected_attr in &injury.affected_attributes {
                    match &affected_attr.attribute {
                        crate::entities::AttributeType::Technical(attr) => {
                            self.reduce_technical_attribute(player, attr.clone(), affected_attr.reduction_percentage);
                        },
                        crate::entities::AttributeType::Physical(attr) => {
                            self.reduce_physical_attribute(player, attr.clone(), affected_attr.reduction_percentage);
                        },
                        crate::entities::AttributeType::Mental(attr) => {
                            self.reduce_mental_attribute(player, attr.clone(), affected_attr.reduction_percentage);
                        },
                    }
                }

                // Decrement weeks remaining
                if injury.weeks_remaining > 0 {
                    // In a real implementation, this would be handled by a separate system
                    // that tracks recovery progress
                }

                // Put the injury back
                player.injury_status = Some(injury);
            }
        }
    }

    /// Updates player form based on match performance
    fn update_form(&self, player: &mut Player, match_rating: f32) {
        // Add the new rating to the form history
        player.form_history.push(match_rating);
        
        // Keep only the last 5 ratings
        if player.form_history.len() > 5 {
            player.form_history.remove(0);
        }
        
        // Calculate new average form
        if !player.form_history.is_empty() {
            let sum: f32 = player.form_history.iter().sum();
            player.form = sum / player.form_history.len() as f32;
        }
    }

    /// Caps an attribute value between 1 and 100
    fn cap_attribute(&self, value: f32) -> f32 {
        value.max(1.0).min(100.0)
    }

    /// Applies diminishing returns to growth based on current attribute level
    fn apply_diminishing_returns(&self, base_growth: f32, current_average: f32) -> f32 {
        // Higher attributes grow more slowly
        let diminishing_factor = 1.0 - (current_average / 200.0); // As attributes approach 100, growth slows
        base_growth * diminishing_factor.max(0.1) // Ensure minimum growth
    }

    /// Reduces a technical attribute by a percentage
    fn reduce_technical_attribute(&self, player: &mut Player, attr: crate::entities::TechnicalAttribute, reduction: f32) {
        match attr {
            crate::entities::TechnicalAttribute::Dribbling => {
                player.technical.dribbling = (player.technical.dribbling as f32 * (1.0 - reduction)).round() as u8;
            },
            crate::entities::TechnicalAttribute::Passing => {
                player.technical.passing = (player.technical.passing as f32 * (1.0 - reduction)).round() as u8;
            },
            crate::entities::TechnicalAttribute::Shooting => {
                player.technical.shooting = (player.technical.shooting as f32 * (1.0 - reduction)).round() as u8;
            },
            crate::entities::TechnicalAttribute::FirstTouch => {
                player.technical.first_touch = (player.technical.first_touch as f32 * (1.0 - reduction)).round() as u8;
            },
            crate::entities::TechnicalAttribute::Tackling => {
                player.technical.tackling = (player.technical.tackling as f32 * (1.0 - reduction)).round() as u8;
            },
            crate::entities::TechnicalAttribute::Crossing => {
                player.technical.crossing = (player.technical.crossing as f32 * (1.0 - reduction)).round() as u8;
            },
        }
    }

    /// Reduces a physical attribute by a percentage
    fn reduce_physical_attribute(&self, player: &mut Player, attr: crate::entities::PhysicalAttribute, reduction: f32) {
        match attr {
            crate::entities::PhysicalAttribute::Pace => {
                player.physical.pace = (player.physical.pace as f32 * (1.0 - reduction)).round() as u8;
            },
            crate::entities::PhysicalAttribute::Stamina => {
                player.physical.stamina = (player.physical.stamina as f32 * (1.0 - reduction)).round() as u8;
            },
            crate::entities::PhysicalAttribute::Strength => {
                player.physical.strength = (player.physical.strength as f32 * (1.0 - reduction)).round() as u8;
            },
            crate::entities::PhysicalAttribute::Agility => {
                player.physical.agility = (player.physical.agility as f32 * (1.0 - reduction)).round() as u8;
            },
            crate::entities::PhysicalAttribute::Jumping => {
                player.physical.jumping = (player.physical.jumping as f32 * (1.0 - reduction)).round() as u8;
            },
        }
    }

    /// Reduces a mental attribute by a percentage
    fn reduce_mental_attribute(&self, player: &mut Player, attr: crate::entities::MentalAttribute, reduction: f32) {
        match attr {
            crate::entities::MentalAttribute::Composure => {
                player.mental.composure = (player.mental.composure as f32 * (1.0 - reduction)).round() as u8;
            },
            crate::entities::MentalAttribute::Vision => {
                player.mental.vision = (player.mental.vision as f32 * (1.0 - reduction)).round() as u8;
            },
            crate::entities::MentalAttribute::WorkRate => {
                player.mental.work_rate = (player.mental.work_rate as f32 * (1.0 - reduction)).round() as u8;
            },
            crate::entities::MentalAttribute::Determination => {
                player.mental.determination = (player.mental.determination as f32 * (1.0 - reduction)).round() as u8;
            },
            crate::entities::MentalAttribute::Positioning => {
                player.mental.positioning = (player.mental.positioning as f32 * (1.0 - reduction)).round() as u8;
            },
            crate::entities::MentalAttribute::Teamwork => {
                player.mental.teamwork = (player.mental.teamwork as f32 * (1.0 - reduction)).round() as u8;
            },
        }
    }
}

/// Categories for attribute growth
#[derive(Debug, Clone, Copy)]
enum GrowthCategory {
    Technical,
    Physical,
    Mental,
    None,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{Player, Position, Foot, CareerStats, Contract, SquadRole, HiddenAttributes};
    use chrono::NaiveDate;

    #[test]
    fn test_age_development_factors() {
        let engine = PlayerDevelopmentEngine::new();
        
        assert_eq!(engine.calculate_age_development_factor(16), 1.2); // Young growth
        assert_eq!(engine.calculate_age_development_factor(22), 1.0); // Normal growth
        assert_eq!(engine.calculate_age_development_factor(26), 0.8); // Slower growth
        assert_eq!(engine.calculate_age_development_factor(31), 0.6); // Declining
        assert_eq!(engine.calculate_age_development_factor(35), 0.4); // Significant decline
    }

    #[test]
    fn test_training_effectiveness() {
        let engine = PlayerDevelopmentEngine::new();
        
        let hidden_attrs = HiddenAttributes {
            injury_proneness: 20,
            consistency: 70,
            big_match_temperament: 80,
            professionalism: 90,
            potential_ceiling: 85,
            versatility: 75,
            ambition: 80,
            loyalty: 60,
            ego: 70,
        };
        
        // Test technical focus
        let tech_eff = engine.calculate_training_effectiveness(TrainingFocus::Technical, &hidden_attrs, 80);
        assert!((tech_eff - 0.612).abs() < 0.01); // 0.8 * (80/100) * (85/100)
        
        // Test physical focus
        let phys_eff = engine.calculate_training_effectiveness(TrainingFocus::Physical, &hidden_attrs, 90);
        assert!((phys_eff - 0.6885).abs() < 0.01); // 0.9 * (90/100) * (85/100)
        
        // Test rest focus
        let rest_eff = engine.calculate_training_effectiveness(TrainingFocus::Rest, &hidden_attrs, 90);
        assert_eq!(rest_eff, 0.0);
    }

    #[test]
    fn test_normalize_performance() {
        let engine = PlayerDevelopmentEngine::new();
        
        assert_eq!(engine.normalize_performance(6.0), 0.0);  // Average performance
        assert_eq!(engine.normalize_performance(10.0), 1.0); // Perfect performance
        assert_eq!(engine.normalize_performance(2.0), -1.0); // Terrible performance
        assert_eq!(engine.normalize_performance(8.0), 0.5);  // Good performance
    }

    #[test]
    fn test_attribute_capping() {
        let engine = PlayerDevelopmentEngine::new();
        
        assert_eq!(engine.cap_attribute(105.0), 100.0); // Above max
        assert_eq!(engine.cap_attribute(-5.0), 1.0);    // Below min
        assert_eq!(engine.cap_attribute(50.0), 50.0);   // Within range
    }

    #[test]
    fn test_diminishing_returns() {
        let engine = PlayerDevelopmentEngine::new();
        
        // With low average attributes, growth should be mostly preserved
        let high_return = engine.apply_diminishing_returns(1.0, 20.0);
        assert!(high_return > 0.85); // Should preserve most of the growth
        
        // With high average attributes, growth should be significantly reduced
        let low_return = engine.apply_diminishing_returns(1.0, 90.0);
        assert!(low_return <= 0.55); // Should reduce growth significantly
    }
}