use std::collections::HashMap;

/// Content for a specific tutorial guide
pub struct TutorialGuide {
    pub title: String,
    pub content: String,
}

/// Manages the tutorial system
pub struct TutorialManager {
    guides: HashMap<String, TutorialGuide>,
}

impl TutorialManager {
    /// Creates a new TutorialManager and registers default guides
    pub fn new() -> Self {
        let mut manager = TutorialManager {
            guides: HashMap::new(),
        };
        
        manager.register_default_guides();
        manager
    }
    
    /// Registers a new tutorial guide
    pub fn register_guide(&mut self, key: &str, title: &str, content: &str) {
        self.guides.insert(
            key.to_string(),
            TutorialGuide {
                title: title.to_string(),
                content: content.to_string(),
            },
        );
    }
    
    /// Gets a tutorial guide by key
    pub fn get_guide(&self, key: &str) -> Option<&TutorialGuide> {
        self.guides.get(key)
    }
    
    /// Registers the default guides for the game
    fn register_default_guides(&mut self) {
        // Main Menu
        self.register_guide(
            "main_menu",
            "Main Menu Guide",
            "The Main Menu is your central hub.\n\
             - View Profile: Check your attributes and status.\n\
             - Team Info: See details about your current club.\n\
             - Continue: Advance time to the next important event.\n\
             \n\
             Tip: You can type 'help' or 'h' at any menu to see this guide again."
        );
        
        // Player Profile
        self.register_guide(
            "player_profile",
            "Player Profile Guide",
            "This screen shows your current attributes and status.\n\
             - Attributes are split into Technical, Physical, and Mental.\n\
             - Attributes grow through training and match experience.\n\
             - Keep an eye on your Contract expiry date!"
        );
        
        // Team Info
        self.register_guide(
            "team_info",
            "Team Information Guide",
            "Here you can see details about your club.\n\
             - Reputation affects the quality of players attracted to the club.\n\
             - Facilities affect your training effectiveness and development."
        );
        
        // Match Report
        self.register_guide(
            "match_report",
            "Match Report Guide",
            "This summary appears after every match.\n\
             - Rating: Your performance score (1-10).\n\
             - Key Events: Highlights of your involvement.\n\
             - Consistent high ratings lead to faster development and better contract offers."
        );
        
        // Training Selection
        self.register_guide(
            "training_selection",
            "Training Selection Guide",
            "Weekly training is crucial for development.\n\
             - Technical: Improves ball skills (Dribbling, Passing, etc.).\n\
             - Physical: Improves athleticism (Pace, Strength, etc.).\n\
             - Tactical: Improves mental attributes (Positioning, Vision).\n\
             - Rest: Recovers fatigue but pauses development."
        );
    }
}
