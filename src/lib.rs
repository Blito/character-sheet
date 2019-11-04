use std::error::Error;

use crate::character::Character;
use crate::ui::MainApp;

mod ui;
mod character;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub struct Config {
    pub character_filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let character_filename = String::from("Dandelion.json");

        Config { character_filename }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let dandelion_json = r#"
    {
        "name": "Dandelion",
        "race": "Rock Gnome",
        "class": "Wizard",
        "level": 3,

        "current_hitpoints": 14,
        "max_hitpoints": 17,

        "stats": {
            "Strength": 11,
            "Dexterity": 13
        },

        "armor_class": 14,
        "initiative": 3,
        "proficiency_bonus": 0,
        "walking_speed_in_ft": 25,

        "skills": [
            { "has_proficiency": false, "stat": "Dexterity", "name": "Acrobatics", "bonus": 3 },
            { "has_proficiency": false, "stat": "Wisdom", "name": "Animal Handling", "bonus": 1 },
            { "has_proficiency": true, "stat": "Intellect", "name": "Arcana", "bonus": 6 },
            { "has_proficiency": false, "stat": "Strength", "name": "Athletics", "bonus": 0 },
            { "has_proficiency": false, "stat": "Charisma", "name": "Deception", "bonus": 0 },
            { "has_proficiency": true, "stat": "Intellect", "name": "History", "bonus": 6 },
            { "has_proficiency": false, "stat": "Wisdom", "name": "Insight", "bonus": 1 },
            { "has_proficiency": false, "stat": "Charisma", "name": "Intimidation", "bonus": 0 },
            { "has_proficiency": true, "stat": "Intellect", "name": "Investigation", "bonus": 6 },
            { "has_proficiency": false, "stat": "Wisdom", "name": "Medicine", "bonus": 1 },
            { "has_proficiency": false, "stat": "Intellect", "name": "Nature", "bonus": 4 },
            { "has_proficiency": false, "stat": "Wisdom", "name": "Perception", "bonus": 1 },
            { "has_proficiency": false, "stat": "Charisma", "name": "Performance", "bonus": 0 },
            { "has_proficiency": false, "stat": "Intellect", "name": "Religion", "bonus": 4 },
            { "has_proficiency": false, "stat": "Dexterity", "name": "Sleight of Hand", "bonus": 3 },
            { "has_proficiency": true, "stat": "Dexterity", "name": "Stealth", "bonus": 5 },
            { "has_proficiency": false, "stat": "Wisdom", "name": "Survival", "bonus": 1 }
        ]
    }
    "#;

    let dandelion: Character = serde_json::from_str(dandelion_json)?;

    let main_app = MainApp::new ( &dandelion )?;

    main_app.draw_app()?;

    Ok(())
}