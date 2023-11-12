use super::{trapping::Trapping, skill_definition::SkillDefinition, talent_definition::TalentDefinition};

pub struct ProfessionClass {
    pub name: String,
    pub trappings: Vec<Trapping>,
    pub skills: Vec<SkillDefinition>,
    pub talents: Vec<TalentDefinition>,
}
