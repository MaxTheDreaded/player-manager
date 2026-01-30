// src/save/save_manager.rs
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;



use crate::core::game_state::GameState;

/// The SaveManager handles saving and loading game states
/// It supports multiple save slots and version migration
pub struct SaveManager;

impl SaveManager {
    /// Creates a new SaveManager instance
    pub fn new() -> Self {
        SaveManager
    }

    /// Saves the current game state to a file
    pub fn save_game(&self, game_state: &GameState, path: &Path) -> Result<(), SaveError> {
        // Create the directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Serialize the game state to JSON
        let json = serde_json::to_string_pretty(game_state)
            .map_err(|e| SaveError::SerializationError(e.to_string()))?;

        // Write to file
        fs::write(path, json)?;

        Ok(())
    }

    /// Loads a game state from a file
    pub fn load_game(&self, path: &Path) -> Result<GameState, SaveError> {
        // Check if file exists
        if !path.exists() {
            return Err(SaveError::FileNotFound(path.to_string_lossy().to_string()));
        }
        
        // Read the file
        let json = fs::read_to_string(path)?;
        
        // Deserialize the game state
        let mut game_state: GameState = serde_json::from_str(&json)
            .map_err(|e| SaveError::DeserializationError(e.to_string()))?;
        
        // Perform version migration if needed
        game_state = self.migrate_save_format(game_state)?;
        
        Ok(game_state)
    }

    /// Checks if a save file exists
    pub fn save_exists(&self, path: &Path) -> bool {
        path.exists()
    }

    /// Lists all available save files in a directory
    pub fn list_save_files(&self, directory: &Path) -> Result<Vec<String>, SaveError> {
        let mut saves = Vec::new();

        if directory.exists() {
            for entry in fs::read_dir(directory)? {
                let entry = entry?;
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Some(filename) = path.file_name() {
                        saves.push(filename.to_string_lossy().to_string());
                    }
                }
            }
        }

        Ok(saves)
    }

    /// Deletes a save file
    pub fn delete_save(&self, path: &Path) -> Result<(), SaveError> {
        if path.exists() {
            fs::remove_file(path)?;
        }

        Ok(())
    }

    /// Performs version migration on loaded save data
    fn migrate_save_format(&self, mut game_state: GameState) -> Result<GameState, SaveError> {
        // Parse the version string to determine what migrations are needed
        let version_parts: Vec<u32> = game_state.save_version
            .split('.')
            .filter_map(|part| part.parse().ok())
            .collect();
        
        if version_parts.len() < 2 {
            return Err(SaveError::InvalidVersion(game_state.save_version));
        }
        
        let major = version_parts[0];
        let minor = version_parts[1];
        
        // Example migration: if version is older than 1.1, add new fields
        if major == 1 && minor < 1 {
            // Migration for version 1.1
            // Add any new fields that were introduced in 1.1
            // For example, if we added a new field to Player:
            // game_state.player.new_field = Some(default_value);
        }
        
        // Example migration: if version is older than 1.2, update data structure
        if major == 1 && minor < 2 {
            // Migration for version 1.2
            // Update any data structures that changed in 1.2
        }
        
        // Update the version to current
        game_state.save_version = "1.0".to_string();
        
        Ok(game_state)
    }

    /// Validates a save file integrity
    pub fn validate_save(&self, path: &Path) -> Result<bool, SaveError> {
        // Attempt to load the save to check if it's valid
        match self.load_game(path) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Creates a backup of a save file
    pub fn backup_save(&self, original_path: &Path) -> Result<(), SaveError> {
        if !original_path.exists() {
            return Err(SaveError::FileNotFound(original_path.to_string_lossy().to_string()));
        }

        let backup_path = original_path.with_extension(format!(
            "{}.backup",
            original_path.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("json")
        ));

        fs::copy(original_path, &backup_path)?;

        Ok(())
    }
}

/// Error types for save/load operations
#[derive(Debug, thiserror::Error)]
pub enum SaveError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid save version: {0}")]
    InvalidVersion(String),

    #[error("Save validation failed")]
    ValidationFailed,
}

/// Save slot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveSlot {
    pub slot_id: u32,
    pub save_name: String,
    pub save_date: chrono::DateTime<chrono::Utc>,
    pub player_name: String,
    pub player_age: u8,
    pub player_position: String,
    pub game_time: chrono::DateTime<chrono::Utc>,
    pub file_path: String,
}

impl SaveSlot {
    pub fn new(slot_id: u32, game_state: &GameState, file_path: String) -> Self {
        SaveSlot {
            slot_id,
            save_name: format!("Save Slot {}", slot_id),
            save_date: chrono::Utc::now(),
            player_name: game_state.player.name.clone(),
            player_age: game_state.player.age,
            player_position: format!("{:?}", game_state.player.primary_position),
            game_time: game_state.current_date,
            file_path,
        }
    }
}

/// Save manager with additional utility functions
impl SaveManager {
    /// Creates a save slot from a game state
    pub fn create_save_slot(&self, slot_id: u32, game_state: &GameState, file_path: String) -> SaveSlot {
        SaveSlot::new(slot_id, game_state, file_path)
    }

    /// Auto-saves the game at key moments
    pub fn auto_save(&self, game_state: &GameState, base_path: &Path) -> Result<(), SaveError> {
        let auto_save_path = base_path.join("autosave.json");
        self.save_game(game_state, &auto_save_path)
    }

    /// Quick saves the game
    pub fn quick_save(&self, game_state: &GameState, base_path: &Path) -> Result<(), SaveError> {
        let quick_save_path = base_path.join("quicksave.json");
        self.save_game(game_state, &quick_save_path)
    }

    /// Quick loads the game
    pub fn quick_load(&self, base_path: &Path) -> Result<GameState, SaveError> {
        let quick_save_path = base_path.join("quicksave.json");
        self.load_game(&quick_save_path)
    }

    /// Gets save metadata without loading the entire file
    pub fn get_save_metadata(&self, path: &Path) -> Result<SaveSlot, SaveError> {
        // For this implementation, we'll load the whole file to get metadata
        // In a production system, we might store metadata separately
        let game_state = self.load_game(path)?;
        
        Ok(SaveSlot {
            slot_id: 0, // Not stored in the file, would need to be passed separately
            save_name: format!("Save at {}", path.display()),
            save_date: chrono::Utc::now(), // Would come from file modification time in practice
            player_name: game_state.player.name,
            player_age: game_state.player.age,
            player_position: format!("{:?}", game_state.player.primary_position),
            game_time: game_state.current_date,
            file_path: path.to_string_lossy().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{Position, Foot, CareerStats, SquadRole, Contract, HiddenAttributes};
    use chrono::NaiveDate;
    use std::collections::HashMap;

    #[test]
    fn test_save_load_cycle() {
        let save_manager = SaveManager::new();
        
        // Create a test game state
        let player = create_test_player();
        let game_state = GameState::new(player, Uuid::new_v4());
        
        // Create a temporary file path for testing
        let temp_path = std::env::temp_dir().join("test_save.json");
        
        // Save the game
        assert!(save_manager.save_game(&game_state, &temp_path).is_ok());
        
        // Load the game
        let loaded_game_state = save_manager.load_game(&temp_path).unwrap();
        
        // Verify the loaded data matches
        assert_eq!(loaded_game_state.player.name, game_state.player.name);
        assert_eq!(loaded_game_state.player.age, game_state.player.age);
        
        // Clean up
        let _ = fs::remove_file(&temp_path);
    }

    #[test]
    fn test_save_exists() {
        let save_manager = SaveManager::new();
        
        let temp_path = std::env::temp_dir().join("nonexistent_save.json");
        
        assert!(!save_manager.save_exists(&temp_path));
    }

    #[test]
    fn test_validate_save() {
        let save_manager = SaveManager::new();
        
        // Test with non-existent file
        let temp_path = std::env::temp_dir().join("invalid_save.json");
        assert!(!save_manager.validate_save(&temp_path).unwrap_or(false));
        
        // Test with valid save
        let player = create_test_player();
        let game_state = GameState::new(player, Uuid::new_v4());
        let valid_path = std::env::temp_dir().join("valid_save.json");
        
        assert!(save_manager.save_game(&game_state, &valid_path).is_ok());
        assert!(save_manager.validate_save(&valid_path).unwrap_or(false));
        
        // Clean up
        let _ = fs::remove_file(&valid_path);
    }

    #[test]
    fn test_list_save_files() {
        let save_manager = SaveManager::new();
        
        // Create a temporary directory for testing
        let temp_dir = std::env::temp_dir().join("save_test_dir");
        fs::create_dir_all(&temp_dir).unwrap();
        
        // Create a test save file
        let player = create_test_player();
        let game_state = GameState::new(player, Uuid::new_v4());
        let save_path = temp_dir.join("test_save.json");
        save_manager.save_game(&game_state, &save_path).unwrap();
        
        // List save files
        let saves = save_manager.list_save_files(&temp_dir).unwrap();
        assert!(saves.contains(&"test_save.json".to_string()));
        
        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }

    // Helper function to create a test player
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
            contract: Contract {
                club_id: Uuid::new_v4(),
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
        }
    }
}