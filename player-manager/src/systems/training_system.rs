// src/systems/training_system.rs
use serde::{Deserialize, Serialize};


use crate::entities::Player;

/// The TrainingSystem manages player training focus and its effects
/// It compares manager-assigned focus with player preferred focus
/// and generates morale effects based on alignment
pub struct TrainingSystem;

impl TrainingSystem {
    /// Creates a new TrainingSystem instance
    pub fn new() -> Self {
        TrainingSystem
    }

    /// Processes a week of training for a player
    pub fn process_training_week(
        &self,
        player: &mut Player,
        manager_assigned_focus: TrainingFocus,
        player_preferred_focus: Option<TrainingFocus>,
        coach_quality: f32,  // 0-100 scale
        training_intensity: f32,  // 0-100 scale
        facilities_quality: f32,  // 0-100 scale
    ) -> TrainingResult {
        // Calculate alignment between manager and player preferences
        let alignment = self.calculate_focus_alignment(manager_assigned_focus, player_preferred_focus);
        
        // Calculate training effectiveness
        let effectiveness = self.calculate_training_effectiveness(
            manager_assigned_focus,
            alignment,
            coach_quality,
            training_intensity,
            facilities_quality,
            &player.hidden,
        );
        
        // Apply training effects to attributes
        self.apply_training_effects(player, manager_assigned_focus, effectiveness);
        
        // Calculate fatigue from training
        let fatigue_increase = self.calculate_fatigue_increase(manager_assigned_focus, training_intensity);
        player.fatigue = (player.fatigue + fatigue_increase).min(100.0);
        
        // Calculate morale effect based on alignment
        let morale_change = self.calculate_alignment_morale_effect(alignment, &player.hidden);
        player.morale = (player.morale + morale_change).clamp(0.0, 100.0);
        
        // Calculate injury risk
        let injury_risk = self.calculate_injury_risk(training_intensity, player.fatigue, &player.hidden);
        
        TrainingResult {
            focus: manager_assigned_focus,
            effectiveness,
            alignment,
            morale_change,
            fatigue_increase,
            injury_risk,
        }
    }

    /// Calculates how well the assigned focus aligns with player preference
    fn calculate_focus_alignment(
        &self,
        assigned_focus: TrainingFocus,
        player_preference: Option<TrainingFocus>,
    ) -> f32 {
        match player_preference {
            Some(pref) if pref == assigned_focus => 1.0,  // Perfect alignment
            Some(pref) if self.are_complementary_focuses(assigned_focus, pref) => 0.7,  // Complementary
            Some(_) => 0.3,  // Misaligned
            None => 0.5,  // No preference specified
        }
    }

    /// Checks if two training focuses are complementary
    fn are_complementary_focuses(&self, focus1: TrainingFocus, focus2: TrainingFocus) -> bool {
        match (focus1, focus2) {
            (TrainingFocus::Technical, TrainingFocus::Tactical) => true,
            (TrainingFocus::Tactical, TrainingFocus::Technical) => true,
            (TrainingFocus::Physical, TrainingFocus::Mental) => true,
            (TrainingFocus::Mental, TrainingFocus::Physical) => true,
            _ => false,
        }
    }

    /// Calculates overall training effectiveness
    fn calculate_training_effectiveness(
        &self,
        focus: TrainingFocus,
        alignment: f32,
        coach_quality: f32,
        intensity: f32,
        facilities: f32,
        hidden_attributes: &crate::entities::HiddenAttributes,
    ) -> f32 {
        // Base effectiveness by focus type
        let base_effectiveness = match focus {
            TrainingFocus::Technical => 0.8,
            TrainingFocus::Physical => 0.9,
            TrainingFocus::Tactical => 0.7,
            TrainingFocus::Mental => 0.75,
            TrainingFocus::Rest => 0.0,  // No growth during rest
        };
        
        // Calculate combined multipliers
        let coach_multiplier = coach_quality / 50.0;  // Normalize to 0-2 scale (50 = baseline)
        let intensity_multiplier = intensity / 50.0;  // Normalize to 0-2 scale (50 = baseline)
        let facilities_multiplier = facilities / 50.0;  // Normalize to 0-2 scale (50 = baseline)
        let alignment_multiplier = 0.5 + (alignment * 0.5);  // 0.5 to 1.0 range
        let professionalism_multiplier = (hidden_attributes.professionalism as f32) / 100.0;
        
        base_effectiveness * 
        coach_multiplier * 
        intensity_multiplier * 
        facilities_multiplier * 
        alignment_multiplier * 
        professionalism_multiplier
    }

    /// Applies training effects to player attributes
    fn apply_training_effects(
        &self,
        player: &mut Player,
        focus: TrainingFocus,
        effectiveness: f32,
    ) {
        match focus {
            TrainingFocus::Technical => {
                self.improve_technical_attributes(player, effectiveness);
            },
            TrainingFocus::Physical => {
                self.improve_physical_attributes(player, effectiveness);
            },
            TrainingFocus::Tactical => {
                self.improve_mental_attributes(player, effectiveness * 0.7); // Tactical training mainly improves mental
            },
            TrainingFocus::Mental => {
                self.improve_mental_attributes(player, effectiveness);
            },
            TrainingFocus::Rest => {
                // Rest reduces fatigue and may have minor positive effects
                player.fatigue = (player.fatigue * 0.7).max(0.0);  // Reduce fatigue by 30%
            },
        }
    }

    /// Improves technical attributes based on training
    fn improve_technical_attributes(&self, player: &mut Player, effectiveness: f32) {
        // Apply improvement with diminishing returns
        let improvement = self.apply_diminishing_returns(effectiveness, player.technical.average());

        // Distribute improvement based on player's position and needs
        let distribution = self.get_technical_distribution(&player.primary_position);

        player.technical.dribbling = self.cap_attribute(
            player.technical.dribbling as f32 + improvement * distribution.dribbling
        ) as u8;
        player.technical.passing = self.cap_attribute(
            player.technical.passing as f32 + improvement * distribution.passing
        ) as u8;
        player.technical.shooting = self.cap_attribute(
            player.technical.shooting as f32 + improvement * distribution.shooting
        ) as u8;
        player.technical.first_touch = self.cap_attribute(
            player.technical.first_touch as f32 + improvement * distribution.first_touch
        ) as u8;
        player.technical.tackling = self.cap_attribute(
            player.technical.tackling as f32 + improvement * distribution.tackling
        ) as u8;
        player.technical.crossing = self.cap_attribute(
            player.technical.crossing as f32 + improvement * distribution.crossing
        ) as u8;
    }

    /// Improves physical attributes based on training
    fn improve_physical_attributes(&self, player: &mut Player, effectiveness: f32) {
        // Apply improvement with diminishing returns
        let improvement = self.apply_diminishing_returns(effectiveness, player.physical.average());

        // Distribute improvement based on player's position and needs
        let distribution = self.get_physical_distribution(&player.primary_position);

        player.physical.pace = self.cap_attribute(
            player.physical.pace as f32 + improvement * distribution.pace
        ) as u8;
        player.physical.stamina = self.cap_attribute(
            player.physical.stamina as f32 + improvement * distribution.stamina
        ) as u8;
        player.physical.strength = self.cap_attribute(
            player.physical.strength as f32 + improvement * distribution.strength
        ) as u8;
        player.physical.agility = self.cap_attribute(
            player.physical.agility as f32 + improvement * distribution.agility
        ) as u8;
        player.physical.jumping = self.cap_attribute(
            player.physical.jumping as f32 + improvement * distribution.jumping
        ) as u8;
    }

    /// Improves mental attributes based on training
    fn improve_mental_attributes(&self, player: &mut Player, effectiveness: f32) {
        // Apply improvement with diminishing returns
        let improvement = self.apply_diminishing_returns(effectiveness, player.mental.average());
        
        // Distribute improvement evenly across mental attributes
        player.mental.composure = self.cap_attribute(
            player.mental.composure as f32 + improvement * 0.17
        ) as u8;
        player.mental.vision = self.cap_attribute(
            player.mental.vision as f32 + improvement * 0.17
        ) as u8;
        player.mental.work_rate = self.cap_attribute(
            player.mental.work_rate as f32 + improvement * 0.16
        ) as u8;
        player.mental.determination = self.cap_attribute(
            player.mental.determination as f32 + improvement * 0.17
        ) as u8;
        player.mental.positioning = self.cap_attribute(
            player.mental.positioning as f32 + improvement * 0.17
        ) as u8;
        player.mental.teamwork = self.cap_attribute(
            player.mental.teamwork as f32 + improvement * 0.16
        ) as u8;
    }

    /// Calculates fatigue increase from training
    fn calculate_fatigue_increase(&self, focus: TrainingFocus, intensity: f32) -> f32 {
        let base_fatigue = match focus {
            TrainingFocus::Technical => 5.0,
            TrainingFocus::Physical => 15.0,  // Physical training is more tiring
            TrainingFocus::Tactical => 8.0,
            TrainingFocus::Mental => 6.0,
            TrainingFocus::Rest => -20.0,  // Rest reduces fatigue
        };
        
        // Intensity affects fatigue
        base_fatigue * (intensity / 50.0)  // Normalize intensity to 0-2 scale
    }

    /// Calculates morale effect based on training alignment
    fn calculate_alignment_morale_effect(&self, alignment: f32, hidden_attributes: &crate::entities::HiddenAttributes) -> f32 {
        // Alignment affects morale
        let alignment_effect = (alignment - 0.5) * 10.0;  // -5 to +5 range
        
        // Loyalty affects how much misalignment impacts morale
        let loyalty_modifier = (hidden_attributes.loyalty as f32) / 100.0;
        
        if alignment_effect < 0.0 {
            // Negative effect: Low loyalty -> High impact
            alignment_effect * (1.5 - loyalty_modifier)
        } else {
            // Positive effect: High loyalty -> High impact
            alignment_effect * loyalty_modifier
        }
    }

    /// Calculates injury risk from training
    fn calculate_injury_risk(&self, intensity: f32, fatigue: f32, hidden_attributes: &crate::entities::HiddenAttributes) -> f32 {
        // Base risk from intensity
        let intensity_risk = intensity / 100.0;
        
        // Fatigue increases injury risk
        let fatigue_risk = fatigue / 200.0;
        
        // Injury proneness affects risk
        let proneness_factor = (hidden_attributes.injury_proneness as f32) / 100.0;
        
        (intensity_risk + fatigue_risk) * proneness_factor
    }

    /// Applies diminishing returns to attribute improvements
    fn apply_diminishing_returns(&self, base_improvement: f32, current_average: f32) -> f32 {
        // Higher attributes grow more slowly
        let diminishing_factor = 1.0 - (current_average / 200.0); // As attributes approach 100, growth slows
        base_improvement * diminishing_factor.max(0.1) // Ensure minimum growth
    }

    /// Caps an attribute value between 1 and 100
    fn cap_attribute(&self, value: f32) -> f32 {
        value.max(1.0).min(100.0)
    }

    /// Gets technical attribute distribution based on position
    fn get_technical_distribution(&self, position: &crate::entities::Position) -> TechnicalAttributeDistribution {
        match position {
            crate::entities::Position::GK => TechnicalAttributeDistribution {
                dribbling: 0.05,
                passing: 0.2,
                shooting: 0.05,
                first_touch: 0.15,
                tackling: 0.1,
                crossing: 0.05,
            },
            crate::entities::Position::CB => TechnicalAttributeDistribution {
                dribbling: 0.08,
                passing: 0.25,
                shooting: 0.12,
                first_touch: 0.18,
                tackling: 0.25,
                crossing: 0.05,
            },
            crate::entities::Position::FB => TechnicalAttributeDistribution {
                dribbling: 0.12,
                passing: 0.22,
                shooting: 0.1,
                first_touch: 0.15,
                tackling: 0.25,
                crossing: 0.16,
            },
            crate::entities::Position::DM => TechnicalAttributeDistribution {
                dribbling: 0.1,
                passing: 0.28,
                shooting: 0.12,
                first_touch: 0.2,
                tackling: 0.22,
                crossing: 0.08,
            },
            crate::entities::Position::CM => TechnicalAttributeDistribution {
                dribbling: 0.15,
                passing: 0.3,
                shooting: 0.15,
                first_touch: 0.18,
                tackling: 0.15,
                crossing: 0.07,
            },
            crate::entities::Position::RM | crate::entities::Position::LM => TechnicalAttributeDistribution {
                dribbling: 0.22,
                passing: 0.22,
                shooting: 0.18,
                first_touch: 0.15,
                tackling: 0.1,
                crossing: 0.13,
            },
            crate::entities::Position::RW | crate::entities::Position::LW => TechnicalAttributeDistribution {
                dribbling: 0.25,
                passing: 0.18,
                shooting: 0.2,
                first_touch: 0.15,
                tackling: 0.08,
                crossing: 0.14,
            },
            crate::entities::Position::CF | crate::entities::Position::SS => TechnicalAttributeDistribution {
                dribbling: 0.2,
                passing: 0.15,
                shooting: 0.28,
                first_touch: 0.18,
                tackling: 0.05,
                crossing: 0.14,
            },
            &crate::entities::Position::RB => TechnicalAttributeDistribution {
                dribbling: 0.12,
                passing: 0.22,
                shooting: 0.1,
                first_touch: 0.15,
                tackling: 0.25,
                crossing: 0.16,
            },
            &crate::entities::Position::LB => TechnicalAttributeDistribution {
                dribbling: 0.12,
                passing: 0.22,
                shooting: 0.1,
                first_touch: 0.15,
                tackling: 0.25,
                crossing: 0.16,
            },
            &crate::entities::Position::AM => TechnicalAttributeDistribution {
                dribbling: 0.18,
                passing: 0.28,
                shooting: 0.2,
                first_touch: 0.18,
                tackling: 0.12,
                crossing: 0.16,
            },
        }
    }

    /// Gets physical attribute distribution based on position
    fn get_physical_distribution(&self, position: &crate::entities::Position) -> PhysicalAttributeDistribution {
        match position {
            crate::entities::Position::GK => PhysicalAttributeDistribution {
                pace: 0.1,
                stamina: 0.15,
                strength: 0.25,
                agility: 0.2,
                jumping: 0.3,
            },
            crate::entities::Position::CB => PhysicalAttributeDistribution {
                pace: 0.15,
                stamina: 0.2,
                strength: 0.3,
                agility: 0.15,
                jumping: 0.2,
            },
            crate::entities::Position::FB => PhysicalAttributeDistribution {
                pace: 0.25,
                stamina: 0.25,
                strength: 0.15,
                agility: 0.2,
                jumping: 0.15,
            },
            crate::entities::Position::DM => PhysicalAttributeDistribution {
                pace: 0.18,
                stamina: 0.3,
                strength: 0.2,
                agility: 0.17,
                jumping: 0.15,
            },
            crate::entities::Position::CM => PhysicalAttributeDistribution {
                pace: 0.2,
                stamina: 0.3,
                strength: 0.15,
                agility: 0.2,
                jumping: 0.15,
            },
            crate::entities::Position::RM | crate::entities::Position::LM => PhysicalAttributeDistribution {
                pace: 0.28,
                stamina: 0.25,
                strength: 0.12,
                agility: 0.25,
                jumping: 0.1,
            },
            crate::entities::Position::RW | crate::entities::Position::LW => PhysicalAttributeDistribution {
                pace: 0.35,
                stamina: 0.22,
                strength: 0.1,
                agility: 0.25,
                jumping: 0.08,
            },
            crate::entities::Position::CF | crate::entities::Position::SS => PhysicalAttributeDistribution {
                pace: 0.3,
                stamina: 0.2,
                strength: 0.22,
                agility: 0.18,
                jumping: 0.1,
            },
            &crate::entities::Position::RB => PhysicalAttributeDistribution {
                pace: 0.25,
                stamina: 0.25,
                strength: 0.15,
                agility: 0.2,
                jumping: 0.15,
            },
            &crate::entities::Position::LB => PhysicalAttributeDistribution {
                pace: 0.25,
                stamina: 0.25,
                strength: 0.15,
                agility: 0.2,
                jumping: 0.15,
            },
            &crate::entities::Position::AM => PhysicalAttributeDistribution {
                pace: 0.28,
                stamina: 0.25,
                strength: 0.12,
                agility: 0.25,
                jumping: 0.1,
            },
        }
    }
}

/// Training focus options
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrainingFocus {
    Technical,
    Physical,
    Tactical,
    Mental,
    Rest,
}

/// Result of a training session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingResult {
    pub focus: TrainingFocus,
    pub effectiveness: f32,
    pub alignment: f32,  // How well the focus matched player preference
    pub morale_change: f32,
    pub fatigue_increase: f32,
    pub injury_risk: f32,
}

/// Distribution of training improvements across technical attributes
#[derive(Debug, Clone)]
struct TechnicalAttributeDistribution {
    dribbling: f32,
    passing: f32,
    shooting: f32,
    first_touch: f32,
    tackling: f32,
    crossing: f32,
}

/// Distribution of training improvements across physical attributes
#[derive(Debug, Clone)]
struct PhysicalAttributeDistribution {
    pace: f32,
    stamina: f32,
    strength: f32,
    agility: f32,
    jumping: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{Player, Position, Foot, CareerStats, Contract, SquadRole, HiddenAttributes};
    use chrono::NaiveDate;

    #[test]
    fn test_focus_alignment() {
        let system = TrainingSystem::new();
        
        // Perfect alignment
        let alignment = system.calculate_focus_alignment(
            TrainingFocus::Technical,
            Some(TrainingFocus::Technical)
        );
        assert_eq!(alignment, 1.0);
        
        // Misaligned
        let alignment = system.calculate_focus_alignment(
            TrainingFocus::Technical,
            Some(TrainingFocus::Physical)
        );
        assert_eq!(alignment, 0.3);
        
        // No preference
        let alignment = system.calculate_focus_alignment(
            TrainingFocus::Technical,
            None
        );
        assert_eq!(alignment, 0.5);
    }

    #[test]
    fn test_training_effectiveness() {
        let system = TrainingSystem::new();
        
        let hidden = HiddenAttributes {
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
        
        // Test high effectiveness scenario
        let effectiveness = system.calculate_training_effectiveness(
            TrainingFocus::Technical,
            1.0,  // Perfect alignment
            80.0, // Good coach
            70.0, // High intensity
            85.0, // Good facilities
            &hidden,
        );
        
        assert!(effectiveness > 1.0);  // Should be greater than base
        
        // Test low effectiveness scenario
        let effectiveness = system.calculate_training_effectiveness(
            TrainingFocus::Physical,
            0.0,  // No alignment
            30.0, // Poor coach
            40.0, // Low intensity
            45.0, // Poor facilities
            &hidden,
        );
        
        assert!(effectiveness < 1.0);  // Should be less than base
    }

    #[test]
    fn test_fatigue_calculation() {
        let system = TrainingSystem::new();
        
        // Physical training should increase fatigue more than technical
        let physical_fatigue = system.calculate_fatigue_increase(TrainingFocus::Physical, 70.0);
        let technical_fatigue = system.calculate_fatigue_increase(TrainingFocus::Technical, 70.0);
        
        assert!(physical_fatigue > technical_fatigue);
        
        // Rest should decrease fatigue
        let rest_fatigue = system.calculate_fatigue_increase(TrainingFocus::Rest, 50.0);
        assert!(rest_fatigue < 0.0);
    }

    #[test]
    fn test_alignment_morale_effect() {
        let system = TrainingSystem::new();
        
        let hidden = HiddenAttributes {
            injury_proneness: 20,
            consistency: 70,
            big_match_temperament: 80,
            professionalism: 90,
            potential_ceiling: 85,
            versatility: 75,
            ambition: 80,
            loyalty: 80,  // High loyalty
            ego: 70,
        };
        
        // Perfect alignment with high loyalty should have positive effect
        let effect = system.calculate_alignment_morale_effect(1.0, &hidden);
        assert!(effect > 0.0);
        
        // Poor alignment with high loyalty should have smaller negative effect
        let effect = system.calculate_alignment_morale_effect(0.0, &hidden);
        assert!(effect > -5.0);  // Should be less negative than low loyalty case
        
        // Test with low loyalty
        let low_loyalty_hidden = HiddenAttributes { loyalty: 20, ..hidden };
        let effect = system.calculate_alignment_morale_effect(0.0, &low_loyalty_hidden);
        assert!(effect < -3.0);  // Should be more negative
    }

    #[test]
    fn test_diminishing_returns() {
        let system = TrainingSystem::new();
        
        // Low attribute should have higher returns
        let high_return = system.apply_diminishing_returns(1.0, 20.0);
        assert!(high_return > 0.8);  // Should preserve most of the improvement
        
        // High attribute should have lower returns
        let low_return = system.apply_diminishing_returns(1.0, 90.0);
        assert!(low_return < 0.6);  // Should reduce improvement significantly
    }
}