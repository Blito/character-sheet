use crate::character::stats;
use crate::character::skill;
use std::collections::HashMap;

pub struct Character {
    name: String,
    race: String,
    class: String,
    level: u16,

    current_hitpoints: i16,
    max_hitpoints: u16,

    //stats: HashMap<stats::Stat, (u16, i16)>,

    armor_class: u16,
    initiative: u16,
    proficiency_bonus: u16,
    walking_speed_in_ft: u16,

    skills: [skill::Skill; 17]
}

impl Character {
    pub fn new(name: String) -> Character {
        use stats::Stat::*;

        Character {
            name,
            race: String::from(""),
            class: String::from(""),
            level: 1,

            current_hitpoints: 1,
            max_hitpoints: 1,

            //stats: HashMap::new(),
            armor_class: 0,
            initiative: 0,
            proficiency_bonus: 0,
            walking_speed_in_ft: 30,
            skills: [
                skill::Skill { has_proficiency: false, stat: Dexterity, name: String::from("Acrobatics"), bonus: 3 },
                skill::Skill { has_proficiency: false, stat: Wisdom, name: String::from("Animal Handling"), bonus: 1 },
                skill::Skill { has_proficiency: true, stat: Intellect, name: String::from("Arcana"), bonus: 6 },
                skill::Skill { has_proficiency: false, stat: Strength, name: String::from("Athletics"), bonus: 0 },
                skill::Skill { has_proficiency: false, stat: Charisma, name: String::from("Deception"), bonus: 0 },
                skill::Skill { has_proficiency: true, stat: Intellect, name: String::from("History"), bonus: 6 },
                skill::Skill { has_proficiency: false, stat: Wisdom, name: String::from("Insight"), bonus: 1 },
                skill::Skill { has_proficiency: false, stat: Charisma, name: String::from("Intimidation"), bonus: 0 },
                skill::Skill { has_proficiency: true, stat: Intellect, name: String::from("Investigation"), bonus: 6 },
                skill::Skill { has_proficiency: false, stat: Wisdom, name: String::from("Medicine"), bonus: 1 },
                skill::Skill { has_proficiency: false, stat: Intellect, name: String::from("Nature"), bonus: 4 },
                skill::Skill { has_proficiency: false, stat: Wisdom, name: String::from("Perception"), bonus: 1 },
                skill::Skill { has_proficiency: false, stat: Charisma, name: String::from("Performance"), bonus: 0 },
                skill::Skill { has_proficiency: false, stat: Intellect, name: String::from("Religion"), bonus: 4 },
                skill::Skill { has_proficiency: false, stat: Dexterity, name: String::from("Sleight of Hand"), bonus: 3 },
                skill::Skill { has_proficiency: true, stat: Dexterity, name: String::from("Stealth"), bonus: 5 },
                skill::Skill { has_proficiency: false, stat: Wisdom, name: String::from("Survival"), bonus: 1 }
            ]
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}