use super::base_attribute::BaseAttribute;

pub struct SkillDefinition<'a> {
    pub name: &'a str,
    pub base_attribute: BaseAttribute,
}
