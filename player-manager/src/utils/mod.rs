// Placeholder utils module to satisfy imports
// This module can be expanded with utility functions as needed

pub mod constants {
    // Common constants used throughout the application
    pub const MAX_PLAYERS_PER_SQUAD: usize = 25;
    pub const SEASON_LENGTH_WEEKS: u32 = 38;
    pub const MATCH_DURATION_MINUTES: u8 = 90;
}

pub mod helpers {
    use uuid::Uuid;
    
    // Helper functions for common operations
    pub fn generate_unique_id() -> Uuid {
        Uuid::new_v4()
    }
    
    pub fn calculate_age(birth_date: chrono::NaiveDate) -> u8 {
        let today = chrono::Utc::now().date_naive();
        let duration = today.signed_duration_since(birth_date);
        (duration.num_days() / 365) as u8
    }
}