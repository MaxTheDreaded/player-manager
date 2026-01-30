// src/systems/reputation_system.rs
use serde::{Deserialize, Serialize};


use crate::entities::Player;

/// The ReputationEngine manages both local and international reputation
/// It converts performances into reputation gains and handles the conversion
/// from local to international reputation over time
pub struct ReputationEngine;

impl ReputationEngine {
    /// Creates a new ReputationEngine instance
    pub fn new() -> Self {
        ReputationEngine
    }

    /// Updates player reputation based on match performance and other factors
    pub fn update_reputation(
        &self,
        player: &mut Player,
        match_rating: f32,
        match_importance: MatchImportance,
        is_big_moment: bool,
        league_strength: f32,  // 0-100 scale of league quality
        team_performance: TeamPerformance,
    ) {
        // Update local reputation based on match performance
        let local_change = self.calculate_local_reputation_change(
            match_rating,
            match_importance,
            is_big_moment,
            team_performance
        );
        player.local_reputation = (player.local_reputation + local_change).clamp(0.0, 100.0);

        // Convert local reputation to international reputation
        let international_gain = self.convert_local_to_international(
            player.local_reputation,
            league_strength,
            match_importance
        );
        
        // Apply international reputation change with decay consideration
        let international_change = international_gain - self.calculate_decay_factor(player.international_reputation);
        player.international_reputation = (player.international_reputation + international_change).clamp(0.0, 100.0);
    }

    /// Calculates local reputation change based on match performance
    fn calculate_local_reputation_change(
        &self,
        rating: f32,
        importance: MatchImportance,
        is_big_moment: bool,
        team_performance: TeamPerformance,
    ) -> f32 {
        // Base change based on match rating
        let base_change = match rating {
            r if r >= 9.0 => 3.0,
            r if r >= 8.0 => 2.0,
            r if r >= 7.0 => 1.0,
            r if r >= 6.5 => 0.2,
            r if r >= 6.0 => -0.5,
            r if r >= 5.0 => -1.0,
            _ => -2.0,
        };

        // Importance multiplier
        let importance_multiplier = match importance {
            MatchImportance::Friendly => 0.5,
            MatchImportance::League => 1.0,
            MatchImportance::Cup => 1.5,
            MatchImportance::Final => 2.0,
            MatchImportance::Continental => 2.5,
        };

        // Big moment bonus
        let big_moment_bonus = if is_big_moment { 1.0 } else { 0.0 };

        // Team performance modifier
        let team_modifier = match team_performance {
            TeamPerformance::Win => 0.5,
            TeamPerformance::Draw => 0.1,
            TeamPerformance::Loss => -0.3,
        };

        (base_change * importance_multiplier) + big_moment_bonus + team_modifier
    }

    /// Converts local reputation to international reputation
    fn convert_local_to_international(
        &self,
        local_rep: f32,
        league_strength: f32,
        match_importance: MatchImportance,
    ) -> f32 {
        // Higher league strength converts local buzz to international fame faster
        let league_factor = league_strength / 100.0;
        
        // Continental matches convert reputation faster
        let importance_factor = match match_importance {
            MatchImportance::Continental => 1.5,
            MatchImportance::Final => 1.3,
            _ => 1.0,
        };
        
        // Calculate conversion rate (local reputation * league strength * importance)
        (local_rep / 100.0) * league_factor * importance_factor * 0.5  // 0.5 is base conversion rate
    }

    /// Calculates decay factor for international reputation
    fn calculate_decay_factor(&self, international_rep: f32) -> f32 {
        // Higher international reputation decays slower
        // Lower reputation decays faster if player isn't performing
        if international_rep > 70.0 {
            0.01  // Very slow decay for top players
        } else if international_rep > 40.0 {
            0.02  // Slow decay for known players
        } else {
            0.05  // Faster decay for lesser known players
        }
    }

    /// Calculates the impact of reputation on transfer interest
    pub fn calculate_transfer_interest_score(
        &self,
        player: &Player,
        target_club_reputation: f32,  // 0-100 scale
        positional_needs: f32,       // 0-100 scale (how much club needs this position)
    ) -> f32 {
        // Base score from player attributes
        let ability_score = self.calculate_player_ability_score(player);
        let potential_score = (player.hidden.potential_ceiling as f32) / 2.0;  // 0-50 scale
        let form_score = player.form * 0.5;  // 0-50 scale
        let reputation_score = player.international_reputation * 0.7;  // 0-70 scale (international matters more for transfers)
        
        // Age factor (younger players more attractive)
        let age_factor = self.calculate_age_factor(player.age);
        
        // Calculate base interest score
        let mut interest_score = ability_score + potential_score + form_score + reputation_score;
        
        // Apply modifiers
        interest_score *= age_factor;
        interest_score *= 1.0 + (target_club_reputation / 200.0);  // Higher club reputation increases interest
        interest_score *= 1.0 + (positional_needs / 100.0);  // Positional need increases interest
        
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

    /// Calculates the impact of reputation on award chances
    pub fn calculate_award_contender_score(
        &self,
        player: &Player,
        season_performance: SeasonPerformance,
        team_success: TeamSuccess,
    ) -> f32 {
        // Factors that influence award chances
        let match_rating_avg = season_performance.average_match_rating;
        let goal_count = season_performance.goals as f32 * 0.1;  // Goals matter for awards
        let assist_count = season_performance.assists as f32 * 0.05;  // Assists matter too
        let clean_sheet_bonus = if player.primary_position.is_goalkeeper_or_defender() {
            season_performance.clean_sheets as f32 * 0.15
        } else {
            0.0
        };
        let team_success_factor = match team_success {
            TeamSuccess::LeagueWinner => 15.0,
            TeamSuccess::CupWinner => 10.0,
            TeamSuccess::Top4 => 5.0,
            TeamSuccess::MidTable => 0.0,
            TeamSuccess::Relegation => -5.0,
        };
        let reputation_boost = player.international_reputation * 0.2;  // International rep helps awards
        
        match_rating_avg + goal_count + assist_count + clean_sheet_bonus + 
        team_success_factor + reputation_boost
    }

    /// Updates reputation based on seasonal performance
    pub fn update_seasonal_reputation(
        &self,
        player: &mut Player,
        season_stats: &SeasonStats,
        team_finish_position: u8,
        awards: &[String],
    ) {
        // Calculate season performance score
        let performance_score = self.calculate_season_performance_score(season_stats, team_finish_position);
        
        // Apply seasonal reputation adjustments
        let seasonal_local_change = performance_score * 0.3;  // Season performance affects local rep
        player.local_reputation = (player.local_reputation + seasonal_local_change).clamp(0.0, 100.0);
        
        // Awards give significant reputation boosts
        for award in awards {
            let award_boost = self.get_award_reputation_boost(award);
            player.international_reputation = (player.international_reputation + award_boost).clamp(0.0, 100.0);
        }
        
        // Team success affects international reputation
        let team_success_boost = self.get_team_success_reputation_boost(team_finish_position);
        player.international_reputation = (player.international_reputation + team_success_boost).clamp(0.0, 100.0);
    }

    /// Calculates season performance score
    fn calculate_season_performance_score(&self, stats: &SeasonStats, team_position: u8) -> f32 {
        // Calculate based on key stats
        let appearance_factor = (stats.appearances as f32) * 0.1;
        let goal_factor = (stats.goals as f32) * 0.3;
        let assist_factor = (stats.assists as f32) * 0.2;
        let rating_factor = stats.average_rating * 2.0;
        let team_factor = match team_position {
            1 => 10.0,   // Champions
            2..=4 => 5.0, // Top 4
            5..=6 => 2.0, // European spots
            7..=15 => 0.0, // Mid table
            16..=17 => -2.0, // Relegation zone
            18..=20 => -5.0, // Relegated
            _ => 0.0,
        };
        
        appearance_factor + goal_factor + assist_factor + rating_factor + team_factor
    }

    /// Gets reputation boost for specific awards
    fn get_award_reputation_boost(&self, award: &str) -> f32 {
        match award.to_lowercase().as_str() {
            "ballon d'or" | "world player of the year" => 25.0,
            "league best player" => 15.0,
            "top scorer" => 10.0,
            "best young player" => 8.0,
            "team of the season" => 5.0,
            _ => 2.0,  // Other awards
        }
    }

    /// Gets reputation boost based on team success
    fn get_team_success_reputation_boost(&self, position: u8) -> f32 {
        match position {
            1 => 12.0,   // Champions
            2..=3 => 8.0, // Top 3
            4..=6 => 5.0, // European spots
            7..=10 => 2.0, // Mid table
            11..=17 => 0.0, // Lower mid table
            _ => -3.0,   // Relegation zone
        }
    }

    /// Calculates reputation decay when player is inactive
    pub fn apply_inactive_decay(&self, player: &mut Player, weeks_inactive: u32) {
        // Decay is more pronounced for international reputation
        let local_decay = (weeks_inactive as f32) * 0.1;  // 0.1 per week
        let international_decay = (weeks_inactive as f32) * 0.3;  // 0.3 per week (faster decay)
        
        player.local_reputation = (player.local_reputation - local_decay).max(0.0);
        player.international_reputation = (player.international_reputation - international_decay).max(0.0);
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

/// How the team performed in the match
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TeamPerformance {
    Win,
    Draw,
    Loss,
}

/// How the team performed in the season
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TeamSuccess {
    LeagueWinner,
    CupWinner,
    Top4,        // Qualification for continental competition
    MidTable,    // Safe in middle of table
    Relegation,  // Relegated or in relegation battle
}

/// Season performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonPerformance {
    pub average_match_rating: f32,
    pub goals: u32,
    pub assists: u32,
    pub appearances: u32,
    pub clean_sheets: u32,  // For goalkeepers and defenders
}

/// Statistics for a single season
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonStats {
    pub appearances: u32,
    pub goals: u32,
    pub assists: u32,
    pub average_rating: f32,
    pub clean_sheets: u32,  // For goalkeepers and defenders
}

/// Extension trait for Position to check if it's goalkeeper or defender
trait PositionExt {
    fn is_goalkeeper_or_defender(&self) -> bool;
}

impl PositionExt for crate::entities::Position {
    fn is_goalkeeper_or_defender(&self) -> bool {
        matches!(self, crate::entities::Position::GK | 
                      crate::entities::Position::CB | 
                      crate::entities::Position::LB | 
                      crate::entities::Position::RB)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{Player, Position, Foot, CareerStats, Contract, SquadRole, HiddenAttributes};
    use chrono::NaiveDate;

    #[test]
    fn test_local_reputation_change() {
        let engine = ReputationEngine::new();
        
        // Test high rating in important match
        let change = engine.calculate_local_reputation_change(
            9.0, 
            MatchImportance::Final, 
            true, 
            TeamPerformance::Win
        );
        assert!(change > 5.0);  // Should be substantial
        
        // Test low rating in friendly
        let change = engine.calculate_local_reputation_change(
            5.0, 
            MatchImportance::Friendly, 
            false, 
            TeamPerformance::Loss
        );
        assert!(change < 0.0);  // Should be negative
    }

    #[test]
    fn test_international_conversion() {
        let engine = ReputationEngine::new();
        
        // Test conversion in strong league
        let gain = engine.convert_local_to_international(80.0, 90.0, MatchImportance::Continental);
        assert!(gain > 0.3);  // Should be substantial
        
        // Test conversion in weak league
        let gain = engine.convert_local_to_international(80.0, 30.0, MatchImportance::League);
        assert!(gain < 0.2);  // Should be smaller
    }

    #[test]
    fn test_age_factor() {
        let engine = ReputationEngine::new();
        
        assert_eq!(engine.calculate_age_factor(20), 1.3);  // Young, high factor
        assert_eq!(engine.calculate_age_factor(27), 1.0);  // Prime years
        assert_eq!(engine.calculate_age_factor(34), 0.6);  // Older, lower factor
    }

    #[test]
    fn test_decay_factor() {
        let engine = ReputationEngine::new();
        
        // High reputation should have low decay
        assert_eq!(engine.calculate_decay_factor(80.0), 0.01);
        
        // Low reputation should have higher decay
        assert_eq!(engine.calculate_decay_factor(20.0), 0.05);
    }

    #[test]
    fn test_award_boost() {
        let engine = ReputationEngine::new();
        
        assert_eq!(engine.get_award_reputation_boost("Ballon d'Or"), 25.0);
        assert_eq!(engine.get_award_reputation_boost("Top Scorer"), 10.0);
        assert_eq!(engine.get_award_reputation_boost("Unknown Award"), 2.0);
    }
}