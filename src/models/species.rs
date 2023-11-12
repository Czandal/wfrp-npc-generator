use super::{generation_base::GenerationBase, trapping::Trapping, talent::Talent, skill::Skill};

pub struct Species {
    pub name: String,

    // attributes
    pub weapon_skill: GenerationBase,
    pub ballistic_skill: GenerationBase,
    pub strength: GenerationBase,
    pub toughness: GenerationBase,
    pub initiative: GenerationBase,
    pub agility: GenerationBase,
    pub dexterity: GenerationBase,
    pub intelligence: GenerationBase,
    pub willpower: GenerationBase,
    pub fellowship: GenerationBase,
    pub wounds: GenerationBase,
    pub movement: u32,

    pub trappings: Vec<Trapping>,
    pub possible_talents: Vec<Talent>,
    pub possible_skills: Vec<Skill>,
}
