#[derive(Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Stat {
    Strength,
    Dexterity,
    Constitution,
    Intellect,
    Wisdom,
    Charisma
}