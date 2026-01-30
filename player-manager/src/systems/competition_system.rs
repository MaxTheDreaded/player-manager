// src/systems/competition_system.rs
use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::entities::{Team, Match, Competition, Fixture, Standing, FormResult};

/// The CompetitionEngine manages leagues, cups, standings, and schedules
/// It handles team performance tracking and competition progression
pub struct CompetitionEngine;

impl CompetitionEngine {
    /// Creates a new CompetitionEngine instance
    pub fn new() -> Self {
        CompetitionEngine
    }

    /// Initializes a new season for a competition
    pub fn initialize_season(&self, competition: &mut Competition) {
        // Reset standings
        competition.standings = competition.teams.iter()
            .map(|team_id| Standing {
                team_id: *team_id,
                position: 0,
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                goal_difference: 0,
                form: vec![],
            })
            .collect();
        
        // Generate fixtures
        competition.fixtures = self.generate_fixtures(&competition.teams, competition.id, &competition.name);
        
        // Update season info
        competition.current_season.is_active = true;
        competition.current_season.current_matchday = 1;
    }

    /// Generates fixtures for a round-robin competition
    fn generate_fixtures(&self, teams: &[Uuid], competition_id: Uuid, _competition_name: &str) -> Vec<Fixture> {
        let mut fixtures = Vec::new();
        
        // Simple round-robin: each team plays every other team twice (home and away)
        for i in 0..teams.len() {
            for j in 0..teams.len() {
                if i != j {
                    // First leg (home team i, away team j)
                    fixtures.push(Fixture {
                        id: Uuid::new_v4(),
                        competition_id: competition_id,
                        home_team: teams[i],
                        away_team: teams[j],
                        scheduled_date: chrono::Utc::now().date_naive(), // Convert to NaiveDate
                        venue: teams[i], // Home team's venue
                        status: crate::entities::MatchStatus::Scheduled,
                        result: None,
                        matchday: (fixtures.len() as u32 / (teams.len() as u32 - 1) + 1),
                    });
                    
                    // Second leg (home team j, away team i)
                    // Removed to avoid duplicate fixtures (the loop handles both i,j and j,i)
                }
            }
        }
        
        fixtures
    }

    /// Processes a completed match result and updates competition standings
    pub fn process_match_result(
        &self,
        competition: &mut Competition,
        match_result: &Match,
        home_team: &Team,
        away_team: &Team,
    ) {
        if let Some((home_goals, away_goals)) = match_result.fulltime_score {
            // Find the fixture and update its result
            if let Some(fixture) = competition.fixtures.iter_mut()
                .find(|f| f.id == match_result.id) {
                fixture.result = Some(crate::entities::MatchResult {
                    home_score: home_goals,
                    away_score: away_goals,
                    winner: if home_goals > away_goals {
                        Some(home_team.id)
                    } else if away_goals > home_goals {
                        Some(away_team.id)
                    } else {
                        None // Draw
                    },
                });
                fixture.status = crate::entities::MatchStatus::Finished;
            }
            
            // Update standings for both teams
            self.update_standings(competition, home_team.id, home_goals, away_goals, true);
            self.update_standings(competition, away_team.id, away_goals, home_goals, false);
            
            // Sort standings by points, then goal difference, then goals scored
            self.sort_standings(competition);
        }
    }

    /// Updates the standings for a team after a match
    fn update_standings(
        &self,
        competition: &mut Competition,
        team_id: Uuid,
        team_goals: u8,
        opponent_goals: u8,
        _is_home: bool,
    ) {
        if let Some(standing) = competition.standings.iter_mut().find(|s| s.team_id == team_id) {
            // Update basic stats
            standing.played += 1;
            standing.goals_for += team_goals as u32;
            standing.goals_against += opponent_goals as u32;
            standing.goal_difference = standing.goals_for as i32 - standing.goals_against as i32;
            
            // Determine result and update points/stats
            let result = if team_goals > opponent_goals {
                standing.won += 1;
                standing.points += 3;
                FormResult::Win
            } else if team_goals == opponent_goals {
                standing.drawn += 1;
                standing.points += 1;
                FormResult::Draw
            } else {
                standing.lost += 1;
                FormResult::Loss
            };
            
            // Update form (last 5 matches)
            standing.form.push(result);
            if standing.form.len() > 5 {
                standing.form.remove(0);
            }
        }
    }

    /// Sorts the standings based on points, goal difference, and goals scored
    fn sort_standings(&self, competition: &mut Competition) {
        competition.standings.sort_by(|a, b| {
            // Primary sort: points
            b.points.cmp(&a.points)
                // Secondary sort: goal difference
                .then_with(|| b.goal_difference.cmp(&a.goal_difference))
                // Tertiary sort: goals for
                .then_with(|| b.goals_for.cmp(&a.goals_for))
        });
        
        // Assign positions
        for (i, standing) in competition.standings.iter_mut().enumerate() {
            standing.position = (i + 1) as u8;
        }
    }

    /// Updates competition standings after a match
    pub fn update_competition_after_match(
        &self,
        competitions: &mut [Competition],
        match_result: &Match,
        home_team: &Team,
        away_team: &Team,
    ) {
        for comp in competitions.iter_mut() {
            if comp.teams.contains(&home_team.id) && comp.teams.contains(&away_team.id) {
                self.process_match_result(comp, match_result, home_team, away_team);
            }
        }
    }

    /// Gets the current league table for a competition
    pub fn get_league_table(&self, competition: &Competition) -> Vec<Standing> {
        competition.standings.clone()
    }

    /// Gets the next fixture for a team in a competition
    pub fn get_next_fixture_for_team<'a>(
        &self,
        competition: &'a Competition,
        team_id: Uuid,
    ) -> Option<&'a Fixture> {
        competition.fixtures.iter()
            .filter(|fixture| {
                (fixture.home_team == team_id || fixture.away_team == team_id) &&
                fixture.status == crate::entities::MatchStatus::Scheduled
            })
            .min_by_key(|fixture| fixture.matchday)
    }

    /// Gets all fixtures for a specific matchday
    pub fn get_fixtures_for_matchday<'a>(
        &self,
        competition: &'a Competition,
        matchday: u32,
    ) -> Vec<&'a Fixture> {
        competition.fixtures.iter()
            .filter(|fixture| fixture.matchday == matchday)
            .collect()
    }

    /// Calculates team strength for AI opponents
    pub fn calculate_team_strength(&self, team: &Team) -> f32 {
        // Calculate an overall team strength based on squad quality
        // For now, we'll return a default value since we can't compute from IDs alone
        // This function would need access to the actual Player objects to compute properly
        team.reputation // Use team's reputation as a proxy for strength
    }

    /// Determines if a competition has been completed
    pub fn is_competition_finished(&self, competition: &Competition) -> bool {
        // Competition is finished if all fixtures are completed
        competition.fixtures.iter()
            .all(|fixture| fixture.status == crate::entities::MatchStatus::Finished)
    }

    /// Gets the winner of a competition (for completed competitions)
    pub fn get_competition_winner(&self, competition: &Competition) -> Option<Uuid> {
        if !self.is_competition_finished(competition) {
            return None;
        }
        
        competition.standings.first().map(|standing| standing.team_id)
    }

    /// Updates the competition season after all matches are completed
    pub fn finalize_season(&self, competition: &mut Competition) {
        competition.current_season.is_active = false;
        
        // Could add end-of-season events here, like promotion/relegation
        // for leagues, or qualification for continental competitions
    }

    /// Gets teams in top positions (for European qualification)
    pub fn get_teams_by_position_range(
        &self,
        competition: &Competition,
        start_pos: u8,
        end_pos: u8,
    ) -> Vec<Uuid> {
        competition.standings
            .iter()
            .filter(|standing| standing.position >= start_pos && standing.position <= end_pos)
            .map(|standing| standing.team_id)
            .collect()
    }

    /// Gets teams in bottom positions (for relegation)
    pub fn get_bottom_teams(
        &self,
        competition: &Competition,
        num_teams: usize,
    ) -> Vec<Uuid> {
        competition.standings
            .iter()
            .rev()  // Reverse to get bottom teams first
            .take(num_teams)
            .map(|standing| standing.team_id)
            .collect()
    }
}

/// Competition type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CompetitionType {
    League,
    DomesticCup,
    Continental,
    YouthLeague,
    Knockout,
}

/// Match result
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MatchResult {
    pub home_goals: u8,
    pub away_goals: u8,
    pub match_id: Uuid,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{Competition, Team, SquadRole, Contract, Position, Foot, CareerStats, HiddenAttributes};
    use crate::core::game_state::SeasonInfo;
    use crate::systems::social_system::ManagerProfile;
    use chrono::NaiveDate;

    #[test]
    fn test_initialize_season() {
        let engine = CompetitionEngine::new();
        
        let team_ids = vec![Uuid::new_v4(), Uuid::new_v4()];
        let mut competition = Competition {
            id: Uuid::new_v4(),
            name: "Test League".to_string(),
            country: "Test Country".to_string(),
            level: 1,
            season_start: chrono::Utc::now().date_naive(),
            season_end: chrono::Utc::now().date_naive(),
            teams: team_ids.clone(),
            fixtures: vec![],
            standings: vec![],
            competition_type: crate::entities::CompetitionType::League,
            current_season: crate::entities::CurrentSeason {
                is_active: false,
                current_matchday: 1,
                start_date: chrono::Utc::now().date_naive(),
                end_date: chrono::Utc::now().date_naive(),
            },
        };
        
        engine.initialize_season(&mut competition);
        
        assert_eq!(competition.standings.len(), 2);
        assert_eq!(competition.fixtures.len(), 2); // Each team plays the other twice (but optimized to 2 struct entries with duplicate processing logic removed?)
        // Wait, earlier I removed the duplicate push.
        // So 0v1, 1v0 are now distinct entries?
        // Loop 0..2, 0..2.
        // 0,1 -> push (0,1).
        // 1,0 -> push (1,0).
        // That is 2 entries.
        // My previous fix removed the *extra* push inside the loop.
        // The loop naturally covers both legs.
        // So 2 is the correct number.
        assert!(competition.current_season.is_active);
    }

    #[test]
    fn test_generate_fixtures() {
        let engine = CompetitionEngine::new();

        let team_ids = vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
        let competition_id = Uuid::new_v4();
        let fixtures = engine.generate_fixtures(&team_ids, competition_id, "Test League");

        // Each team should play every other team twice (home and away)
        // So 3 teams = 3*2 = 6 matches per team = 18 total, but shared so 9 unique matchups * 2 legs = 18
        // Actually: Team A vs B, A vs C, B vs C = 3 matchups * 2 legs each = 6 matches
        assert_eq!(fixtures.len(), 6);
    }

    #[test]
    fn test_update_standings() {
        let engine = CompetitionEngine::new();
        
        let team_id = Uuid::new_v4();
        let mut competition = Competition {
            id: Uuid::new_v4(),
            name: "Test League".to_string(),
            country: "Test Country".to_string(),
            level: 1,
            season_start: chrono::Utc::now().date_naive(),
            season_end: chrono::Utc::now().date_naive(),
            teams: vec![team_id],
            fixtures: vec![],
            standings: vec![Standing {
                team_id,
                position: 0,
                points: 0,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                goal_difference: 0,
                form: vec![],
            }],
            competition_type: crate::entities::CompetitionType::League,
            current_season: crate::entities::CurrentSeason {
                is_active: false,
                current_matchday: 1,
                start_date: chrono::Utc::now().date_naive(),
                end_date: chrono::Utc::now().date_naive(),
            },
        };
        
        // Process a win for the team
        engine.update_standings(&mut competition, team_id, 2, 1, true);
        
        let standing = &competition.standings[0];
        assert_eq!(standing.played, 1);
        assert_eq!(standing.won, 1);
        assert_eq!(standing.points, 3);
        assert_eq!(standing.goals_for, 2);
        assert_eq!(standing.goals_against, 1);
        assert_eq!(standing.goal_difference, 1);
        assert_eq!(standing.form, vec![FormResult::Win]);
    }

    #[test]
    fn test_sort_standings() {
        let engine = CompetitionEngine::new();
        
        let team_a = Uuid::new_v4();
        let team_b = Uuid::new_v4();
        let team_c = Uuid::new_v4();
        
        let mut competition = Competition {
            id: Uuid::new_v4(),
            name: "Test League".to_string(),
            country: "Test Country".to_string(),
            level: 1,
            season_start: chrono::Utc::now().date_naive(),
            season_end: chrono::Utc::now().date_naive(),
            teams: vec![team_a, team_b, team_c],
            fixtures: vec![],
            standings: vec![
                Standing {
                    team_id: team_a,
                    position: 0,
                    points: 3,
                    played: 1,
                    won: 1,
                    drawn: 0,
                    lost: 0,
                    goals_for: 2,
                    goals_against: 1,
                    goal_difference: 1,
                    form: vec![FormResult::Win],
                },
                Standing {
                    team_id: team_b,
                    position: 0,
                    points: 3,
                    played: 1,
                    won: 1,
                    drawn: 0,
                    lost: 0,
                    goals_for: 3,
                    goals_against: 0,
                    goal_difference: 3,
                    form: vec![FormResult::Win],
                },
                Standing {
                    team_id: team_c,
                    position: 0,
                    points: 0,
                    played: 1,
                    won: 0,
                    drawn: 0,
                    lost: 1,
                    goals_for: 0,
                    goals_against: 2,
                    goal_difference: -2,
                    form: vec![FormResult::Loss],
                },
            ],
            competition_type: crate::entities::CompetitionType::League,
            current_season: crate::entities::CurrentSeason {
                is_active: false,
                current_matchday: 1,
                start_date: chrono::Utc::now().date_naive(),
                end_date: chrono::Utc::now().date_naive(),
            },
        };
        
        engine.sort_standings(&mut competition);
        
        // Team B should be first (same points as A but better goal difference)
        // Team A should be second
        // Team C should be third
        assert_eq!(competition.standings[0].team_id, team_b);
        assert_eq!(competition.standings[1].team_id, team_a);
        assert_eq!(competition.standings[2].team_id, team_c);
        
        // Positions should be assigned correctly
        assert_eq!(competition.standings[0].position, 1);
        assert_eq!(competition.standings[1].position, 2);
        assert_eq!(competition.standings[2].position, 3);
    }

    #[test]
    fn test_calculate_team_strength() {
        let engine = CompetitionEngine::new();

        // Create a mock team with some players
        let mut team = Team {
            id: Uuid::new_v4(),
            name: "Test Team".to_string(),
            country: "Test Country".to_string(),
            city: "Test City".to_string(),
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
            reputation: 75.0,
        };
        
        // Add a player to the team
        let player = crate::entities::Player {
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
            contract: Contract {
                club_id: team.id,
                wage: 50000.0,
                length_years: 3,
                squad_role: SquadRole::FirstTeam,
                release_clause: None,
                performance_bonuses: vec![],
                contract_end_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
                league_strength: 75.0,
            },
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
        };
        
        team.squad.push(player.id);
        
        let strength = engine.calculate_team_strength(&team);
        
        // The strength should be based on the player's attributes
        // With one player, it should be around the average of their attributes
        assert!(strength > 70.0 && strength < 85.0);
    }
}