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

    stats: HashMap<stats::Stat, (u16, i16)>,

    armor_class: u16,
    initiative: u16,
    proficiency_bonus: u16,
    walking_speed_in_ft: u16,

    skills: [skill::Skill; 17]
}

impl Character {
    pub fn new(
        name: String,
        race: String,
        class: String,
        level: u16,
        current_hitpoints: i16,
        max_hitpoints: u16,

        stats: HashMap<stats::Stat, (u16, i16)>,

        armor_class: u16,
        initiative: u16,
        proficiency_bonus: u16,
        walking_speed_in_ft: u16,

        //skills: [skill::Skill; 17]
    ) -> Character {
        use stats::Stat::*;

        Character {
            name,
            race,
            class,
            level,

            current_hitpoints,
            max_hitpoints,

            stats: HashMap::new(),
            armor_class,
            initiative,
            proficiency_bonus,
            walking_speed_in_ft,
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

    pub fn get_race(&self) -> &str { &self.race }

    pub fn get_class(&self) -> &str {
        &self.class
    }

    pub fn get_level(&self) -> &u16 { &self.level }

    pub fn get_current_hitpoints(&self) -> &i16 { &self.current_hitpoints }

    pub fn get_max_hitpoints(&self) -> &u16 { &self.max_hitpoints }

    pub fn get_armor_class(&self) -> &u16 { &self.armor_class }
}