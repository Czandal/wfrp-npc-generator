use super::{talent::Talent, skill::Skill, trapping::Trapping};

#[derive(Debug)]
pub struct Character {
    pub talents: Vec<Talent>,
    pub skills: Vec<Skill>,
    pub trappings: Vec<Trapping>,

    // attributes
    pub weapon_skill: u32,
    pub ballistic_skill: u32,
    pub strength: u32,
    pub toughness: u32,
    pub initiative: u32,
    pub agility: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub willpower: u32,
    pub fellowship: u32,
    pub wounds: u32,
    pub movement: u32,
}
