use super::max_talent_level::MaxTalentLevel;

#[derive(PartialEq)]
pub struct Talent<'a> {
    pub name: &'a str,
    pub level: u32,
    pub max_level: MaxTalentLevel<'a>,
    pub description: &'a str,
}
