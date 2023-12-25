use super::{generation_base::GenerationBase, trapping::Trapping, skill::Skill, talent_definition::TalentDefinition};

pub struct Species<'a> {
    pub name: &'a str,

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
    pub movement: u32,

    pub trappings: Vec<Trapping<'a>>,
    pub possible_talents: Vec<TalentDefinition<'a>>,
    pub possible_skills: Vec<Skill<'a>>,
}
