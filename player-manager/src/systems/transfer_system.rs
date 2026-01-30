// src/systems/transfer_system.rs
use serde::{Deserialize, Serialize};
// Transfer system cleanup
// Removed unused HashMap
use uuid::Uuid;
use chrono::Datelike;

use crate::entities::{Player, Team, Contract};

/// The TransferEngine manages transfer interest, offers, and negotiations
/// It generates transfer interest based on player performance and club needs
pub struct TransferEngine;

impl TransferEngine {
    /// Creates a new TransferEngine instance
    pub fn new() -> Self {
        TransferEngine
    }

    /// Evaluates all clubs to see if they have interest in a player
    pub fn evaluate_transfer_interest(
        &self,
        player: &Player,
        all_teams: &[Team],
        current_club_id: Uuid,
    ) -> Vec<TransferInterest> {
        let mut interests = Vec::new();
        
        for team in all_teams {
            // Skip current club
            if team.id == current_club_id {
                continue;
            }
            
            // Calculate interest score
            let interest_score = self.calculate_transfer_interest_score(player, team);
            
            // Only add if interest is above threshold
            if interest_score > 30.0 {
                let interest_level = self.determine_interest_level(interest_score);
                
                interests.push(TransferInterest {
                    club_id: team.id,
                    interest_level,
                    last_evaluation_date: chrono::Utc::now(),
                    evaluation_score: interest_score,
                });
            }
        }
        
        interests
    }

    /// Calculates the transfer interest score for a club in a player
    fn calculate_transfer_interest_score(&self, player: &Player, team: &Team) -> f32 {
        // Base score from player attributes
        let ability_score = self.calculate_player_ability_score(player);
        let potential_score = (player.hidden.potential_ceiling as f32) / 2.0;  // 0-50 scale
        let form_score = player.form * 0.5;  // 0-50 scale
        let reputation_score = player.international_reputation * 0.7;  // 0-70 scale (international matters more)
        
        // Age factor (younger players more attractive)
        let age_factor = self.calculate_age_factor(player.age);
        
        // Positional need factor
        let positional_need = self.calculate_positional_need(player.primary_position, team);
        
        // Financial capacity factor
        let financial_factor = self.calculate_financial_capacity(team);
        
        // Calculate base interest score
        let mut interest_score = ability_score + potential_score + form_score + reputation_score;
        
        // Apply modifiers
        interest_score *= age_factor;
        interest_score *= 1.0 + (team.reputation / 200.0);  // Higher club reputation increases interest
        interest_score *= 1.0 + (positional_need / 100.0);  // Positional need increases interest
        interest_score *= financial_factor;
        
        interest_score
    }

    /// Calculates a player's overall ability score for transfer purposes
    fn calculate_player_ability_score(&self, player: &Player) -> f32 {
        // Weighted average of attributes
        let technical_weight = 0.35;
        let physical_weight = 0.25;
        let mental_weight = 0.40;
        
        let technical_avg = player.technical.average();
        let physical_avg = player.physical.average();
        let mental_avg = player.mental.average();
        
        (technical_avg * technical_weight) + 
        (physical_avg * physical_weight) + 
        (mental_avg * mental_weight)
    }

    /// Calculates age factor for transfer interest
    fn calculate_age_factor(&self, age: u8) -> f32 {
        match age {
            15..=21 => 1.3,  // High potential, high interest
            22..=25 => 1.1,  // Prime development years
            26..=29 => 1.0,  // Peak years
            30..=32 => 0.8,  // Beginning decline
            33..=35 => 0.6,  // Significant decline
            _ => 0.4,         // Veteran years
        }
    }

    /// Calculates how much a team needs a specific position
    fn calculate_positional_need(&self, position: crate::entities::Position, _team: &Team) -> f32 {
        // This is a simplified version - in a real implementation, 
        // this would analyze the team's current squad composition
        // and determine gaps in positions
        
        // For now, we'll return a base value based on position importance
        match position {
            crate::entities::Position::GK => 10.0,  // Goalkeepers are important
            crate::entities::Position::CB => 15.0,  // Defense is important
            crate::entities::Position::FB => 12.0,  // Fullbacks are important
            crate::entities::Position::DM => 14.0,  // Defensive midfielders are important
            crate::entities::Position::CM => 16.0,  // Central midfielders are very important
            crate::entities::Position::RM | crate::entities::Position::LM => 13.0,  // Wide midfielders
            crate::entities::Position::RW | crate::entities::Position::LW => 15.0,  // Wingers are important
            crate::entities::Position::CF | crate::entities::Position::SS => 18.0,  // Forwards are very important
            crate::entities::Position::RB => 12.0,  // Right back important
            crate::entities::Position::LB => 12.0,  // Left back important
            crate::entities::Position::AM => 16.0,  // Attacking midfielder very important
        }
    }

    /// Calculates financial capacity factor
    fn calculate_financial_capacity(&self, team: &Team) -> f32 {
        // Higher financial power allows for more interest
        team.financial_power / 50.0  // Normalize to around 1.0 for average teams
    }

    /// Determines interest level based on score
    fn determine_interest_level(&self, score: f32) -> InterestLevel {
        if score >= 80.0 {
            InterestLevel::OfficialOffer
        } else if score >= 70.0 {
            InterestLevel::PreparingOffer
        } else if score >= 60.0 {
            InterestLevel::Shortlisted
        } else if score >= 50.0 {
            InterestLevel::Scouting
        } else {
            InterestLevel::Monitoring
        }
    }

    /// Generates a transfer offer for a player
    pub fn generate_transfer_offer(
        &self,
        player: &Player,
        interested_club: &Team,
        current_contract: &Contract,
    ) -> TransferOffer {
        // Calculate transfer fee based on player value
        let transfer_fee = self.calculate_transfer_fee(player, interested_club, current_contract);
        
        // Calculate wage offer based on player's ability and club's financial power
        let offered_wage = self.calculate_wage_offer(player, interested_club);
        
        // Calculate contract length based on age and club's youth focus
        let contract_length = self.calculate_contract_length(player.age, interested_club.youth_focus);
        
        TransferOffer {
            id: Uuid::new_v4(),
            buying_club_id: interested_club.id,
            target_player_id: player.id,
            offered_wage,
            contract_length_years: contract_length,
            transfer_fee,
            offer_date: chrono::Utc::now(),
            expiry_date: chrono::Utc::now() + chrono::Duration::days(14), // 2 weeks to respond
        }
    }

    /// Calculates transfer fee based on player value
    fn calculate_transfer_fee(&self, player: &Player, interested_club: &Team, _current_contract: &Contract) -> Option<f32> {
        // Base value from player attributes and performance
        let base_value = self.calculate_player_market_value(player);
        
        // Apply club reputation multiplier
        let reputation_multiplier = interested_club.reputation / 50.0;  // Normalize to ~1.0 for average clubs
        
        // Apply age factor (younger players cost more)
        let age_factor = match player.age {
            18..=24 => 1.2,
            25..=28 => 1.0,
            29..=31 => 0.8,
            32..=34 => 0.6,
            _ => 0.4,
        };
        
        // Apply performance factor
        let performance_factor = player.form / 50.0;  // Normalize form to 0-2 scale
        
        // Calculate base fee
        let base_fee = base_value * reputation_multiplier * age_factor * performance_factor;
        
        // Apply financial capacity constraint
        if base_fee > interested_club.financial_power * 1000.0 {
            // Club can't afford, reduce to max they can pay
            Some(interested_club.financial_power * 1000.0 * 0.8)  // 80% of capacity
        } else {
            Some(base_fee)
        }
    }

    /// Calculates player's market value
    fn calculate_player_market_value(&self, player: &Player) -> f32 {
        // Combine various factors to determine market value
        let ability_value = self.calculate_player_ability_score(player) * 100.0;
        let reputation_value = player.international_reputation * 50.0;
        let form_value = player.form * 30.0;
        let potential_value = (player.hidden.potential_ceiling as f32) * 20.0;
        
        ability_value + reputation_value + form_value + potential_value
    }

    /// Calculates wage offer based on player ability and club finances
    fn calculate_wage_offer(&self, player: &Player, interested_club: &Team) -> f32 {
        // Base wage from player ability
        let base_wage = self.calculate_player_ability_score(player) * 1000.0;
        
        // Apply club financial power multiplier
        let financial_multiplier = interested_club.financial_power / 50.0;
        
        // Apply reputation premium
        let reputation_multiplier = 1.0 + (player.international_reputation / 200.0);
        
        base_wage * financial_multiplier * reputation_multiplier
    }

    /// Calculates contract length based on age and club preferences
    fn calculate_contract_length(&self, age: u8, youth_focus: f32) -> u8 {
        match age {
            15..=21 => {
                // Young players: longer contracts for clubs with high youth focus
                if youth_focus > 70.0 {
                    5  // 5-year deal for youth-focused clubs
                } else {
                    4  // 4-year deal for others
                }
            },
            22..=25 => 4,  // Prime years: 4-year deal
            26..=28 => 3,  // Peak decline: 3-year deal
            29..=31 => 2,  // Later career: 2-year deal
            _ => 1,         // Veterans: 1-year deal
        }
    }

    /// Processes a player's response to a transfer offer
    pub fn process_player_response(
        &self,
        player: &Player,
        offer: &TransferOffer,
        response: PlayerResponse,
    ) -> TransferOutcome {
        match response {
            PlayerResponse::Interested => {
                // Check if offer meets player's expectations
                if self.offer_meets_expectations(player, offer) {
                    TransferOutcome::NegotiationStarted
                } else {
                    TransferOutcome::CounterOfferSuggested
                }
            },
            PlayerResponse::NotInterested => TransferOutcome::OfferRejected,
            PlayerResponse::LetAgentHandle => TransferOutcome::AgentActionRequired,
        }
    }

    /// Checks if an offer meets the player's expectations
    fn offer_meets_expectations(&self, player: &Player, offer: &TransferOffer) -> bool {
        // This would be more complex in a real implementation
        // considering player's ambition, loyalty, relationships, etc.
        
        // Simple heuristic: check if offered wage is within 20% of player's perceived value
        let player_perceived_value = self.calculate_player_perceived_value(player);
        let wage_ratio = offer.offered_wage / player_perceived_value;
        
        wage_ratio >= 0.8 && wage_ratio <= 1.2
    }

    /// Calculates player's perceived value (what they think they're worth)
    fn calculate_player_perceived_value(&self, player: &Player) -> f32 {
        // Combine reputation, form, and ego to determine perceived value
        let reputation_factor = player.international_reputation / 50.0;  // Normalize to 0-2 scale
        let form_factor = player.form / 50.0;  // Normalize to 0-2 scale
        let ego_factor = (player.hidden.ego as f32) / 50.0;  // Normalize to 0-2 scale
        
        // Base value from attributes
        let base_value = self.calculate_player_ability_score(player) * 1000.0;
        
        base_value * reputation_factor * form_factor * ego_factor
    }

    /// Processes contract negotiations
    pub fn negotiate_contract(
        &self,
        _player: &Player,
        offer: &TransferOffer,
        negotiation_preferences: &NegotiationPreferences,
    ) -> ContractNegotiationResult {
        // Simulate negotiation process
        let mut final_offer = offer.clone();
        
        // Apply negotiation preferences
        if negotiation_preferences.prefer_longer_contract {
            final_offer.contract_length_years = final_offer.contract_length_years.min(6);
        }
        
        if negotiation_preferences.prefer_higher_wage {
            final_offer.offered_wage *= 1.05;  // 5% increase request
        }
        
        // Check if club accepts modified terms
        let club_acceptance = self.club_acceptance_probability(&final_offer);
        
        if club_acceptance > 0.5 {
            ContractNegotiationResult::Accepted(final_offer)
        } else {
            ContractNegotiationResult::Rejected
        }
    }

    /// Calculates probability that club accepts modified terms
    fn club_acceptance_probability(&self, _offer: &TransferOffer) -> f32 {
        // Simplified acceptance probability
        // In reality, this would consider club's budget, priorities, etc.
        0.7  // 70% chance for now
    }

    /// Checks if a player's contract is expiring soon
    pub fn is_contract_expiring_soon(&self, contract: &Contract, months_threshold: u32) -> bool {
        let today = chrono::Utc::now().date_naive();
        let expiry_date = contract.contract_end_date;
        
        // Calculate difference in months
        let month_diff = ((expiry_date.year() - today.year()) * 12) as i32 + 
                         (expiry_date.month() as i32 - today.month() as i32);
        
        month_diff <= months_threshold as i32 && month_diff >= 0
    }

    /// Generates contract renewal offer for current club
    pub fn generate_contract_renewal(
        &self,
        player: &Player,
        current_club: &Team,
        current_contract: &Contract,
    ) -> TransferOffer {
        // Calculate improved terms based on performance
        let performance_improvement = (player.form - 6.5).max(0.0) * 0.1;  // Positive form above average
        
        let new_wage = current_contract.wage * (1.0 + performance_improvement);
        let new_length = if player.age < 28 {
            current_contract.length_years.min(5)  // Extend for younger players
        } else {
            current_contract.length_years.min(3)  // Shorter for older players
        };
        
        TransferOffer {
            id: Uuid::new_v4(),
            buying_club_id: current_club.id,
            target_player_id: player.id,
            offered_wage: new_wage,
            contract_length_years: new_length,
            transfer_fee: None,  // No fee for renewals
            offer_date: chrono::Utc::now(),
            expiry_date: chrono::Utc::now() + chrono::Duration::days(30), // More time for renewals
        }
    }
}

/// Transfer interest level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InterestLevel {
    Monitoring,
    Scouting,
    Shortlisted,
    PreparingOffer,
    OfficialOffer,
}

/// Transfer interest record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferInterest {
    pub club_id: Uuid,
    pub interest_level: InterestLevel,
    pub last_evaluation_date: chrono::DateTime<chrono::Utc>,
    pub evaluation_score: f32,
}

/// Transfer offer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOffer {
    pub id: Uuid,
    pub buying_club_id: Uuid,
    pub target_player_id: Uuid,
    pub offered_wage: f32,
    pub contract_length_years: u8,
    pub transfer_fee: Option<f32>,
    pub offer_date: chrono::DateTime<chrono::Utc>,
    pub expiry_date: chrono::DateTime<chrono::Utc>,
}

/// Player's response to transfer offer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerResponse {
    Interested,
    NotInterested,
    LetAgentHandle,
}

/// Result of transfer process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferOutcome {
    NegotiationStarted,
    CounterOfferSuggested,
    OfferRejected,
    AgentActionRequired,
}

/// Negotiation preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiationPreferences {
    pub prefer_longer_contract: bool,
    pub prefer_higher_wage: bool,
    pub prefer_prestige_club: bool,
    pub prefer_playing_time: bool,
}

/// Result of contract negotiation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractNegotiationResult {
    Accepted(TransferOffer),
    Rejected,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{Position, Foot, CareerStats, SquadRole, HiddenAttributes};
    use crate::systems::social_system::ManagerProfile;
    use chrono::NaiveDate;

    #[test]
    fn test_calculate_player_ability_score() {
        let engine = TransferEngine::new();
        
        let player = create_test_player();
        
        let score = engine.calculate_player_ability_score(&player);
        
        // The score should be based on the player's attributes
        assert!(score > 70.0 && score < 90.0);
    }

    #[test]
    fn test_age_factor() {
        let engine = TransferEngine::new();
        
        assert_eq!(engine.calculate_age_factor(20), 1.3);  // Young, high factor
        assert_eq!(engine.calculate_age_factor(27), 1.0);  // Prime years
        assert_eq!(engine.calculate_age_factor(34), 0.6);  // Older, lower factor
    }

    #[test]
    fn test_contract_expiring_soon() {
        let engine = TransferEngine::new();
        
        let mut contract = create_test_contract();
        // Set contract to expire in 2 months
        let today = chrono::Utc::now().date_naive();
        contract.contract_end_date = today + chrono::Duration::days(60);
        
        assert!(engine.is_contract_expiring_soon(&contract, 3));  // Should be true within 3 months
        assert!(!engine.is_contract_expiring_soon(&contract, 1)); // Should be false within 1 month
    }

    #[test]
    fn test_transfer_interest_score() {
        let engine = TransferEngine::new();
        
        let player = create_test_player();
        let team = create_test_team();
        
        let score = engine.calculate_transfer_interest_score(&player, &team);
        
        // The score should be reasonable
        assert!(score > 0.0);
    }

    #[test]
    fn test_determine_interest_level() {
        let engine = TransferEngine::new();
        
        assert_eq!(engine.determine_interest_level(85.0), InterestLevel::OfficialOffer);
        assert_eq!(engine.determine_interest_level(75.0), InterestLevel::PreparingOffer);
        assert_eq!(engine.determine_interest_level(65.0), InterestLevel::Shortlisted);
        assert_eq!(engine.determine_interest_level(55.0), InterestLevel::Scouting);
        assert_eq!(engine.determine_interest_level(45.0), InterestLevel::Monitoring);
    }

    // Helper functions for tests
    fn create_test_player() -> Player {
        Player {
            id: Uuid::new_v4(),
            name: "Test Player".to_string(),
            age: 25,
            birth_date: NaiveDate::from_ymd_opt(1998, 1, 1).unwrap(),
            nationality: "Country".to_string(),
            height: 180,
            weight: 75,
            preferred_foot: Foot::Right,
            primary_position: Position::CM,
            secondary_positions: vec![],
            technical: crate::entities::TechnicalAttributes {
                dribbling: 75,
                passing: 80,
                shooting: 70,
                first_touch: 78,
                tackling: 72,
                crossing: 65,
            },
            physical: crate::entities::PhysicalAttributes {
                pace: 70,
                stamina: 85,
                strength: 75,
                agility: 72,
                jumping: 68,
            },
            mental: crate::entities::MentalAttributes {
                composure: 80,
                vision: 85,
                work_rate: 75,
                determination: 82,
                positioning: 78,
                teamwork: 80,
            },
            hidden: HiddenAttributes {
                injury_proneness: 20,
                consistency: 70,
                big_match_temperament: 80,
                professionalism: 90,
                potential_ceiling: 85,
                versatility: 75,
                ambition: 80,
                loyalty: 60,
                ego: 70,
            },
            fitness: 90.0,
            fatigue: 10.0,
            form: 7.5,
            morale: 75.0,
            sharpness: 80.0,
            local_reputation: 65.0,
            international_reputation: 40.0,
            contract: create_test_contract(),
            career_stats: CareerStats {
                seasons_played: 3,
                total_appearances: 50,
                total_goals: 10,
                total_assists: 8,
                total_yellow_cards: 15,
                total_red_cards: 1,
                average_rating: 7.2,
                highest_rating: 9.0,
                season_stats: vec![],
                awards: vec![],
                trophies: vec![],
            },
            relationships: HashMap::new(),
            injury_status: None,
            form_history: vec![7.0, 7.5, 8.0, 6.8, 7.2],
            tutorial_state: HashMap::new(),
        }
    }

    fn create_test_contract() -> Contract {
        Contract {
            club_id: Uuid::new_v4(),
            wage: 50000.0,
            length_years: 3,
            squad_role: SquadRole::FirstTeam,
            release_clause: None,
            performance_bonuses: vec![],
            contract_end_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            league_strength: 75.0,
        }
    }

    fn create_test_team() -> Team {
        Team {
            id: Uuid::new_v4(),
            name: "Test Team".to_string(),
            country: "Test Country".to_string(),
            city: "Test City".to_string(),
            reputation: 75.0,
            finances: crate::entities::Finances {
                balance: 1000000.0,
                weekly_wage_bill: 50000.0,
                revenue_per_week: 100000.0,
                debt: 0.0,
            },
            squad: vec![],
            staff: vec![],
            youth_academy_level: 5,
            facilities: crate::entities::Facilities {
                training_ground_quality: 7,
                stadium_capacity: 20000,
                stadium_quality: 6,
                youth_facilities: 8,
            },
            financial_power: 75.0,
            youth_focus: 60.0,
            facilities_quality: 70.0,
            medical_quality: 80.0,
            tactical_identity: "Possession".to_string(),
        }
    }
}