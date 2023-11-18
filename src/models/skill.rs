use super::base_attribute::BaseAttribute;
#[derive(Debug)]
pub struct Skill {
    pub name: String,
    pub value: u32,
    pub base_attribute: BaseAttribute,
}
