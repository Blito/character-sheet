use crate::character::stats;

pub struct Skill {
    pub has_proficiency: bool,
    pub stat: stats::Stat,
    pub name: String,
    pub bonus: i16
}