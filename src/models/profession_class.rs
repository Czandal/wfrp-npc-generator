use super::{trapping::Trapping, skill_definition::SkillDefinition, talent_definition::TalentDefinition};

pub struct ProfessionClass<'a> {
    pub name: &'a str,
    pub trappings: Vec<Trapping<'a>>,
    pub skills: Vec<SkillDefinition<'a>>,
    pub talents: Vec<TalentDefinition<'a>>,
}
