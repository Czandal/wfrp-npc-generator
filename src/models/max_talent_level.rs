use super::base_attribute::BaseAttribute;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FixedMaxTalentLevel {
    pub fixed_max_level: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttributeDependentMaxTalentLevel {
    pub attribute_determining_level: BaseAttribute,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SkillDependentMaxTalentLevel<'a> {
    pub skill_determining_level: &'a str,
}

// it can be fixed or it can be attribute dependent (or even skill dependent)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaxTalentLevel<'a> {
    Fixed(FixedMaxTalentLevel),
    AttributeDependent(AttributeDependentMaxTalentLevel),
    SkillDependent(SkillDependentMaxTalentLevel<'a>),
}
