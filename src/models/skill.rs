use super::base_attribute::BaseAttribute;

pub struct Skill<'a> {
    pub name: &'a str,
    pub value: u32,
    pub base_attribute: BaseAttribute,
}
