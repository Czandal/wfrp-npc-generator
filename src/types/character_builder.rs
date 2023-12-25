use crate::{
    interfaces::random_provider::RandomProvider,
    models::{
        base_attribute::BaseAttribute,
        generation_base::GenerationBase,
        max_talent_level::MaxTalentLevel,
        species::Species,
        talent::Talent,
        talent_definition::{SingularTalentDefinition, TalentDefinition},
        trapping::Trapping,
    },
};

#[derive(Debug, Clone, Copy)]
struct CharacterAttribute {
    id: BaseAttribute,
    base_value: u32,
    advances: u32,
}

impl CharacterAttribute {
    pub fn advance(&mut self, advancement_floor: u32) -> () {
        self.advances = if advancement_floor > self.advances {
            advancement_floor
        } else {
            self.advances
        };
    }
    pub fn increase_base(&mut self, value: u32) -> () {
        self.base_value += value;
    }
    pub fn is_attribute(&self, attr: BaseAttribute) -> bool {
        attr == self.id
    }
    pub fn value(&self) -> u32 {
        self.base_value + self.advances
    }
    pub fn new(id: BaseAttribute, base: u32) -> CharacterAttribute {
        CharacterAttribute {
            id,
            base_value: base,
            advances: 0,
        }
    }
    pub fn bonus(&self) -> u32 {
        self.value() / 10
    }
}

#[derive(Debug)]
struct SkillWrapper<'a> {
    pub attribute: &'a CharacterAttribute,
    pub name: &'a str,
    pub advances: u32,
}

impl SkillWrapper<'_> {
    pub fn advance(&mut self, advancement_floor: u32) -> () {
        self.advances = if advancement_floor > self.advances {
            advancement_floor
        } else {
            self.advances
        };
    }
    pub fn value(&self) -> u32 {
        self.attribute.value() + self.advances
    }
    pub fn new(id: BaseAttribute, base: u32) -> CharacterAttribute {
        CharacterAttribute {
            id,
            base_value: base,
            advances: 0,
        }
    }
    pub fn bonus(&self) -> u32 {
        self.value() / 10
    }
}

#[derive(Debug)]
struct FixedMaxLevelTalentWrapper<'a> {
    pub level: u32,
    pub max_level: u32,
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Debug)]
struct AttributeDependentMaxLevelTalentWrapper<'a> {
    pub level: u32,
    pub name: &'a str,
    pub description: &'a str,
    pub attribute_determining_level: &'a CharacterAttribute,
}

#[derive(Debug)]
struct SkillDependentMaxLevelTalentWrapper<'a> {
    pub level: u32,
    pub name: &'a str,
    pub description: &'a str,
    pub skill_determining_level: &'a SkillWrapper<'a>,
}

#[derive(Debug)]
enum TalentWrapper<'a> {
    Fixed(FixedMaxLevelTalentWrapper<'a>),
    AttributeDependent(AttributeDependentMaxLevelTalentWrapper<'a>),
    SkillDependent(SkillDependentMaxLevelTalentWrapper<'a>),
}

impl<'a> TalentWrapper<'a> {
    pub fn get_name(&self) -> &'a str {
        match self {
            TalentWrapper::Fixed(f) => f.name,
            TalentWrapper::AttributeDependent(a) => a.name,
            TalentWrapper::SkillDependent(s) => s.name,
        }
    }
    pub fn merge(&mut self, other: &TalentWrapper) -> () {
        macro_rules! mismatched_talent_wrapper {
            ($expected: literal, $actual: literal) => {
                panic!("TalentWrapper underlying type mismatch while trying to merge - self is {:?} and other is {:?}", $expected, $actual)
            };
        }
        match self {
            TalentWrapper::Fixed(f) => match other {
                TalentWrapper::Fixed(f_other) => {
                    f.level = if f.level + f_other.level > f.max_level {
                        f.max_level
                    } else {
                        f.level + f_other.level
                    };
                }
                TalentWrapper::AttributeDependent(_) => {
                    mismatched_talent_wrapper!("Fixed", "AttributeDependent")
                }
                TalentWrapper::SkillDependent(_) => {
                    mismatched_talent_wrapper!("Fixed", "SkillDependent")
                }
            },
            TalentWrapper::AttributeDependent(a) => match other {
                TalentWrapper::Fixed(_) => {
                    mismatched_talent_wrapper!("AttributeDependent", "Fixed")
                }
                TalentWrapper::AttributeDependent(a_other) => {
                    a.level = if a.attribute_determining_level.bonus() < a.level + a_other.level {
                        a.attribute_determining_level.bonus()
                    } else {
                        a.level + a_other.level
                    };
                }
                TalentWrapper::SkillDependent(_) => {
                    mismatched_talent_wrapper!("AttributeDependent", "SkillDependent")
                }
            },
            TalentWrapper::SkillDependent(s) => match other {
                TalentWrapper::Fixed(_) => mismatched_talent_wrapper!("SkillDependent", "Fixed"),
                TalentWrapper::AttributeDependent(_) => {
                    mismatched_talent_wrapper!("SkillDependent", "AttributeDependent")
                }
                TalentWrapper::SkillDependent(s_other) => {
                    s.level = if s.skill_determining_level.bonus() < s.level + s_other.level {
                        s.skill_determining_level.bonus()
                    } else {
                        s.level + s_other.level
                    };
                }
            },
        }
    }
}

pub struct CharacterBuilder<'v> {
    random_provider: &'v dyn RandomProvider,

    weapon_skill: CharacterAttribute,
    ballistic_skill: CharacterAttribute,
    strength: CharacterAttribute,
    toughness: CharacterAttribute,
    initiative: CharacterAttribute,
    agility: CharacterAttribute,
    dexterity: CharacterAttribute,
    intelligence: CharacterAttribute,
    willpower: CharacterAttribute,
    fellowship: CharacterAttribute,
    movement: u32,

    talents: Vec<TalentWrapper<'v>>,
    skills: Vec<SkillWrapper<'v>>,
    trappings: Vec<Trapping<'v>>,
}

impl<'v> CharacterBuilder<'v> {
    pub fn new(species: &'v Species<'v>, random_provider: &'v dyn RandomProvider) -> Self {
        let mut char_build = CharacterBuilder {
            weapon_skill: CharacterAttribute::new(
                BaseAttribute::WeaponSkill,
                random_provider.generate(&species.weapon_skill),
            ),
            ballistic_skill: CharacterAttribute::new(
                BaseAttribute::WeaponSkill,
                random_provider.generate(&species.ballistic_skill),
            ),
            strength: CharacterAttribute::new(
                BaseAttribute::Strength,
                random_provider.generate(&species.strength),
            ),
            toughness: CharacterAttribute::new(
                BaseAttribute::Strength,
                random_provider.generate(&species.toughness),
            ),
            initiative: CharacterAttribute::new(
                BaseAttribute::Strength,
                random_provider.generate(&species.initiative),
            ),
            agility: CharacterAttribute::new(
                BaseAttribute::Strength,
                random_provider.generate(&species.agility),
            ),
            dexterity: CharacterAttribute::new(
                BaseAttribute::Strength,
                random_provider.generate(&species.dexterity),
            ),
            intelligence: CharacterAttribute::new(
                BaseAttribute::Strength,
                random_provider.generate(&species.intelligence),
            ),
            willpower: CharacterAttribute::new(
                BaseAttribute::Strength,
                random_provider.generate(&species.willpower),
            ),
            fellowship: CharacterAttribute::new(
                BaseAttribute::Strength,
                random_provider.generate(&species.fellowship),
            ),
            movement: species.movement,
            random_provider,
            trappings: vec![],
            talents: vec![],
            skills: vec![],
        };
        let talent = char_build.talent_from_definition(species.possible_talents.first().unwrap());
        let already_existing_talent = char_build
            .talents
            .iter_mut()
            .find(|tal| tal.get_name() == talent.get_name());
        match already_existing_talent {
            Some(existing_tal) => existing_tal.merge(&talent),
            None => char_build.talents.push(talent),
        }
        return char_build;
    }

    fn talent_from_definition(&self, talent_definition: &TalentDefinition) -> TalentWrapper {
        match talent_definition {
            TalentDefinition::Singular(talent_def) => match talent_def.max_level {
                MaxTalentLevel::Fixed(fixed) => TalentWrapper::Fixed(FixedMaxLevelTalentWrapper {
                    name: talent_def.name,
                    level: 1,
                    max_level: fixed.fixed_max_level,
                    description: talent_def.description,
                }),
                MaxTalentLevel::AttributeDependent(attr_dep) => {
                    TalentWrapper::AttributeDependent(AttributeDependentMaxLevelTalentWrapper {
                        level: 1,
                        name: talent_def.name,
                        description: talent_def.description,
                        attribute_determining_level: match attr_dep.attribute_determining_level {
                            BaseAttribute::WeaponSkill => &self.weapon_skill,
                            BaseAttribute::BallisticSkill => &self.ballistic_skill,
                            BaseAttribute::Strength => &self.strength,
                            BaseAttribute::Toughness => &self.toughness,
                            BaseAttribute::Initiative => &self.initiative,
                            BaseAttribute::Agility => &self.agility,
                            BaseAttribute::Dexterity => &self.dexterity,
                            BaseAttribute::Intelligence => &self.intelligence,
                            BaseAttribute::Willpower => &self.willpower,
                            BaseAttribute::Fellowship => &self.fellowship,
                            BaseAttribute::Wounds => panic!(
                                "Tried to retrieve reference to CharacterAttribute to Wounds, which is literal"
                            ),
                            BaseAttribute::Movement => panic!(
                                "Tried to retrieve reference to CharacterAttribute to Movement, which is literal"
                            ),
                        },
                    })
                }
                MaxTalentLevel::SkillDependent(skill_dep) => {
                    TalentWrapper::SkillDependent(SkillDependentMaxLevelTalentWrapper {
                        level: 1,
                        name: talent_def.name,
                        description: talent_def.description,
                        skill_determining_level: todo!(),
                    })
                }
            },
            TalentDefinition::Random(random) => {
                let possible_talents_count = random.possible_talents.len();
                let random_talent_index = (self.random_provider.generate(&GenerationBase {
                    dices: vec![possible_talents_count as u32],
                    base_value: 0,
                }) - 1) as usize;
                let talent_def = random.possible_talents[random_talent_index];
                match talent_def.max_level {
                    MaxTalentLevel::Fixed(fixed) => {
                        TalentWrapper::Fixed(FixedMaxLevelTalentWrapper {
                            name: talent_def.name,
                            level: 1,
                            max_level: fixed.fixed_max_level,
                            description: talent_def.description,
                        })
                    }
                    MaxTalentLevel::AttributeDependent(attr_dep) => {
                        TalentWrapper::AttributeDependent(AttributeDependentMaxLevelTalentWrapper {
                            level: 1,
                            name: talent_def.name,
                            description: talent_def.description,
                            attribute_determining_level: match attr_dep.attribute_determining_level
                            {
                                BaseAttribute::WeaponSkill => &self.weapon_skill,
                                BaseAttribute::BallisticSkill => &self.ballistic_skill,
                                BaseAttribute::Strength => &self.strength,
                                BaseAttribute::Toughness => &self.toughness,
                                BaseAttribute::Initiative => &self.initiative,
                                BaseAttribute::Agility => &self.agility,
                                BaseAttribute::Dexterity => &self.dexterity,
                                BaseAttribute::Intelligence => &self.intelligence,
                                BaseAttribute::Willpower => &self.willpower,
                                BaseAttribute::Fellowship => &self.fellowship,
                                BaseAttribute::Wounds => panic!(
                                    "Tried to retrieve reference to CharacterAttribute to Wounds, which is literal"
                                ),
                                BaseAttribute::Movement => panic!(
                                    "Tried to retrieve reference to CharacterAttribute to Movement, which is literal"
                                ),
                            },
                        })
                    }
                    MaxTalentLevel::SkillDependent(skill_dep) => {
                        TalentWrapper::SkillDependent(SkillDependentMaxLevelTalentWrapper {
                            level: 1,
                            name: talent_def.name,
                            description: talent_def.description,
                            skill_determining_level: todo!(),
                        })
                    }
                }
            }
        }
    }
}
