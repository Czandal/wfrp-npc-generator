use super::{generation_base::GenerationBase, trapping::Trapping, talent::Talent, skill::Skill};

pub struct Species {
    name: String,

    // attributes
    weapon_skill: GenerationBase,
    ballistic_skill: GenerationBase,
    strength: GenerationBase,
    toughness: GenerationBase,
    initiative: GenerationBase,
    agility: GenerationBase,
    dexterity: GenerationBase,
    intelligence: GenerationBase,
    willpower: GenerationBase,
    fellowship: GenerationBase,
    wounds: GenerationBase,
    movement: u32,

    trappings: Vec<Trapping>,
    possible_talents: Vec<Talent>,
    possible_skills: Vec<Skill>,
}
