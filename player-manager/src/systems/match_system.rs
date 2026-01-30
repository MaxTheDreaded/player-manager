use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::entities::{
    Match, MatchEvent, Player, EventType, Position, PitchZone, 
    MatchHalf, PlayerMatchStats, MatchLineup
};

/// The MatchEngine simulates football matches and produces player ratings
/// It generates match events based on player attributes, form, morale, and other factors
pub struct MatchEngine {
    rng: rand::rngs::ThreadRng,
}

impl MatchEngine {
    /// Creates a new MatchEngine instance
    pub fn new() -> Self {
        MatchEngine {
            rng: rand::thread_rng(),
        }
    }

    /// Simulates a complete match and returns the updated match object
    pub fn simulate_match(
        &mut self,
        mut game_match: Match,
        home_players: &[Player],
        away_players: &[Player],
        home_lineup: &MatchLineup,
        away_lineup: &MatchLineup,
    ) -> Match {
        // Initialize match state
        let mut match_state = MatchState::new(home_players, away_players, home_lineup, away_lineup);
        
        // Simulate match in time slices (minutes)
        for minute in 0..90 {
            let events_this_minute = self.generate_events_for_minute(&mut match_state, minute);
            game_match.events.extend(events_this_minute);
        }
        
        // Handle extra time if needed (simplified)
        if game_match.requires_extra_time() {
            for minute in 90..120 {
                let events_this_minute = self.generate_events_for_minute(&mut match_state, minute);
                game_match.events.extend(events_this_minute);
            }
        }
        
        // Calculate final ratings for all players
        let ratings = self.calculate_player_ratings(&game_match, &match_state);
        game_match.player_ratings = ratings;
        
        // Update player stats
        self.update_player_match_stats(&mut game_match, &match_state);
        
        game_match.status = crate::entities::MatchStatus::Finished;
        game_match
    }

    /// Generates events for a specific minute of the match
    fn generate_events_for_minute(&mut self, match_state: &mut MatchState, minute: u8) -> Vec<MatchEvent> {
        let mut events = Vec::new();
        
        // Determine which team is more likely to have possession based on tactics
        let home_possession_chance = match_state.home_tactical_balance;
        let is_home_possession = self.rng.gen::<f32>() < home_possession_chance;
        
        // Determine which team is involved in the action
        let team_id = if is_home_possession {
            match_state.home_team_id
        } else {
            match_state.away_team_id
        };
        
        // Select a player from the possessing team
        let player_id = self.select_player_for_action(match_state, team_id, minute);
        
        // Generate an action based on the player's position and attributes
        if let Some(action) = self.generate_action_for_player(match_state, player_id, minute) {
            events.push(action);
        }
        
        // Occasionally generate defensive actions from the opposing team
        if self.rng.gen::<f32>() < 0.3 {  // 30% chance of defensive action
            let defending_team_id = if team_id == match_state.home_team_id {
                match_state.away_team_id
            } else {
                match_state.home_team_id
            };
            
            let defending_player_id = self.select_player_for_defensive_action(match_state, defending_team_id, minute);
            if let Some(defensive_action) = self.generate_defensive_action(match_state, defending_player_id, minute) {
                events.push(defensive_action);
            }
        }
        
        events
    }

    /// Selects a player to participate in an action based on their position and involvement likelihood
    fn select_player_for_action(&mut self, match_state: &MatchState, team_id: Uuid, minute: u8) -> Uuid {
        // Get players from the team who are on the field
        let team_players = if team_id == match_state.home_team_id {
            &match_state.home_players
        } else {
            &match_state.away_players
        };
        
        // Weight selection based on position importance and player attributes
        let mut weighted_players = Vec::new();
        for player_ref in team_players {
            let player = &player_ref.player;
            let involvement_weight = self.calculate_player_involvement_weight(player, minute);
            weighted_players.push((player.id, involvement_weight));
        }
        
        // Select a player based on weights
        self.weighted_random_selection(&weighted_players)
    }

    /// Selects a player for a defensive action
    fn select_player_for_defensive_action(&mut self, match_state: &MatchState, team_id: Uuid, _minute: u8) -> Uuid {
        // Similar to offensive action but favor defensive players
        let team_players = if team_id == match_state.home_team_id {
            &match_state.home_players
        } else {
            &match_state.away_players
        };
        
        let mut weighted_players = Vec::new();
        for player_ref in team_players {
            let player = &player_ref.player;
            let defensive_weight = self.calculate_player_defensive_weight(player);
            weighted_players.push((player.id, defensive_weight));
        }
        
        self.weighted_random_selection(&weighted_players)
    }

    /// Calculates how likely a player is to be involved in an action
    fn calculate_player_involvement_weight(&self, player: &Player, _minute: u8) -> f32 {
        // Base weight on position and attributes
        let position_factor = match player.primary_position {
            Position::CF | Position::SS => 1.2,  // Forwards more involved in attacks
            Position::RW | Position::LW => 1.1,
            Position::CM => 1.0,
            Position::RM | Position::LM => 0.9,
            Position::CB => 0.7,
            Position::FB => 0.8,
            Position::DM => 0.85,
            Position::GK => 0.3,  // Goalkeepers less involved in attacks
            Position::RB => 0.7,
            Position::LB => 0.7,
            Position::AM => 0.9,
        };
        
        // Form and morale affect involvement
        let form_factor = player.form / 50.0;  // Normalize form (0-100) to 0-2 scale
        let morale_factor = player.morale / 50.0;  // Normalize morale (0-100) to 0-2 scale
        
        // Attribute factor based on technical abilities
        let technical_avg = player.technical.average() / 50.0;  // Normalize to 0-2 scale
        
        position_factor * form_factor * morale_factor * technical_avg
    }

    /// Calculates how likely a player is to be involved in defensive actions
    fn calculate_player_defensive_weight(&self, player: &Player) -> f32 {
        let position_factor = match player.primary_position {
            Position::CB => 1.3,   // Center backs most defensive
            Position::FB => 1.2,   // Full backs also defensive
            Position::DM => 1.1,   // Defensive midfielders
            Position::CM => 0.9,   // Central midfielders
            Position::RM | Position::LM => 0.8,
            Position::RW | Position::LW => 0.7,
            Position::CF | Position::SS => 0.5,  // Forwards least defensive
            Position::GK => 1.0,   // Goalkeepers defend but differently
            Position::RB => 1.2,   // Right back defensive
            Position::LB => 1.2,   // Left back defensive
            Position::AM => 0.7,   // Attacking midfielder less defensive
        };
        
        let tackling_ability = player.technical.tackling as f32 / 50.0;
        let positioning = player.mental.positioning as f32 / 50.0;
        
        position_factor * tackling_ability * positioning
    }

    /// Performs weighted random selection
    fn weighted_random_selection(&mut self, weighted_items: &[(Uuid, f32)]) -> Uuid {
        if weighted_items.is_empty() {
            return Uuid::nil();  // Return nil UUID if no items
        }
        
        // Calculate total weight
        let total_weight: f32 = weighted_items.iter().map(|(_, weight)| weight).sum();
        
        if total_weight == 0.0 {
            return weighted_items[0].0;  // Return first item if all weights are zero
        }
        
        // Generate a random value between 0 and total weight
        let random_value = self.rng.gen::<f32>() * total_weight;
        
        // Find the selected item
        let mut current_weight = 0.0;
        for &(item, weight) in weighted_items {
            current_weight += weight;
            if random_value <= current_weight {
                return item;
            }
        }
        
        // Fallback (shouldn't happen if weights are positive)
        weighted_items[weighted_items.len() - 1].0
    }

    /// Generates an action for a specific player
    fn generate_action_for_player(&mut self, match_state: &MatchState, player_id: Uuid, minute: u8) -> Option<MatchEvent> {
        // Find the player
        let player = match self.find_player_by_id(match_state, player_id) {
            Some(p) => p,
            None => return None,
        };
        
        // Determine action type based on position and game state
        let action_type = self.decide_action_type(player, match_state, minute);
        
        // Create the event with appropriate context
        let event = MatchEvent {
            id: Uuid::new_v4(),
            match_id: match_state.match_id,
            minute,
            half: if minute < 45 { MatchHalf::First } else { MatchHalf::Second },
            event_type: action_type.clone(),
            player_involved: player_id,
            secondary_player: self.select_secondary_player(match_state, player_id),
            pitch_zone: self.determine_pitch_zone(minute),
            success: self.determine_success_based_on_attributes(player, &action_type),
            base_impact: self.get_base_impact(&action_type),
            time_multiplier: self.calculate_time_multiplier(minute, match_state.score_difference),
            position_multiplier: self.calculate_position_multiplier(&action_type, player.primary_position),
            difficulty_multiplier: self.calculate_difficulty_multiplier(player, match_state),
            clutch_multiplier: self.calculate_clutch_multiplier(minute, match_state.score_difference, match_state.match_importance),
            total_impact_score: 0.0, // This will be calculated after all multipliers
            team_id: match_state.home_team_id, // Assuming home team for this example
            player_id: player_id,
            description: format!("Action by player {} at minute {}", player_id, minute),
            rating_impact: Some(0.0), // Placeholder value
        };
        
        // Calculate the total impact score
        let total_impact = event.base_impact * 
                          event.time_multiplier * 
                          event.position_multiplier * 
                          event.difficulty_multiplier * 
                          event.clutch_multiplier;
        
        Some(MatchEvent {
            total_impact_score: total_impact,
            ..event
        })
    }

    /// Generates a defensive action
    fn generate_defensive_action(&mut self, match_state: &MatchState, player_id: Uuid, minute: u8) -> Option<MatchEvent> {
        // Find the player
        let player = match self.find_player_by_id(match_state, player_id) {
            Some(p) => p,
            None => return None,
        };
        
        // Determine defensive action type
        let action_type = self.decide_defensive_action_type(player);
        
        // Create the event
        let event = MatchEvent {
            id: Uuid::new_v4(),
            match_id: match_state.match_id,
            minute,
            half: if minute < 45 { MatchHalf::First } else { MatchHalf::Second },
            event_type: action_type.clone(),
            player_involved: player_id,
            secondary_player: self.select_secondary_player(match_state, player_id),
            pitch_zone: self.determine_pitch_zone(minute),
            success: self.determine_success_based_on_attributes(player, &action_type),
            base_impact: self.get_base_impact(&action_type),
            time_multiplier: self.calculate_time_multiplier(minute, match_state.score_difference),
            position_multiplier: self.calculate_position_multiplier(&action_type, player.primary_position),
            difficulty_multiplier: self.calculate_difficulty_multiplier(player, match_state),
            clutch_multiplier: self.calculate_clutch_multiplier(minute, match_state.score_difference, match_state.match_importance),
            total_impact_score: 0.0,
            team_id: match_state.home_team_id, // Assuming home team for this example
            player_id: player_id,
            description: format!("Defensive action by player {} at minute {}", player_id, minute),
            rating_impact: Some(0.0), // Placeholder value
        };
        
        // Calculate the total impact score
        let total_impact = event.base_impact * 
                          event.time_multiplier * 
                          event.position_multiplier * 
                          event.difficulty_multiplier * 
                          event.clutch_multiplier;
        
        Some(MatchEvent {
            total_impact_score: total_impact,
            ..event
        })
    }

    /// Finds a player by ID in the match state
    fn find_player_by_id<'a>(&self, match_state: &'a MatchState, player_id: Uuid) -> Option<&'a Player> {
        // Check home team
        for player_ref in &match_state.home_players {
            if player_ref.player.id == player_id {
                return Some(&player_ref.player);
            }
        }
        
        // Check away team
        for player_ref in &match_state.away_players {
            if player_ref.player.id == player_id {
                return Some(&player_ref.player);
            }
        }
        
        None
    }

    /// Decides what type of action a player should take based on their position
    fn decide_action_type(&mut self, player: &Player, _match_state: &MatchState, _minute: u8) -> EventType {
        match player.primary_position {
            Position::GK => {
                // Goalkeeper actions
                let roll = self.rng.gen::<f32>();
                if roll < 0.6 {
                    EventType::ClaimCross
                } else if roll < 0.8 {
                    EventType::PunchClear
                } else if roll < 0.95 {
                    EventType::Save
                } else {
                    EventType::SweeperClearance
                }
            },
            Position::CB => {
                // Center back actions
                let roll = self.rng.gen::<f32>();
                if roll < 0.4 {
                    EventType::TackleWon
                } else if roll < 0.7 {
                    EventType::Interception
                } else if roll < 0.9 {
                    EventType::Clearance
                } else {
                    EventType::AerialDuelWon
                }
            },
            Position::FB => {
                // Full back actions
                let roll = self.rng.gen::<f32>();
                if roll < 0.3 {
                    EventType::CrossSuccess
                } else if roll < 0.6 {
                    EventType::TackleWon
                } else if roll < 0.85 {
                    EventType::PassSuccess
                } else {
                    EventType::DribbleSuccess
                }
            },
            Position::DM => {
                // Defensive midfielder actions
                let roll = self.rng.gen::<f32>();
                if roll < 0.4 {
                    EventType::TackleWon
                } else if roll < 0.75 {
                    EventType::Interception
                } else if roll < 0.95 {
                    EventType::PassSuccess
                } else {
                    EventType::DribbleSuccess
                }
            },
            Position::CM => {
                // Central midfielder actions
                let roll = self.rng.gen::<f32>();
                if roll < 0.25 {
                    EventType::KeyPass
                } else if roll < 0.5 {
                    EventType::PassSuccess
                } else if roll < 0.7 {
                    EventType::DribbleSuccess
                } else if roll < 0.9 {
                    EventType::TackleWon
                } else {
                    EventType::ThroughBall
                }
            },
            Position::RM | Position::LM => {
                // Wide midfielder actions
                let roll = self.rng.gen::<f32>();
                if roll < 0.3 {
                    EventType::CrossSuccess
                } else if roll < 0.55 {
                    EventType::KeyPass
                } else if roll < 0.8 {
                    EventType::DribbleSuccess
                } else {
                    EventType::PassSuccess
                }
            },
            Position::RW | Position::LW => {
                // Winger actions
                let roll = self.rng.gen::<f32>();
                if roll < 0.4 {
                    EventType::DribbleSuccess
                } else if roll < 0.7 {
                    EventType::CrossSuccess
                } else if roll < 0.9 {
                    EventType::KeyPass
                } else {
                    EventType::ShotOnTarget
                }
            },
            Position::CF | Position::SS => {
                // Forward actions
                let roll = self.rng.gen::<f32>();
                if roll < 0.5 {
                    EventType::ShotOnTarget
                } else if roll < 0.75 {
                    EventType::Goal
                } else if roll < 0.95 {
                    EventType::DribbleSuccess
                } else {
                    EventType::Assist
                }
            },
            Position::RB => {
                // Right back actions
                let roll = self.rng.gen::<f32>();
                if roll < 0.3 {
                    EventType::CrossSuccess
                } else if roll < 0.6 {
                    EventType::TackleWon
                } else if roll < 0.85 {
                    EventType::PassSuccess
                } else {
                    EventType::DribbleSuccess
                }
            },
            Position::LB => {
                // Left back actions
                let roll = self.rng.gen::<f32>();
                if roll < 0.3 {
                    EventType::CrossSuccess
                } else if roll < 0.6 {
                    EventType::TackleWon
                } else if roll < 0.85 {
                    EventType::PassSuccess
                } else {
                    EventType::DribbleSuccess
                }
            },
            Position::AM => {
                // Attacking midfielder actions
                let roll = self.rng.gen::<f32>();
                if roll < 0.25 {
                    EventType::KeyPass
                } else if roll < 0.5 {
                    EventType::PassSuccess
                } else if roll < 0.7 {
                    EventType::DribbleSuccess
                } else if roll < 0.9 {
                    EventType::ShotOnTarget
                } else {
                    EventType::ThroughBall
                }
            },
        }
    }

    /// Decides what type of defensive action a player should take
    fn decide_defensive_action_type(&mut self, player: &Player) -> EventType {
        match player.primary_position {
            Position::GK => {
                if self.rng.gen::<f32>() < 0.8 {
                    EventType::Save
                } else {
                    EventType::ClaimCross
                }
            },
            Position::CB => {
                if self.rng.gen::<f32>() < 0.5 {
                    EventType::TackleWon
                } else if self.rng.gen::<f32>() < 0.8 {
                    EventType::Interception
                } else {
                    EventType::Clearance
                }
            },
            Position::FB => {
                if self.rng.gen::<f32>() < 0.6 {
                    EventType::TackleWon
                } else {
                    EventType::Interception
                }
            },
            _ => {
                if self.rng.gen::<f32>() < 0.5 {
                    EventType::TackleWon
                } else {
                    EventType::Interception
                }
            },
        }
    }

    /// Selects a secondary player for the event (opponent or teammate)
    fn select_secondary_player(&mut self, match_state: &MatchState, primary_player_id: Uuid) -> Option<Uuid> {
        // Find which team the primary player is on
        let is_home_player = match self.find_player_by_id(match_state, primary_player_id) {
            Some(player) => {
                match_state.home_players.iter().any(|p| p.player.id == player.id)
            },
            None => return None,
        };
        
        // Select from opposite team (for challenges) or same team (for assists/passes)
        let team_players = if is_home_player {
            &match_state.away_players
        } else {
            &match_state.home_players
        };
        
        if team_players.is_empty() {
            return None;
        }
        
        // Randomly select a player from the other team
        let idx = self.rng.gen_range(0..team_players.len());
        Some(team_players[idx].player.id)
    }

    /// Determines the pitch zone for an event
    fn determine_pitch_zone(&mut self, minute: u8) -> PitchZone {
        // More likely to be in final third as game goes on
        let final_third_chance = 0.2 + (minute as f32 / 90.0) * 0.3;
        
        if self.rng.gen::<f32>() < final_third_chance {
            if self.rng.gen::<f32>() < 0.6 {
                PitchZone::FinalThird
            } else {
                PitchZone::Box
            }
        } else if self.rng.gen::<f32>() < 0.5 {
            PitchZone::MiddleThird
        } else {
            PitchZone::DefensiveThird
        }
    }

    /// Determines if an action is successful based on player attributes
    fn determine_success_based_on_attributes(&mut self, player: &Player, action_type: &EventType) -> bool {
        // Base success rate varies by action type
        let base_success_rate = match action_type {
            EventType::Goal => (player.technical.shooting as f32) / 120.0,
            EventType::ShotOnTarget => (player.technical.shooting as f32) / 100.0,
            EventType::KeyPass => (player.technical.passing as f32) / 100.0,
            EventType::Assist => (player.technical.passing as f32) / 90.0,
            EventType::DribbleSuccess => (player.technical.dribbling as f32) / 100.0,
            EventType::TackleWon => (player.technical.tackling as f32) / 100.0,
            EventType::Interception => (player.mental.vision as f32) / 100.0,
            EventType::Block => (player.mental.positioning as f32) / 100.0,
            EventType::Clearance => (player.mental.positioning as f32) / 90.0,
            EventType::Save => (player.hidden.big_match_temperament as f32) / 100.0,
            _ => 0.7, // Default success rate
        };
        
        // Apply form and morale modifiers
        let form_modifier = player.form / 70.0; // Normalize form around average
        let morale_modifier = player.morale / 70.0; // Normalize morale around average
        
        let adjusted_success_rate = (base_success_rate * form_modifier * morale_modifier).min(0.95);
        
        self.rng.gen::<f32>() < adjusted_success_rate
    }

    /// Gets the base impact value for an event type
    fn get_base_impact(&self, event_type: &EventType) -> f32 {
        match event_type {
            EventType::Goal => 8.0,
            EventType::Assist => 5.0,
            EventType::KeyPass => 2.5,
            EventType::ShotOnTarget => 1.5,
            EventType::ShotOffTarget => 0.8,
            EventType::DribbleSuccess => 0.7,
            EventType::TackleWon => 1.2,
            EventType::Interception => 1.0,
            EventType::Block => 2.0,
            EventType::Clearance => 0.8,
            EventType::AerialDuelWon => 0.6,
            EventType::Save => 2.5,
            EventType::ReflexSave => 3.5,
            EventType::OneOnOneSave => 4.0,
            EventType::ClaimCross => 0.5,
            EventType::PunchClear => 0.6,
            EventType::SweeperClearance => 1.0,
            EventType::GoalConceded => -2.0,
            EventType::FoulCommitted => -0.5,
            EventType::YellowCard => -1.0,
            EventType::RedCard => -3.0,
            EventType::MissedBigChance => -2.5,
            EventType::PenaltyWon => 2.0,
            EventType::PenaltyConceded => -2.0,
            EventType::PenaltySaved => 4.0,
            EventType::PenaltyMissed => -3.0,
            _ => 0.0,
        }
    }

    /// Calculates time-based multiplier for events
    fn calculate_time_multiplier(&self, minute: u8, score_difference: i8) -> f32 {
        // Events later in the game have higher impact
        let time_factor = 1.0 + (minute as f32 / 90.0) * 0.3; // Up to 30% bonus for late game
        
        // Important moments (close scores, late game) have higher impact
        let pressure_factor = if score_difference.abs() <= 1 && minute > 70 {
            1.4  // High pressure situation
        } else if score_difference.abs() <= 1 {
            1.2  // Close game
        } else {
            1.0  // Normal situation
        };
        
        time_factor * pressure_factor
    }

    /// Calculates position-based multiplier for events
    fn calculate_position_multiplier(&self, event_type: &EventType, position: Position) -> f32 {
        match event_type {
            // Scoring events
            EventType::Goal => match position {
                Position::CF | Position::SS => 1.0,  // Expected for forwards
                Position::RW | Position::LW => 1.1,  // Slightly more impressive for wingers
                Position::CM => 1.2,                 // More impressive for midfielders
                Position::DM => 1.3,                 // Even more impressive for defensive mids
                Position::FB => 1.4,                 // Very impressive for fullbacks
                Position::CB => 1.5,                 // Highly impressive for center backs
                Position::GK => 2.0,                 // Extremely impressive for goalkeepers
                Position::RB => 1.4,                 // Very impressive for right backs
                Position::LB => 1.4,                 // Very impressive for left backs
                Position::AM => 1.2,                 // More impressive for attacking mids
                Position::RM | Position::LM => 1.2,  // More impressive for attacking mids
            },
            // Defensive events by attackers
            EventType::TackleWon | EventType::Interception | EventType::Clearance => match position {
                Position::CF | Position::SS => 1.4,  // Very impressive for forwards
                Position::RW | Position::LW => 1.3,  // Impressive for wingers
                Position::CM => 1.2,                 // Impressive for midfielders
                Position::DM | Position::CB | Position::FB => 1.0,  // Normal for defensive players
                Position::GK => 1.1,                 // Slightly impressive for keepers
                Position::RB => 1.0,                 // Normal for right backs
                Position::LB => 1.0,                 // Normal for left backs
                Position::AM => 1.2,                 // Impressive for attacking mids
                Position::RM | Position::LM => 1.0,  // Normal for wide mids
            },
            // Creating events by defenders
            EventType::KeyPass | EventType::Assist => match position {
                Position::CB => 1.5,                 // Very impressive for center backs
                Position::FB => 1.4,                 // Impressive for fullbacks
                Position::DM => 1.3,                 // Impressive for defensive mids
                Position::CM => 1.1,                 // Somewhat impressive for central mids
                Position::RM => 1.0,                 // Normal for right mids
                Position::LM => 1.0,                 // Normal for left mids
                Position::RW | Position::LW => 1.0,  // Normal for creative roles
                Position::CF | Position::SS => 1.2,  // Somewhat impressive for forwards
                Position::RB => 1.4,                 // Impressive for right backs
                Position::LB => 1.4,                 // Impressive for left backs
                Position::AM => 1.1,                 // Somewhat impressive for attacking mids
                Position::GK => 1.6,                 // Very impressive for goalkeepers
            },
            _ => 1.0,  // Default multiplier
        }
    }

    /// Calculates difficulty multiplier based on attributes and match state
    fn calculate_difficulty_multiplier(&self, _player: &Player, match_state: &MatchState) -> f32 {
        // Higher-rated opponents make successful actions more valuable
        let opposition_quality = match_state.average_opposition_rating;
        
        // Calculate based on how difficult the action was
        let difficulty_factor = opposition_quality / 50.0; // Normalize around average rating
        
        // Actions against stronger opposition are more valuable
        1.0 + (difficulty_factor - 1.0) * 0.3  // Up to 30% bonus for difficult actions
    }

    /// Calculates clutch multiplier based on game situation
    fn calculate_clutch_multiplier(&self, minute: u8, score_difference: i8, match_importance: MatchImportance) -> f32 {
        let mut multiplier: f32 = 1.0;
        
        // Late game situations
        if minute > 75 {
            multiplier *= 1.2;
        }
        
        // Close games
        if score_difference.abs() <= 1 {
            multiplier *= 1.15;
        }
        
        // Important matches
        match match_importance {
            MatchImportance::Friendly => 0.8,
            MatchImportance::League => 1.0,
            MatchImportance::Cup => 1.2,
            MatchImportance::Final => 1.5,
            MatchImportance::Continental => 1.4,
        };
        
        // Combination of late + close + important = very high multiplier
        multiplier.min(2.0f32)  // Cap to prevent excessive ratings
    }

    /// Calculates player ratings based on their match events
    fn calculate_player_ratings(&self, game_match: &Match, match_state: &MatchState) -> HashMap<Uuid, f32> {
        let mut ratings = HashMap::new();
        
        // Get all players who participated in the match
        let all_players: Vec<Uuid> = game_match.events
            .iter()
            .map(|event| event.player_involved)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        
        for player_id in all_players {
            let player_events: Vec<&MatchEvent> = game_match.events
                .iter()
                .filter(|event| event.player_involved == player_id)
                .collect();
            
            let rating = self.calculate_single_player_rating(&player_events, match_state);
            ratings.insert(player_id, rating.clamp(4.5, 9.9));
        }
        
        ratings
    }

    /// Calculates a single player's rating based on their events
    fn calculate_single_player_rating(&self, events: &[&MatchEvent], _match_state: &MatchState) -> f32 {
        if events.is_empty() {
            return 6.0; // Default rating for no involvement
        }
        
        // Step 1: Aggregate event impacts
        let mut positive_impact = 0.0;
        let mut negative_impact = 0.0;
        
        for event in events {
            if event.total_impact_score >= 0.0 {
                positive_impact += event.total_impact_score;
            } else {
                negative_impact += event.total_impact_score.abs();
            }
        }
        
        // Step 2: Calculate involvement score
        let involvement_score = self.calculate_involvement_score(events);
        
        // Step 3: Calculate consistency factor
        let consistency_factor = self.calculate_consistency_factor(events);
        
        // Step 4: Apply penalties for negative events
        let final_positive = positive_impact * consistency_factor;
        let final_negative = negative_impact * 1.2; // Mistakes matter more
        
        // Step 5: Calculate raw score
        let raw_score = 6.0 + final_positive - final_negative;
        
        // Step 6: Apply involvement cap if needed
        if involvement_score < 0.3 {
            raw_score.min(6.8) // Cap for low involvement
        } else {
            raw_score
        }
    }

    /// Calculates how involved a player was in the match
    fn calculate_involvement_score(&self, events: &[&MatchEvent]) -> f32 {
        // Count meaningful events (not just minor touches)
        let meaningful_events = events.iter()
            .filter(|event| event.base_impact.abs() > 0.3)
            .count();
        
        // Normalize to 0-1 scale (arbitrary threshold of 10 events for full involvement)
        (meaningful_events as f32 / 10.0).min(1.0)
    }

    /// Calculates consistency factor to prevent stat padding
    fn calculate_consistency_factor(&self, events: &[&MatchEvent]) -> f32 {
        if events.is_empty() {
            return 1.0;
        }
        
        // Group events by type to detect repetition
        let mut event_counts = std::collections::HashMap::new();
        for event in events {
            *event_counts.entry(event.event_type.clone()).or_insert(0) += 1;
        }
        
        // Apply diminishing returns for repeated event types
        let mut total_weighted_impact: f32 = 0.0;
        let mut total_impact: f32 = 0.0;
        
        for (event_type, _count) in event_counts {
            let events_of_type: Vec<&MatchEvent> = events.iter()
                .filter(|e| e.event_type == event_type)
                .copied()
                .collect();
                
            for (idx, event) in events_of_type.iter().enumerate() {
                // Apply diminishing returns: first event = full value, subsequent events = reduced value
                let diminishing_factor = if idx == 0 { 1.0 } else { 0.7 / (idx as f32) };
                total_weighted_impact += event.total_impact_score * diminishing_factor;
                total_impact += event.total_impact_score;
            }
        }
        
        // Consistency factor is the ratio of weighted impact to total impact
        // Closer to 1.0 means more diverse, consistent performance
        // Lower means repetitive, padded stats
        if total_impact.abs() > 0.001f32 {  // Avoid division by zero
            (total_weighted_impact / total_impact).max(0.5f32)  // Minimum 0.5 to prevent extreme penalties
        } else {
            1.0
        }
    }

    /// Updates player match stats based on events
    fn update_player_match_stats(&self, game_match: &mut Match, _match_state: &MatchState) {
        // Initialize stats for all players
        for player_id in &game_match.lineup.home_starting_xi {
            if let Some(player_in_match) = game_match.lineup.players.iter_mut()
                .find(|p| p.player_id == *player_id) {
                player_in_match.stats = PlayerMatchStats::default();
            }
        }

        for player_id in &game_match.lineup.away_starting_xi {
            if let Some(player_in_match) = game_match.lineup.players.iter_mut()
                .find(|p| p.player_id == *player_id) {
                player_in_match.stats = PlayerMatchStats::default();
            }
        }
        
        // Process all events to update stats
        for event in &game_match.events {
            self.update_stats_from_event(&mut game_match.lineup, event);
        }
        
        // Set minutes played (simplified - all starters play full match)
        for player_id in &game_match.lineup.home_starting_xi {
            if let Some(player_in_match) = game_match.lineup.players.iter_mut()
                .find(|p| p.player_id == *player_id) {
                player_in_match.stats.minutes_played = 90;
            }
        }

        for player_id in &game_match.lineup.away_starting_xi {
            if let Some(player_in_match) = game_match.lineup.players.iter_mut()
                .find(|p| p.player_id == *player_id) {
                player_in_match.stats.minutes_played = 90;
            }
        }
    }

    /// Updates player stats based on a single event
    fn update_stats_from_event(&self, lineup: &mut MatchLineup, event: &MatchEvent) {
        // Find the player in the lineup
        if let Some(player_in_match) = lineup.players.iter_mut()
            .find(|p| p.player_id == event.player_involved) {
            self.increment_stat_for_event(&mut player_in_match.stats, &event.event_type);
        }
    }

    /// Increments the appropriate stat based on event type
    fn increment_stat_for_event(&self, stats: &mut PlayerMatchStats, event_type: &EventType) {
        match event_type {
            EventType::Goal => stats.goals += 1,
            EventType::Assist => stats.assists += 1,
            EventType::ShotOnTarget => stats.shots_on_target += 1,
            EventType::ShotOffTarget => stats.shots_off_target += 1,
            EventType::TackleWon => stats.tackles_won += 1,
            EventType::Interception => stats.interceptions += 1,
            EventType::Clearance => stats.clearances += 1,
            EventType::Save => {
                if let Some(ref mut saves) = stats.saves {
                    *saves += 1;
                } else {
                    stats.saves = Some(1);
                }
            },
            EventType::YellowCard => stats.yellow_cards += 1,
            EventType::RedCard => stats.red_cards += 1,
            _ => {} // Other events don't directly increment basic stats
        }
    }
}

/// Represents the state of a match during simulation
#[derive(Debug)]
struct MatchState<'a> {
    match_id: Uuid,
    home_team_id: Uuid,
    away_team_id: Uuid,
    home_players: Vec<PlayerInMatchRef<'a>>,
    away_players: Vec<PlayerInMatchRef<'a>>,
    home_tactical_balance: f32,  // 0.0 = all away possession, 1.0 = all home possession
    score_difference: i8,        // Home goals - Away goals
    average_opposition_rating: f32,
    match_importance: MatchImportance,
}

/// Reference to a player in the match context
#[derive(Debug)]
struct PlayerInMatchRef<'a> {
    player: &'a Player,
    _position: Position,
}

impl<'a> MatchState<'a> {
    fn new(
        home_players: &'a [Player],
        away_players: &'a [Player],
        _home_lineup: &MatchLineup,
        _away_lineup: &MatchLineup,
    ) -> Self {
        let home_refs: Vec<PlayerInMatchRef> = home_players.iter()
            .map(|p| PlayerInMatchRef { player: p, _position: p.primary_position })  // Simplified position assignment
            .collect();
            
        let away_refs: Vec<PlayerInMatchRef> = away_players.iter()
            .map(|p| PlayerInMatchRef { player: p, _position: p.primary_position })  // Simplified position assignment
            .collect();
        
        MatchState {
            match_id: Uuid::new_v4(),  // Placeholder
            home_team_id: Uuid::new_v4(),  // Placeholder
            away_team_id: Uuid::new_v4(),  // Placeholder
            home_players: home_refs,
            away_players: away_refs,
            home_tactical_balance: 0.5,  // Equal possession initially
            score_difference: 0,
            average_opposition_rating: 6.5,  // Placeholder average
            match_importance: MatchImportance::League,  // Placeholder
        }
    }
}

/// Match importance levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MatchImportance {
    Friendly,
    League,
    Cup,
    Final,
    Continental,
}

impl Match {
    /// Determines if the match requires extra time (for knockout competitions)
    fn requires_extra_time(&self) -> bool {
        // Simplified: matches that must have a winner require extra time
        matches!(self.competition_type, crate::entities::CompetitionType::Knockout)
    }
}

impl Default for PlayerMatchStats {
    fn default() -> Self {
        PlayerMatchStats {
            tackles: 0,
            tackles_won: 0,
            interceptions: 0,
            passes_completed: 0,
            passes_attempted: 0,
            shots_on_target: 0,
            shots_off_target: 0,
            dribbles_successful: 0,
            dribbles_attempted: 0,
            aerials_won: 0,
            aerials_lost: 0,
            fouls_committed: 0,
            fouls_suffered: 0,
            offsides: 0,
            clearances: 0,
            blocks: 0,
            duels_won: 0,
            duels_lost: 0,
            saves: None,
            goals: 0,
            assists: 0,
            yellow_cards: 0,
            red_cards: 0,
            minutes_played: 0,
            possession_time: 0.0,
            distance_covered: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_engine_creation() {
        let mut engine = MatchEngine::new();
        let _ = engine.rng.gen::<bool>(); // Just to use the rng field
    }

    #[test]
    fn test_base_impact_values() {
        let engine = MatchEngine::new();
        
        assert_eq!(engine.get_base_impact(&EventType::Goal), 8.0);
        assert_eq!(engine.get_base_impact(&EventType::Assist), 5.0);
        assert_eq!(engine.get_base_impact(&EventType::Save), 2.5);
        assert_eq!(engine.get_base_impact(&EventType::YellowCard), -1.0);
    }

    #[test]
    fn test_time_multiplier() {
        let engine = MatchEngine::new();
        
        // Test late game multiplier
        let multiplier = engine.calculate_time_multiplier(85, 0);
        assert!(multiplier > 1.0);
        
        // Test close game multiplier
        let multiplier = engine.calculate_time_multiplier(75, 1);
        assert!(multiplier > 1.0);
    }

    #[test]
    fn test_calculate_involvement_score() {
        let engine = MatchEngine::new();
        
        // Empty events should return 0.0
        let empty_events: Vec<&MatchEvent> = vec![];
        let score = engine.calculate_involvement_score(&empty_events);
        assert_eq!(score, 0.0);
    }
}