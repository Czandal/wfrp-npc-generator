use super::{trapping::Trapping, skill_definition::SkillDefinition, talent_definition::TalentDefinition, profession_class::ProfessionClass, base_attribute::BaseAttribute};

pub struct Profession<'a> {
    pub trappings: Vec<Trapping<'a>>,
    pub skills: Vec<SkillDefinition<'a>>,
    pub talents: Vec<TalentDefinition<'a>>,

    pub profession_level: u8,
    pub attributes: Vec<BaseAttribute>,
    pub name: &'a str,
    pub class: ProfessionClass<'a>,
}
