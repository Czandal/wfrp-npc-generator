use super::max_talent_level::MaxTalentLevel;

pub struct SingularTalentDefinition<'a> {
    pub name: &'a str,
    pub max_level: MaxTalentLevel<'a>,
    pub description: &'a str,
}

pub struct RandomTalentDefinition<'a> {
    pub possible_talents: Vec<&'a SingularTalentDefinition<'a>>
}


pub enum TalentDefinition<'a> {
    Singular(SingularTalentDefinition<'a>),
    Random(RandomTalentDefinition<'a>),
}

