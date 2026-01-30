use player_manager::entities::{
    Player, Team, Competition, Position, Foot, CareerStats, Contract, 
    SquadRole, HiddenAttributes, Finances, Facilities, 
    CurrentSeason
};
use player_manager::core::{TimeEngine, EventEngine, game_state::GameState};
use player_manager::ui::ConsoleUI;
use chrono::{NaiveDate, Datelike};
use std::collections::HashMap;
use std::io::{self, Write};
use uuid::Uuid;

fn main() {
    println!("⚽ Starting From Boots to Ballon d'Or - Football Career Simulator!");
    println!("--------------------------------------------------");
    
    // Get player name
    print!("Enter your player's name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();
    let name = if name.is_empty() { "Alex Johnson" } else { name };

    // Get nationality
    print!("Enter your nationality (e.g., English, Spanish, Brazilian): ");
    io::stdout().flush().unwrap();
    let mut nationality = String::new();
    io::stdin().read_line(&mut nationality).expect("Failed to read line");
    let nationality = nationality.trim();
    let nationality = if nationality.is_empty() { "English" } else { nationality };

    // Get age
    let age: u8 = loop {
        print!("Enter your starting age (15-17): ");
        io::stdout().flush().unwrap();
        let mut age_str = String::new();
        io::stdin().read_line(&mut age_str).expect("Failed to read line");
        match age_str.trim().parse::<u8>() {
            Ok(a) if (15..=17).contains(&a) => {
                break a;
            }
            _ => println!("Invalid age. Please enter a number between 15 and 17."),
        }
    };

    // Get position
    println!("Select your preferred position:");
    println!("1. GK (Goalkeeper)");
    println!("2. RB (Right Back)");
    println!("3. CB (Center Back)");
    println!("4. LB (Left Back)");
    println!("5. DM (Defensive Midfielder)");
    println!("6. RM (Right Midfield)");
    println!("7. CM (Center Midfield)");
    println!("8. LM (Left Midfield)");
    println!("9. AM (Attacking Midfielder)");
    println!("10. RW (Right Wing)");
    println!("11. LW (Left Wing)");
    println!("12. CF (Center Forward)");
    println!("13. SS (Secondary Striker)");

    let position = loop {
        print!("Enter choice (1-13): ");
        io::stdout().flush().unwrap();
        let mut pos_str = String::new();
        io::stdin().read_line(&mut pos_str).expect("Failed to read line");
        match pos_str.trim().parse::<u32>() {
            Ok(1) => break Position::GK,
            Ok(2) => break Position::RB,
            Ok(3) => break Position::CB,
            Ok(4) => break Position::LB,
            Ok(5) => break Position::DM,
            Ok(6) => break Position::RM,
            Ok(7) => break Position::CM,
            Ok(8) => break Position::LM,
            Ok(9) => break Position::AM,
            Ok(10) => break Position::RW,
            Ok(11) => break Position::LW,
            Ok(12) => break Position::CF,
            Ok(13) => break Position::SS,
            _ => println!("Invalid choice. Please enter a number between 1 and 13."),
        }
    };

    // Initialize all core systems
    let time_engine = TimeEngine::new(chrono::Utc::now());
    let event_engine = EventEngine::new();
    
    // Initialize teams and competitions first to get IDs
    let team = create_sample_team();
    let team_id = team.id;
    let teams = vec![team];
    let competitions = vec![create_sample_competition(team_id)];
    
    // Create a starting player with the correct club ID
    let player = create_starting_player(name.to_string(), nationality.to_string(), age, position, team_id);
    
    // Create initial game state
    let game_state = GameState::new(player, team_id);
    
    // Initialize UI
    let mut ui = ConsoleUI::new(time_engine, event_engine);
    
    // Run the main game loop
    ui.run_main_loop(game_state.player, teams, competitions);
    
    println!("Thanks for playing!");
}

fn create_starting_player(name: String, nationality: String, age: u8, position: Position, club_id: Uuid) -> Player {
    let birth_year = chrono::Utc::now().date_naive().year() - age as i32;
    Player {
        id: Uuid::new_v4(),
        name,
        age,
        birth_date: NaiveDate::from_ymd_opt(birth_year, 6, 15).unwrap(),
        nationality,
        height: 178,
        weight: 72,
        preferred_foot: Foot::Right,
        primary_position: position,
        secondary_positions: vec![],
        technical: player_manager::entities::TechnicalAttributes {
            dribbling: 65,
            passing: 70,
            shooting: 60,
            first_touch: 68,
            tackling: 65,
            crossing: 55,
        },
        physical: player_manager::entities::PhysicalAttributes {
            pace: 60,
            stamina: 70,
            strength: 65,
            agility: 65,
            jumping: 60,
        },
        mental: player_manager::entities::MentalAttributes {
            composure: 65,
            vision: 70,
            work_rate: 75,
            determination: 75,
            positioning: 68,
            teamwork: 70,
        },
        hidden: HiddenAttributes {
            injury_proneness: 15,
            consistency: 65,
            big_match_temperament: 70,
            professionalism: 80,
            potential_ceiling: 88,
            versatility: 70,
            ambition: 85,
            loyalty: 60,
            ego: 65,
        },
        fitness: 85.0,
        fatigue: 10.0,
        form: 6.8,
        morale: 75.0,
        sharpness: 80.0,
        local_reputation: 30.0,
        international_reputation: 5.0,
        contract: Contract {
            club_id,
            wage: 10000.0,
            length_years: 2,
            squad_role: SquadRole::Prospect,
            release_clause: Some(2000000.0),
            performance_bonuses: vec![],
            contract_end_date: NaiveDate::from_ymd_opt(2026, 6, 15).unwrap(),
            league_strength: 50.0,
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
        form_history: vec![6.5, 6.8, 7.0, 6.7, 6.9],
        tutorial_state: HashMap::new(),
    }
}

fn create_sample_team() -> Team {
    Team {
        id: Uuid::new_v4(),
        name: "Manchester United".to_string(),
        country: "England".to_string(),
        city: "Manchester".to_string(),
        reputation: 90.0,
        finances: Finances {
            balance: 100_000_000.0,
            weekly_wage_bill: 4_000_000.0,
            revenue_per_week: 5_000_000.0,
            debt: 500_000_000.0,
        },
        financial_power: 95.0,
        youth_focus: 70.0,
        tactical_identity: "Attacking".to_string(),
        facilities_quality: 90.0,
        medical_quality: 85.0,
        squad: vec![Uuid::new_v4()], // Just a dummy ID for now
        staff: vec![],
        youth_academy_level: 9,
        facilities: Facilities {
            training_ground_quality: 9,
            stadium_capacity: 74000,
            stadium_quality: 9,
            youth_facilities: 9,
        },
    }
}

fn create_sample_competition(team_id: Uuid) -> Competition {
    Competition {
        id: Uuid::new_v4(),
        name: "Premier League".to_string(),
        country: "England".to_string(),
        competition_type: player_manager::entities::CompetitionType::League,
        level: 1,
        teams: vec![team_id],
        current_season: CurrentSeason {
            start_date: chrono::Utc::now().date_naive(),
            end_date: (chrono::Utc::now() + chrono::Duration::days(365)).date_naive(),
            current_matchday: 1,
            is_active: true,
        },
        fixtures: vec![],
        standings: vec![],
        season_start: chrono::Utc::now().date_naive(),
        season_end: (chrono::Utc::now() + chrono::Duration::days(365)).date_naive(),
    }
}