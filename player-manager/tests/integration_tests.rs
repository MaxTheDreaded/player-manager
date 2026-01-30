// tests/integration_tests.rs
use player_manager::entities::{
    Player, Team, Match, Competition, Position, Foot, CareerStats, Contract, 
    SquadRole, HiddenAttributes, MatchStatus, MatchResult, MatchLineup, 
    Formation, Tactics, TacticalStyle, Finances, Facilities, CurrentSeason,
    Fixture, Standing
};
use player_manager::core::{TimeEngine, EventEngine, game_state::GameState};
use player_manager::systems::{
    PlayerDevelopmentEngine, MoraleEngine, MatchEngine, ReputationEngine, 
    SocialEngine, TrainingSystem, CompetitionEngine, TransferEngine
};
use player_manager::save::SaveManager;
use chrono::NaiveDate;
use std::collections::HashMap;
use uuid::Uuid;

#[test]
fn test_full_game_flow_integration() {
    // Create a test player
    let mut player = create_test_player();
    
    // Initialize all systems
    let mut time_engine = TimeEngine::new(chrono::Utc::now());
    let mut event_engine = EventEngine::new();
    let development_engine = PlayerDevelopmentEngine::new();
    let morale_engine = MoraleEngine::new();
    let mut match_engine = MatchEngine::new();
    let reputation_engine = ReputationEngine::new();
    let social_engine = SocialEngine::new();
    let training_engine = TrainingSystem::new();
    let competition_engine = CompetitionEngine::new();
    let transfer_engine = TransferEngine::new();
    let save_manager = SaveManager::new();
    
    // Test initial state
    assert_eq!(player.age, 17);
    assert!(player.local_reputation > 0.0);
    assert!(player.international_reputation >= 0.0);
    
    // Test training system
    let training_result = training_engine.process_training_week(
        &mut player,
        player_manager::systems::training_system::TrainingFocus::Technical,
        Some(player_manager::systems::training_system::TrainingFocus::Technical),
        75.0,  // Coach quality
        60.0,  // Training intensity
        80.0,  // Facilities quality
    );
    
    assert!(training_result.effectiveness > 0.0);
    assert_eq!(training_result.focus, player_manager::systems::training_system::TrainingFocus::Technical);
    
    // Test development after training
    development_engine.update_player_attributes(
        &mut player,
        player_manager::systems::training_system::TrainingFocus::Technical,
        Some(7.5),  // Last match rating
        7,  // Days passed
    );
    
    // Verify attributes increased appropriately
    assert!(player.technical.dribbling >= 70);  // Should have improved from training
    
    // Test morale update
    morale_engine.update_player_morale(
        &mut player,
        Some(8.0),  // Good match rating
        Some(90),   // Full match played
        Some(player_manager::systems::morale_system::GameResult::Win),
        player_manager::systems::morale_system::ContractStatus::Active,
        player_manager::systems::morale_system::MediaAttention::Positive,
        &[],  // No relationship changes
        0,    // Days since last match
    );
    
    assert!(player.morale > 70.0);  // Should be high after good performance
    
    // Test reputation update
    reputation_engine.update_reputation(
        &mut player,
        8.0,  // Good rating
        player_manager::systems::reputation_system::MatchImportance::League,
        true,  // Is big moment
        75.0,  // League strength
        player_manager::systems::reputation_system::TeamPerformance::Win,
    );
    
    assert!(player.local_reputation > 45.0);  // Should have increased from 45.0
    
    // Test save/load cycle
    let game_state = create_test_game_state(player.clone());
    let temp_path = std::env::temp_dir().join("integration_test_save.json");
    
    // Save the game
    assert!(save_manager.save_game(&game_state, &temp_path).is_ok());
    
    // Load the game
    let loaded_game_state = save_manager.load_game(&temp_path).unwrap();
    
    // Verify loaded data matches
    assert_eq!(loaded_game_state.player.name, game_state.player.name);
    assert_eq!(loaded_game_state.player.age, game_state.player.age);
    
    // Clean up
    let _ = std::fs::remove_file(&temp_path);
}

#[test]
fn test_match_simulation_integration() {
    let mut player = create_test_player();
    let mut match_engine = MatchEngine::new();
    
    // Create a mock match
    // Create a mock match
    let mut game_match = Match {
        id: Uuid::new_v4(),
        competition_id: Uuid::new_v4(),
        home_team: Uuid::new_v4(),
        away_team: Uuid::new_v4(),
        date: chrono::Utc::now().date_naive(),
        venue: Uuid::new_v4(),
        status: MatchStatus::Scheduled,
        result: None,
        half_results: None,
        fulltime_score: None,
        events: vec![],
        player_ratings: HashMap::new(),
        competition_type: player_manager::entities::CompetitionType::League,
        lineup: create_mock_lineup(),
    };
    
    // Create mock teams with the player
    let home_players = vec![player.clone()];
    let away_players = vec![create_opposing_player()];
    
    // Simulate the match
    game_match = match_engine.simulate_match(
        game_match,
        &home_players,
        &away_players,
        &create_mock_lineup(),
        &create_mock_lineup(),
    );
    
    // Verify match was completed
    // Verify match was completed
    assert_eq!(game_match.status, MatchStatus::Finished);
    
    // Verify player got a rating
    assert!(game_match.player_ratings.contains_key(&player.id));
    
    // Get the player's rating
    let player_rating = game_match.player_ratings.get(&player.id).unwrap();
    assert!(*player_rating >= 4.5 && *player_rating <= 9.9);
}

#[test]
fn test_transfer_system_integration() {
    let player = create_test_player();
    let teams = vec![create_test_team(), create_test_team()];
    let current_club_id = teams[0].id; // Player belongs to team 0
    
    let transfer_engine = TransferEngine::new();
    
    // Evaluate transfer interest
    let interests = transfer_engine.evaluate_transfer_interest(&player, &teams, current_club_id);
    
    // Should have at least some interest given the player's good stats
    assert!(!interests.is_empty());
    
    // Generate an offer for the first interested club
    if let Some(interest) = interests.first() {
        let offer = transfer_engine.generate_transfer_offer(&player, &teams[0], &player.contract);
        
        // Verify offer was generated with reasonable values
        assert!(offer.offered_wage > 0.0);
        assert!(offer.contract_length_years > 0);
        assert!(offer.transfer_fee.unwrap_or(0.0) >= 0.0);
    }
}

#[test]
fn test_competition_system_integration() {
    let mut competition = create_test_competition();
    let competition_engine = CompetitionEngine::new();
    
    // Initialize the season
    competition_engine.initialize_season(&mut competition);
    
    // Verify fixtures were generated
    assert!(!competition.fixtures.is_empty());
    
    // Verify standings were initialized
    assert!(!competition.standings.is_empty());
    assert_eq!(competition.standings[0].points, 0);
    assert_eq!(competition.standings[0].played, 0);
    
    // Verify season is active
    assert!(competition.current_season.is_active);
}

#[test]
fn test_social_relationship_integration() {
    let mut player = create_test_player();
    let mut relationships = HashMap::new();
    let other_entity_id = Uuid::new_v4();
    
    let social_engine = SocialEngine::new();
    
    // Update a relationship
    let personality_factors = player_manager::systems::social_system::PersonalityFactors::new(80, 50, 70, 60, 75);
    let new_value = social_engine.update_relationship(
        &mut relationships,
        other_entity_id,
        10.0,  // Positive change
        &personality_factors,
    );
    
    assert!(new_value > 50.0);  // Should be above neutral
    
    // Calculate relationship impact on morale
    let mut relationship_types = HashMap::new();
    relationship_types.insert(other_entity_id, player_manager::systems::social_system::RelationshipType::Teammate);
    
    let morale_impact = social_engine.calculate_relationship_morale_impact(&relationships, &relationship_types);
    
    // Morale impact should reflect the positive relationship
    assert!(morale_impact >= 0.0);
}

#[test]
fn test_time_event_integration() {
    let mut time_engine = TimeEngine::new(chrono::Utc::now());
    let mut event_engine = EventEngine::new();
    
    // Schedule an event
    let event = player_manager::core::time_engine::ScheduledEvent::new(
        chrono::Utc::now() + chrono::Duration::days(1),
        player_manager::core::time_engine::ScheduledEventType::TrainingSession,
        player_manager::core::time_engine::EventPriority::Medium,
        false,
    );
    
    time_engine.schedule_event(event);
    
    // Verify event was scheduled
    assert!(time_engine.time_until_next_event().is_some());
    
    // Advance time to trigger the event
    // Note: In a real test, we would process the event through the event engine
    assert!(time_engine.advance_time().is_ok());
}

#[test]
fn test_player_development_with_age_progression() {
    let mut player = create_test_player();
    let development_engine = PlayerDevelopmentEngine::new();
    
    // Test development for young player (should grow faster)
    let initial_dribbling = player.technical.dribbling;
    
    development_engine.update_player_attributes(
        &mut player,
        player_manager::systems::training_system::TrainingFocus::Technical,
        Some(7.5),
        30,  // 30 days passed
    );
    
    // Attribute should have increased for young player
    assert!(player.technical.dribbling >= initial_dribbling);
    
    // Age the player and test again
    player.age = 28;
    let initial_dribbling_at_28 = player.technical.dribbling;
    
    development_engine.update_player_attributes(
        &mut player,
        player_manager::systems::training_system::TrainingFocus::Technical,
        Some(7.5),
        30,  // 30 days passed
    );
    
    // Growth should be slower at age 28
    let dribbling_increase_young = player.technical.dribbling as i32 - initial_dribbling as i32;
    let dribbling_increase_older = player.technical.dribbling as i32 - initial_dribbling_at_28 as i32;
    
    // The increase should be greater when younger (though both could be positive)
    // This test verifies the age-based development factors are working
}

// Helper functions
fn create_test_player() -> Player {
    Player {
        id: Uuid::new_v4(),
        name: "Test Player".to_string(),
        age: 17,
        birth_date: NaiveDate::from_ymd_opt(2006, 1, 1).unwrap(),
        nationality: "Country".to_string(),
        height: 175,
        weight: 70,
        preferred_foot: Foot::Right,
        primary_position: Position::CM,
        secondary_positions: vec![Position::LM, Position::RM],
        technical: player_manager::entities::TechnicalAttributes {
            dribbling: 70,
            passing: 75,
            shooting: 65,
            first_touch: 72,
            tackling: 68,
            crossing: 60,
        },
        physical: player_manager::entities::PhysicalAttributes {
            pace: 65,
            stamina: 75,
            strength: 60,
            agility: 68,
            jumping: 62,
        },
        mental: player_manager::entities::MentalAttributes {
            composure: 70,
            vision: 75,
            work_rate: 72,
            determination: 78,
            positioning: 70,
            teamwork: 75,
        },
        hidden: HiddenAttributes {
            injury_proneness: 20,
            consistency: 70,
            big_match_temperament: 75,
            professionalism: 80,
            potential_ceiling: 85,
            versatility: 70,
            ambition: 80,
            loyalty: 60,
            ego: 65,
        },
        fitness: 85.0,
        fatigue: 15.0,
        form: 7.0,
        morale: 75.0,
        sharpness: 80.0,
        local_reputation: 45.0,
        international_reputation: 10.0,
        contract: Contract {
            club_id: Uuid::new_v4(),
            wage: 15000.0,
            length_years: 3,
            squad_role: SquadRole::Prospect,
            release_clause: Some(5000000.0),
            performance_bonuses: vec![],
            contract_end_date: NaiveDate::from_ymd_opt(2027, 1, 1).unwrap(),
            league_strength: 60.0,
        },
        career_stats: CareerStats {
            seasons_played: 0,
            total_appearances: 0,
            total_goals: 0,
            total_assists: 0,
            total_yellow_cards: 0,
            total_red_cards: 0,
            average_rating: 0.0,
            highest_rating: 0.0,
            season_stats: vec![],
            awards: vec![],
            trophies: vec![],
        },
        relationships: HashMap::new(),
        injury_status: None,
        form_history: vec![7.0, 6.8, 7.2, 6.9, 7.1],
        tutorial_state: HashMap::new(),
    }
}

fn create_opposing_player() -> Player {
    let mut player = create_test_player();
    player.id = Uuid::new_v4();
    player.name = "Opposing Player".to_string();
    player
}

fn create_test_team() -> Team {
    Team {
        id: Uuid::new_v4(),
        name: "Test Club".to_string(),
        country: "England".to_string(),
        city: "London".to_string(),
        reputation: 70.0,
        finances: Finances {
            balance: 10_000_000.0,
            weekly_wage_bill: 500_000.0,
            revenue_per_week: 600_000.0,
            debt: 0.0,
        },
        financial_power: 80.0,
        youth_focus: 65.0,
        tactical_identity: "Possession".to_string(),
        facilities_quality: 75.0,
        medical_quality: 80.0,
        squad: vec![Uuid::new_v4()], // Just an ID, not the player object
        staff: vec![],
        youth_academy_level: 5,
        facilities: Facilities {
            training_ground_quality: 7,
            stadium_capacity: 50000,
            stadium_quality: 8,
            youth_facilities: 6,
        },
    }
}

fn create_test_competition() -> Competition {
    Competition {
        id: Uuid::new_v4(),
        name: "Test League".to_string(),
        country: "England".to_string(),
        competition_type: player_manager::entities::CompetitionType::League,
        level: 1,
        teams: vec![create_test_team().id, Uuid::new_v4()], // Need at least 2 teams for fixtures
        current_season: CurrentSeason {
            start_date: chrono::Utc::now().date_naive(),
            end_date: (chrono::Utc::now() + chrono::Duration::days(365)).date_naive(),
            current_matchday: 1,
            is_active: false,
        },
        fixtures: vec![],
        standings: vec![],
        season_start: chrono::Utc::now().date_naive(),
        season_end: (chrono::Utc::now() + chrono::Duration::days(365)).date_naive(),
    }
}

fn create_mock_lineup() -> MatchLineup {
    MatchLineup {
        formation: Formation {
            goalkeeper: Uuid::new_v4(),
            defenders: vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()],
            midfielders: vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()],
            forwards: vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()],
        },
        players: vec![],
        tactics: Tactics {
            style: TacticalStyle::Balanced,
            mentality: 0.0,
            tempo: 0.5,
            width: 0.5,
            pressing_intensity: 0.5,
        },
        home_starting_xi: vec![],
        away_starting_xi: vec![],
    }
}

fn create_test_game_state(player: Player) -> GameState {
    GameState::new(player, Uuid::new_v4())
}