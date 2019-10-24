use crate::character::stats;
use crate::character::skill;

use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Character {
    name: String,
    race: String,
    class: String,
    level: u16,

    current_hitpoints: i16,
    max_hitpoints: u16,

    stats: HashMap<stats::Stat, i16>,

    armor_class: u16,
    initiative: u16,
    proficiency_bonus: u16,
    walking_speed_in_ft: u16,

    skills: [skill::Skill; 17]
}

impl Character {

    pub fn get_name(&self) -> &str { &self.name }

    pub fn get_race(&self) -> &str { &self.race }

    pub fn get_class(&self) -> &str { &self.class }

    pub fn get_level(&self) -> &u16 { &self.level }

    pub fn get_current_hitpoints(&self) -> &i16 { &self.current_hitpoints }

    pub fn get_max_hitpoints(&self) -> &u16 { &self.max_hitpoints }

    pub fn get_armor_class(&self) -> &u16 { &self.armor_class }

    pub fn get_initiative(&self) -> &u16 { &self.initiative }

    pub fn get_proficiency_bonus(&self) -> &u16 { &self.proficiency_bonus }

    pub fn get_walking_speed_in_ft(&self) -> &u16 { &self.walking_speed_in_ft }
}