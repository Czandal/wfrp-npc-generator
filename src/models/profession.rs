use super::{trapping::Trapping, skill_definition::SkillDefinition, talent_definition::TalentDefinition, profession_class::ProfessionClass};

// TODO: Fill it with data
pub struct Profession {
    pub trappings: Vec<Trapping>,
    pub skills: Vec<SkillDefinition>,
    pub talents: Vec<TalentDefinition>,

    pub profession_level: u8,
    pub name: String,
    pub class: ProfessionClass,
}
