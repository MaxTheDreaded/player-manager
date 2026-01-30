// src/ui/console_ui.rs
use std::io::{self, Write};
use std::collections::HashMap;
use uuid::Uuid;

use crate::entities::{Player, Team, Match, Competition};
use crate::core::time_engine::TimeEngine;
use crate::core::event_engine::{EventEngine, UserDecisionRequest};
use crate::ui::tutorial::TutorialManager;

/// The ConsoleUI provides the text-based interface for the game
/// It displays data, presents choices, and sends user decisions back to the system
pub struct ConsoleUI {
    time_engine: TimeEngine,
    _event_engine: EventEngine,
    tutorial_manager: TutorialManager,
}

impl ConsoleUI {
    /// Creates a new ConsoleUI instance
    pub fn new(time_engine: TimeEngine, event_engine: EventEngine) -> Self {
        ConsoleUI {
            time_engine,
            _event_engine: event_engine,
            tutorial_manager: TutorialManager::new(),
        }
    }

    /// Main game loop for the console interface
    pub fn run_main_loop(&mut self, mut player: Player, all_teams: Vec<Team>, _competitions: Vec<Competition>) {
        println!("⚽ Welcome to From Boots to Ballon d'Or!");
        println!("Playing as: {}", player.name);
        println!("Age: {}, Position: {:?}", player.age, player.primary_position);
        println!();

        // Show main menu tutorial if first time
        self.show_tutorial_if_needed("main_menu", &mut player.tutorial_state);

        loop {
            // Show current status
            self.display_weekly_status(&mut player, &all_teams);
            
            // Check for any events requiring user input
            if let Some(user_decision) = self.check_for_user_decisions() {
                self.handle_user_decision(&mut player, user_decision);
            } else {
                // If no user decisions, advance time
                match self.time_engine.advance_time() {
                    Ok(()) => {
                        // Time advanced successfully
                        println!("Time advanced. Checking for events...");
                    },
                    Err(e) => {
                        eprintln!("Error advancing time: {}", e);
                        break;
                    }
                }
            }
            
            // Prompt user to continue
            println!("\nPress Enter to continue...");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input_trim = input.trim();
            
            if input_trim.eq_ignore_ascii_case("help") || input_trim.eq_ignore_ascii_case("h") {
                self.show_tutorial("main_menu");
                continue;
            }
            
            if input_trim.eq_ignore_ascii_case("quit") || input_trim.eq_ignore_ascii_case("q") {
                break;
            }
        }
    }

    /// Displays the weekly status screen
    fn display_weekly_status(&self, player: &mut Player, all_teams: &[Team]) {
        println!("┌─────────────────────────────────────────────────────────┐");
        println!("│                    WEEKLY STATUS                        │");
        println!("├─────────────────────────────────────────────────────────┤");
        
        // Show team info tutorial if first time (conceptually part of main screen info)
        self.show_tutorial_if_needed("team_info", &mut player.tutorial_state); // Just exemplary
        
        // Player info
        println!("│ Player: {:<45} │", player.name);
        println!("│ Age: {:<5} Position: {:<32} │", player.age, format!("{:?}", player.primary_position));
        println!("├─────────────────────────────────────────────────────────┤");
        
        // Fitness and form
        println!("│ Fitness: {:<7.1} Morale: {:<7.1} Form: {:<7.1} │", 
                 player.fitness, player.morale, player.form);
        println!("│ Fatigue: {:<7.1} Sharpness: {:<6.1} │", 
                 player.fatigue, player.sharpness);
        println!("├─────────────────────────────────────────────────────────┤");
        
        // Reputation
        println!("│ Local Rep: {:<9.1} International Rep: {:<10.1} │", 
                 player.local_reputation, player.international_reputation);
        println!("├─────────────────────────────────────────────────────────┤");
        
        // Contract info
        let unknown_club = "Unknown Club".to_string();
        let current_team = all_teams.iter()
            .find(|team| team.id == player.contract.club_id)
            .map(|team| &team.name)
            .unwrap_or(&unknown_club);
        
        println!("│ Club: {:<48} │", current_team);
        println!("│ Squad Role: {:<10} Wage: £{:<18.0} │", 
                 format!("{:?}", player.contract.squad_role), player.contract.wage);
        println!("├─────────────────────────────────────────────────────────┤");
        
        // Upcoming matches
        println!("│ Upcoming Matches:                                       │");
        // In a real implementation, this would show actual upcoming matches
        println!("│ - No matches scheduled this week                        │");
        println!("└─────────────────────────────────────────────────────────┘");
        println!();
    }

    /// Checks for any events requiring user decisions
    fn check_for_user_decisions(&mut self) -> Option<UserDecisionRequest> {
        // In a real implementation, this would check the event engine for user input events
        // For now, we'll return None
        None
    }

    /// Handles a user decision
    fn handle_user_decision(&mut self, player: &mut Player, decision: UserDecisionRequest) {
        match decision.decision_type {
            crate::core::event_engine::DecisionType::TrainingFocusSelection => {
                self.handle_training_focus_selection(player, &decision);
            },
            crate::core::event_engine::DecisionType::MatchDayChoice => {
                self.handle_match_day_choice(player, &decision);
            },
            crate::core::event_engine::DecisionType::TransferOfferResponse => {
                self.handle_transfer_offer_response(player, &decision);
            },
            crate::core::event_engine::DecisionType::ContractNegotiation => {
                self.handle_contract_negotiation(player, &decision);
            },
            crate::core::event_engine::DecisionType::ManagerConversation => {
                self.handle_manager_conversation(player, &decision);
            },
            crate::core::event_engine::DecisionType::MediaInterview => {
                self.handle_media_interview(player, &decision);
            },
            crate::core::event_engine::DecisionType::PersonalLifeChoice => {
                self.handle_personal_life_choice(player, &decision);
            },
        }
    }

    /// Handles training focus selection
    fn handle_training_focus_selection(&mut self, _player: &mut Player, decision: &UserDecisionRequest) {
        println!("🎯 SELECT TRAINING FOCUS");
        println!("Choose your training focus for this week:");
        
        for (i, option) in decision.options.iter().enumerate() {
            println!("{}. {}", i + 1, option.text);
        }
        
        print!("Enter your choice (1-{}): ", decision.options.len());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        if let Ok(choice) = input.trim().parse::<usize>() {
            if choice > 0 && choice <= decision.options.len() {
                let selected_option = &decision.options[choice - 1];
                println!("You selected: {}", selected_option.text);
                
                // In a real implementation, this would update the player's training focus
                // and pass it to the training system
            } else {
                println!("Invalid choice. Using default.");
            }
        } else {
            println!("Invalid input. Using default.");
        }
    }

    /// Handles match day choice
    fn handle_match_day_choice(&mut self, player: &mut Player, decision: &UserDecisionRequest) {
        println!("⚽ MATCH DAY DECISION");
        println!("What would you like to do before the match?");
        
        for (i, option) in decision.options.iter().enumerate() {
            println!("{}. {}", i + 1, option.text);
        }
        
        print!("Enter your choice (1-{}): ", decision.options.len());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        if let Ok(choice) = input.trim().parse::<usize>() {
            if choice > 0 && choice <= decision.options.len() {
                let selected_option = &decision.options[choice - 1];
                println!("You selected: {}", selected_option.text);
                
                // Apply the choice's consequences
                for consequence in &selected_option.consequences {
                    self.apply_consequence(player, consequence);
                }
            } else {
                println!("Invalid choice.");
            }
        } else {
            println!("Invalid input.");
        }
    }

    /// Handles transfer offer response
    fn handle_transfer_offer_response(&mut self, player: &mut Player, decision: &UserDecisionRequest) {
        println!("💼 TRANSFER OFFER");
        println!("You have received a transfer offer!");
        
        // Display offer details (would come from context in real implementation)
        println!("Club: Manchester United");
        println!("Wage: £200,000/week");
        println!("Contract: 5 years");
        println!("Transfer Fee: £50,000,000");
        
        println!("\nYour options:");
        for (i, option) in decision.options.iter().enumerate() {
            println!("{}. {}", i + 1, option.text);
        }
        
        print!("Enter your choice (1-{}): ", decision.options.len());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        if let Ok(choice) = input.trim().parse::<usize>() {
            if choice > 0 && choice <= decision.options.len() {
                let selected_option = &decision.options[choice - 1];
                println!("You selected: {}", selected_option.text);
                
                // Apply the choice's consequences
                for consequence in &selected_option.consequences {
                    self.apply_consequence(player, consequence);
                }
            } else {
                println!("Invalid choice.");
            }
        } else {
            println!("Invalid input.");
        }
    }

    /// Handles contract negotiation
    fn handle_contract_negotiation(&mut self, player: &mut Player, decision: &UserDecisionRequest) {
        println!("📋 CONTRACT NEGOTIATION");
        println!("Your current contract is expiring. Negotiate new terms:");
        
        for (i, option) in decision.options.iter().enumerate() {
            println!("{}. {}", i + 1, option.text);
        }
        
        print!("Enter your choice (1-{}): ", decision.options.len());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        if let Ok(choice) = input.trim().parse::<usize>() {
            if choice > 0 && choice <= decision.options.len() {
                let selected_option = &decision.options[choice - 1];
                println!("You selected: {}", selected_option.text);
                
                // Apply the choice's consequences
                for consequence in &selected_option.consequences {
                    self.apply_consequence(player, consequence);
                }
            } else {
                println!("Invalid choice.");
            }
        } else {
            println!("Invalid input.");
        }
    }

    /// Handles manager conversation
    fn handle_manager_conversation(&mut self, player: &mut Player, decision: &UserDecisionRequest) {
        println!("👥 MANAGER CONVERSATION");
        println!("Your manager wants to talk to you about your role in the team.");
        
        for (i, option) in decision.options.iter().enumerate() {
            println!("{}. {}", i + 1, option.text);
        }
        
        print!("Enter your choice (1-{}): ", decision.options.len());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        if let Ok(choice) = input.trim().parse::<usize>() {
            if choice > 0 && choice <= decision.options.len() {
                let selected_option = &decision.options[choice - 1];
                println!("You selected: {}", selected_option.text);
                
                // Apply the choice's consequences
                for consequence in &selected_option.consequences {
                    self.apply_consequence(player, consequence);
                }
            } else {
                println!("Invalid choice.");
            }
        } else {
            println!("Invalid input.");
        }
    }

    /// Handles media interview
    fn handle_media_interview(&mut self, player: &mut Player, decision: &UserDecisionRequest) {
        println!("🎤 MEDIA INTERVIEW");
        println!("You're being interviewed after the match.");
        
        for (i, option) in decision.options.iter().enumerate() {
            println!("{}. {}", i + 1, option.text);
        }
        
        print!("Enter your choice (1-{}): ", decision.options.len());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        if let Ok(choice) = input.trim().parse::<usize>() {
            if choice > 0 && choice <= decision.options.len() {
                let selected_option = &decision.options[choice - 1];
                println!("You selected: {}", selected_option.text);
                
                // Apply the choice's consequences
                for consequence in &selected_option.consequences {
                    self.apply_consequence(player, consequence);
                }
            } else {
                println!("Invalid choice.");
            }
        } else {
            println!("Invalid input.");
        }
    }

    /// Handles personal life choice
    fn handle_personal_life_choice(&mut self, player: &mut Player, decision: &UserDecisionRequest) {
        println!("🏠 PERSONAL LIFE CHOICE");
        println!("Something important has happened in your personal life.");
        
        for (i, option) in decision.options.iter().enumerate() {
            println!("{}. {}", i + 1, option.text);
        }
        
        print!("Enter your choice (1-{}): ", decision.options.len());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        if let Ok(choice) = input.trim().parse::<usize>() {
            if choice > 0 && choice <= decision.options.len() {
                let selected_option = &decision.options[choice - 1];
                println!("You selected: {}", selected_option.text);
                
                // Apply the choice's consequences
                for consequence in &selected_option.consequences {
                    self.apply_consequence(player, consequence);
                }
            } else {
                println!("Invalid choice.");
            }
        } else {
            println!("Invalid input.");
        }
    }

    /// Applies a consequence to the player
    fn apply_consequence(&self, player: &mut Player, consequence: &crate::core::event_engine::Consequence) {
        use crate::core::event_engine::ConsequenceType;
        
        match consequence.consequence_type {
            ConsequenceType::MoraleChange => {
                player.morale = (player.morale + consequence.value).clamp(0.0, 100.0);
                println!("Morale changed by {:.1}", consequence.value);
            },
            ConsequenceType::ReputationChange => {
                player.local_reputation = (player.local_reputation + consequence.value).clamp(0.0, 100.0);
                println!("Local reputation changed by {:.1}", consequence.value);
            },
            ConsequenceType::AttributeImprovement(ref _attr_type) => {
                // In a real implementation, this would modify the appropriate attribute
                println!("Attribute improved");
            },
            ConsequenceType::RelationshipChange => {
                // In a real implementation, this would modify relationships
                println!("Relationship changed");
            },
            ConsequenceType::FinancialImpact => {
                // In a real implementation, this would modify player's finances
                println!("Financial impact applied");
            },
            ConsequenceType::PlayingTimeImpact => {
                // In a real implementation, this would affect playing time
                println!("Playing time affected");
            },
            ConsequenceType::ContractStatusChange => {
                // In a real implementation, this would modify contract status
                println!("Contract status changed");
            },
        }
    }

    /// Displays player profile
    pub fn display_player_profile(&self, player: &mut Player) {
        println!("┌─────────────────────────────────────────────────────────┐");
        println!("│                    PLAYER PROFILE                       │");
        println!("├─────────────────────────────────────────────────────────┤");
        
        // Show tutorial if first time
        self.show_tutorial_if_needed("player_profile", &mut player.tutorial_state);
        
        println!("│ Name: {:<48} │", player.name);
        println!("│ Age: {:<3} Nationality: {:<31} │", player.age, player.nationality);
        println!("│ Height: {:<4}cm Weight: {:<5}kg {:<24} │", player.height, player.weight, "");
        println!("│ Preferred Foot: {:<34} │", format!("{:?}", player.preferred_foot));
        println!("│ Primary Position: {:<32} │", format!("{:?}", player.primary_position));
        println!("├─────────────────────────────────────────────────────────┤");
        println!("│                     ATTRIBUTES                          │");
        println!("├─────────────────────────────────────────────────────────┤");
        println!("│ Technical:                                              │");
        println!("│   Dribbling: {:<3} Passing: {:<3} Shooting: {:<3}        │", 
                 player.technical.dribbling, player.technical.passing, player.technical.shooting);
        println!("│   First Touch: {:<3} Tackling: {:<3} Crossing: {:<3}     │", 
                 player.technical.first_touch, player.technical.tackling, player.technical.crossing);
        println!("│ Physical:                                               │");
        println!("│   Pace: {:<3} Stamina: {:<3} Strength: {:<3}           │", 
                 player.physical.pace, player.physical.stamina, player.physical.strength);
        println!("│   Agility: {:<3} Jumping: {:<3}                        │", 
                 player.physical.agility, player.physical.jumping);
        println!("│ Mental:                                                 │");
        println!("│   Composure: {:<3} Vision: {:<3} Work Rate: {:<3}      │", 
                 player.mental.composure, player.mental.vision, player.mental.work_rate);
        println!("│   Determination: {:<3} Positioning: {:<3} Teamwork: {:<3} │", 
                 player.mental.determination, player.mental.positioning, player.mental.teamwork);
        println!("├─────────────────────────────────────────────────────────┤");
        println!("│                     CURRENT STATUS                      │");
        println!("├─────────────────────────────────────────────────────────┤");
        println!("│ Fitness: {:<6.1} Fatigue: {:<6.1} Form: {:<6.1}        │", 
                 player.fitness, player.fatigue, player.form);
        println!("│ Morale: {:<6.1} Sharpness: {:<6.1}                     │", 
                 player.morale, player.sharpness);
        println!("├─────────────────────────────────────────────────────────┤");
        println!("│                     REPUTATION                          │");
        println!("├─────────────────────────────────────────────────────────┤");
        println!("│ Local: {:<6.1} International: {:<6.1}                   │", 
                 player.local_reputation, player.international_reputation);
        println!("└─────────────────────────────────────────────────────────┘");
    }

    /// Displays team information
    pub fn display_team_info(&self, team: &Team, seen_states: &mut HashMap<String, bool>) {
        println!("┌─────────────────────────────────────────────────────────┐");
        println!("│                    TEAM INFORMATION                     │");
        println!("├─────────────────────────────────────────────────────────┤");
        
        // Show tutorial if first time
        self.show_tutorial_if_needed("team_info", seen_states);
        println!("│ Club: {:<48} │", team.name);
        println!("│ Reputation: {:<8.1} Financial Power: {:<10.1} │", 
                 team.reputation, team.financial_power);
        println!("│ Youth Focus: {:<8.1} Facilities: {:<12.1} │", 
                 team.youth_focus, team.facilities_quality);
        println!("│ Medical Quality: {:<6.1} Tactical Style: {:<10} │", 
                 team.medical_quality, format!("{:?}", team.tactical_identity));
        println!("├─────────────────────────────────────────────────────────┤");
        println!("│ Squad Size: {:<42} │", team.squad.len());
        println!("│ Manager: {:<46} │", "Unknown"); // Would come from manager profile
        println!("└─────────────────────────────────────────────────────────┘");
    }

    /// Displays match report
    pub fn display_match_report(&self, game_match: &Match, player: &mut Player) {
        println!("┌─────────────────────────────────────────────────────────┐");
        println!("│                      MATCH REPORT                       │");
        println!("├─────────────────────────────────────────────────────────┤");
        
        // Show tutorial if first time
        self.show_tutorial_if_needed("match_report", &mut player.tutorial_state);
        
        
        // Match info
        if let Some((home_goals, away_goals)) = game_match.fulltime_score {
            println!("│ {:<20} {} - {} {:<20} │", 
                     "Home Team", home_goals, away_goals, "Away Team");
        }
        
        // Player rating
        if let Some(rating) = game_match.player_ratings.get(&player.id) {
            println!("│ Your Rating: {:<42.1} │", rating);
        }
        
        // Player stats
        // In a real implementation, this would show actual player stats from the match
        println!("│ Goals: 0  Assists: 0  Shots: 0  Tackles: 0           │");
        println!("│ Passes: 0  Dribbles: 0  Saves: 0  Cards: 0           │");
        
        // Match events involving player
        println!("│ Key Events:                                             │");
        let player_events: Vec<_> = game_match.events
            .iter()
            .filter(|event| event.player_involved == player.id)
            .take(3)  // Show first 3 events
            .collect();
        
        if player_events.is_empty() {
            println!("│ No significant events                                    │");
        } else {
            for event in player_events {
                println!("│ - {:?} in the {}' minute                           │", 
                         event.event_type, event.minute);
            }
        }
        
        println!("└─────────────────────────────────────────────────────────┘");
    }

    /// Displays league table
    pub fn display_league_table(&self, competition: &Competition, seen_states: &mut HashMap<String, bool>) {
        println!("┌─────────────────────────────────────────────────────────┐");
        println!("│                      LEAGUE TABLE                       │");
        println!("├────┬────────────────────────────┬──────┬────┬────┬────┤");
        
        // Show tutorial if first time
        self.show_tutorial_if_needed("league_table", seen_states);
        
        println!("│ Pos│ Club                       │ Pts  │ GF │ GA │ GD │");
        println!("├────┼────────────────────────────┼──────┼────┼────┼────┤");
        
        for standing in &competition.standings {
            println!("│ {:>2} │ {:<25} │ {:>4} │ {:>2} │ {:>2} │ {:>3} │",
                     standing.position,
                     self.get_team_name_by_id(competition, standing.team_id),
                     standing.points,
                     standing.goals_for,
                     standing.goals_against,
                     standing.goal_difference);
        }
        
        println!("└────┴────────────────────────────┴──────┴────┴────┴────┘");
    }

    /// Helper to get team name by ID
    fn get_team_name_by_id(&self, _competition: &Competition, team_id: Uuid) -> String {
        // In a real implementation, this would look up the team name
        format!("Team {}", team_id.as_u128() % 1000)  // Placeholder
    }

    /// Displays main menu
    pub fn display_main_menu(&self) -> MainMenuOption {
        println!("\n┌─────────────────────────────────────────────────────────┐");
        println!("│                        MAIN MENU                        │");
        println!("├─────────────────────────────────────────────────────────┤");
        println!("│ 1. View Player Profile                                  │");
        println!("│ 2. View Team Information                                │");
        println!("│ 3. View League Table                                    │");
        println!("│ 4. View Match Report                                    │");
        println!("│ 5. Continue Game                                        │");
        println!("│ 6. Save Game                                            │");
        println!("│ 7. Load Game                                            │");
        println!("│ 8. Quit                                                 │");
        println!("└─────────────────────────────────────────────────────────┘");
        
        print!("Select an option (1-8): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim() {
            "1" => MainMenuOption::ViewPlayerProfile,
            "2" => MainMenuOption::ViewTeamInfo,
            "3" => MainMenuOption::ViewLeagueTable,
            "4" => MainMenuOption::ViewMatchReport,
            "5" => MainMenuOption::ContinueGame,
            "6" => MainMenuOption::SaveGame,
            "7" => MainMenuOption::LoadGame,
            "8" => MainMenuOption::Quit,
            _ => {
                println!("Invalid option. Continuing game...");
                MainMenuOption::ContinueGame
            }
        }
    }
    
    /// Shows a tutorial if it hasn't been seen yet
    pub fn show_tutorial_if_needed(&self, key: &str, seen_states: &mut HashMap<String, bool>) {
        if !seen_states.contains_key(key) {
            self.show_tutorial(key);
            seen_states.insert(key.to_string(), true);
        }
    }

    /// Shows a specific tutorial guide
    pub fn show_tutorial(&self, key: &str) {
        if let Some(guide) = self.tutorial_manager.get_guide(key) {
            println!("\n💡 GUIDE: {}", guide.title);
            println!("─────────────────────────────────────────────────────────");
            println!("{}", guide.content);
            println!("─────────────────────────────────────────────────────────\n");
        }
    }
}

/// Main menu options
#[derive(Debug, Clone, Copy)]
pub enum MainMenuOption {
    ViewPlayerProfile,
    ViewTeamInfo,
    ViewLeagueTable,
    ViewMatchReport,
    ContinueGame,
    SaveGame,
    LoadGame,
    Quit,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{Position, Foot, CareerStats, SquadRole, HiddenAttributes};
    use crate::systems::social_system::ManagerProfile;
    use chrono::NaiveDate;

    #[test]
    fn test_display_player_profile() {
        let ui = ConsoleUI::new(
            crate::core::time_engine::TimeEngine::new(chrono::Utc::now()),
            crate::core::event_engine::EventEngine::new(),
        );
        
        // Initialize player with empty tutorial state
        let mut player = create_test_player();
        player.tutorial_state = HashMap::new();
        
        // This would normally print to stdout, but we can at least test that it doesn't panic
        ui.display_player_profile(&mut player);
    }

    #[test]
    fn test_display_team_info() {
        let ui = ConsoleUI::new(
            crate::core::time_engine::TimeEngine::new(chrono::Utc::now()),
            crate::core::event_engine::EventEngine::new(),
        );
        
        let team = create_test_team();
        let mut seen_states = HashMap::new();
        
        // This would normally print to stdout, but we can at least test that it doesn't panic
        ui.display_team_info(&team, &mut seen_states);
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
            tutorial_state: std::collections::HashMap::new(),
        }
    }

    fn create_test_contract() -> crate::entities::Contract {
        crate::entities::Contract {
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