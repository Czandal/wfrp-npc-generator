use crate::interfaces::character_generator::CharacterGenerator;
use crate::interfaces::random_provider::RandomProvider;
use crate::models::base_attribute::BaseAttribute;
use crate::models::character::Character;
use crate::models::generation_base::GenerationBase;
use crate::models::max_talent_level::{FixedMaxTalentLevel, MaxTalentLevel};
use crate::models::profession::Profession;
use crate::models::skill::Skill;
use crate::models::skill_definition::{self, SkillDefinition};
use crate::models::species::Species;
use crate::models::talent::Talent;
use crate::models::talent_definition::TalentDefinition;
use crate::models::trapping::Trapping;

pub struct NonPlayableCharacterGenerator {
    random_provider: Box<dyn RandomProvider>,
}

impl NonPlayableCharacterGenerator {
    pub fn new(random_provider: Box<dyn RandomProvider>) -> NonPlayableCharacterGenerator {
        NonPlayableCharacterGenerator { random_provider }
    }

    fn talent_from_talent_definition<'a>(
        &self,
        talent_definition: &TalentDefinition<'a>,
    ) -> Talent<'a> {
        match talent_definition {
            TalentDefinition::Singular(talent_def) => Talent {
                name: talent_def.name,
                level: 1,
                max_level: talent_def.max_level,
                description: talent_def.description,
            },
            TalentDefinition::Random(random) => {
                let possible_talents_count = random.possible_talents.len();
                let random_talent_index = (self.random_provider.generate(&GenerationBase {
                    dices: vec![possible_talents_count as u32],
                    base_value: 0,
                }) - 1) as usize;
                let talent_def = random.possible_talents[random_talent_index];
                Talent {
                    name: talent_def.name,
                    level: 1,
                    max_level: talent_def.max_level,
                    description: talent_def.description,
                }
            }
        }
    }

    fn unique_dice_values(&self, dice: u32, number_of_unique_rolls: usize) -> Vec<u32> {
        if number_of_unique_rolls > dice as usize {
            panic!("Unique number of rolls cannot exceed number of dice faces, dice = {}, number_of_unique_rolls = {}", dice, number_of_unique_rolls);
        }
        let mut output = Vec::with_capacity(number_of_unique_rolls);
        let gen_base = GenerationBase {
            dices: vec![dice],
            base_value: 0,
        };
        for _i in 0..number_of_unique_rolls {
            let mut random_value = self.random_provider.generate(&gen_base);
            while output.iter().find(|&&x| x == random_value).is_some() {
                random_value = self.random_provider.generate(&gen_base);
            }
            output.push(random_value);
        }
        output
    }
}

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

struct CharacterPrototype<'a> {
    pub talents: Vec<Talent<'a>>,
    pub skills: Vec<Skill<'a>>,
    pub trappings: Vec<Trapping<'a>>,

    // attributes
    pub weapon_skill: CharacterAttribute,
    pub ballistic_skill: CharacterAttribute,
    pub strength: CharacterAttribute,
    pub toughness: CharacterAttribute,
    pub initiative: CharacterAttribute,
    pub agility: CharacterAttribute,
    pub dexterity: CharacterAttribute,
    pub intelligence: CharacterAttribute,
    pub willpower: CharacterAttribute,
    pub fellowship: CharacterAttribute,
    pub wounds: u32,
    pub movement: u32,
}

impl<'a> CharacterPrototype<'a> {
    // NOTE: It takes mutable reference to self to invalidate any other use of it
    pub fn consume(self: CharacterPrototype<'a>) -> Character<'a> {
        Character {
            talents: self.talents,
            skills: self.skills,
            trappings: self.trappings,
            weapon_skill: self.weapon_skill.value(),
            ballistic_skill: self.ballistic_skill.value(),
            strength: self.strength.value(),
            toughness: self.toughness.value(),
            initiative: self.initiative.value(),
            agility: self.agility.value(),
            dexterity: self.dexterity.value(),
            intelligence: self.intelligence.value(),
            willpower: self.willpower.value(),
            fellowship: self.fellowship.value(),
            // wounds = BS + 2BT + BWp
            wounds: self.strength.bonus() + 2 * self.toughness.bonus() + self.willpower.bonus(),
            movement: self.movement,
        }
    }
}

impl CharacterGenerator for NonPlayableCharacterGenerator {
    fn generate<'a>(
        &self,
        professions: &Vec<Profession<'a>>,
        species: &'a Species,
    ) -> Character<'a> {
        // 1. Create character prototype
        let mut prototype = CharacterPrototype {
            talents: vec![],
            skills: vec![],
            trappings: vec![],
            weapon_skill: CharacterAttribute::new(
                BaseAttribute::WeaponSkill,
                self.random_provider.generate(&species.weapon_skill),
            ),
            ballistic_skill: CharacterAttribute::new(
                BaseAttribute::BallisticSkill,
                self.random_provider.generate(&species.ballistic_skill),
            ),
            strength: CharacterAttribute::new(
                BaseAttribute::Strength,
                self.random_provider.generate(&species.strength),
            ),
            toughness: CharacterAttribute::new(
                BaseAttribute::Toughness,
                self.random_provider.generate(&species.toughness),
            ),
            initiative: CharacterAttribute::new(
                BaseAttribute::Initiative,
                self.random_provider.generate(&species.initiative),
            ),
            agility: CharacterAttribute::new(
                BaseAttribute::Agility,
                self.random_provider.generate(&species.agility),
            ),
            dexterity: CharacterAttribute::new(
                BaseAttribute::Dexterity,
                self.random_provider.generate(&species.dexterity),
            ),
            intelligence: CharacterAttribute::new(
                BaseAttribute::Intelligence,
                self.random_provider.generate(&species.intelligence),
            ),
            willpower: CharacterAttribute::new(
                BaseAttribute::Willpower,
                self.random_provider.generate(&species.willpower),
            ),
            fellowship: CharacterAttribute::new(
                BaseAttribute::Fellowship,
                self.random_provider.generate(&species.fellowship),
            ),
            wounds: 0,
            movement: species.movement,
        };

        let with_applied_talents =
            species
                .possible_talents
                .iter()
                .fold(prototype, |prot, talent_def| {
                    let talent = self.talent_from_talent_definition(&talent_def);
                    if let Some(element) = prot.talents.iter().find(|x| **x == talent) {
                        let max_level = match element.max_level {
                            MaxTalentLevel::Fixed(fixed) => fixed.fixed_max_level,
                            MaxTalentLevel::AttributeDependent(attr_dep) => {
                                match attr_dep.attribute_determining_level {
                                    BaseAttribute::WeaponSkill => prot.weapon_skill.bonus(),
                                    BaseAttribute::BallisticSkill => prot.ballistic_skill.bonus(),
                                    BaseAttribute::Strength => prot.strength.bonus(),
                                    BaseAttribute::Toughness => prot.toughness.bonus(),
                                    BaseAttribute::Initiative => prot.initiative.bonus(),
                                    BaseAttribute::Agility => prot.agility.bonus(),
                                    BaseAttribute::Dexterity => prot.dexterity.bonus(),
                                    BaseAttribute::Intelligence => prot.intelligence.bonus(),
                                    BaseAttribute::Willpower => prot.willpower.bonus(),
                                    BaseAttribute::Fellowship => prot.fellowship.bonus(),
                                    BaseAttribute::Wounds => {
                                        panic!("Cannot have talent dependent on wounds")
                                    }
                                    BaseAttribute::Movement => {
                                        panic!("Cannot have talent dependent on Movement")
                                    }
                                }
                            }
                            MaxTalentLevel::SkillDependent(skill_dep) => 1,
                        };
                        element.level = if element.level + 1 > max_level {
                            max_level
                        } else {
                            element.level + 1
                        };
                    } else {
                        prot.talents.push(talent);
                    }
                    prot
                });

        // 1. Create character fields and init them with species attributes
        let mut talents: Vec<Talent> = vec![];
        let mut skills: Vec<Skill> = vec![];
        let mut trappings: Vec<Trapping> = species.trappings.clone();
        // NOTE: Do not change order of the elements
        let mut attributes = [
            CharacterAttribute::new(
                BaseAttribute::WeaponSkill,
                self.random_provider.generate(&species.weapon_skill),
            ),
            CharacterAttribute::new(
                BaseAttribute::BallisticSkill,
                self.random_provider.generate(&species.ballistic_skill),
            ),
            CharacterAttribute::new(
                BaseAttribute::Strength,
                self.random_provider.generate(&species.strength),
            ),
            CharacterAttribute::new(
                BaseAttribute::Toughness,
                self.random_provider.generate(&species.toughness),
            ),
            CharacterAttribute::new(
                BaseAttribute::Initiative,
                self.random_provider.generate(&species.initiative),
            ),
            CharacterAttribute::new(
                BaseAttribute::Agility,
                self.random_provider.generate(&species.agility),
            ),
            CharacterAttribute::new(
                BaseAttribute::Dexterity,
                self.random_provider.generate(&species.dexterity),
            ),
            CharacterAttribute::new(
                BaseAttribute::Intelligence,
                self.random_provider.generate(&species.intelligence),
            ),
            CharacterAttribute::new(
                BaseAttribute::Willpower,
                self.random_provider.generate(&species.willpower),
            ),
            CharacterAttribute::new(
                BaseAttribute::Fellowship,
                self.random_provider.generate(&species.fellowship),
            ),
        ];
        // 2. Apply rest of species config
        // 2.1 Apply talents from species
        for talent_definition in species.possible_talents.iter() {
            let talent = self.talent_from_talent_definition(&talent_definition);
            if let Some(element) = talents.iter_mut().find(|x| **x == talent) {
                let max_level = match element.max_level {
                    MaxTalentLevel::Fixed(fixed) => fixed.fixed_max_level,
                    MaxTalentLevel::AttributeDependent(attr_dep) => {
                        match attr_dep.attribute_determining_level {
                            BaseAttribute::WeaponSkill => attributes[0].bonus(),
                            BaseAttribute::BallisticSkill => attributes[1].bonus(),
                            BaseAttribute::Strength => attributes[2].bonus(),
                            BaseAttribute::Toughness => attributes[3].bonus(),
                            BaseAttribute::Initiative => attributes[4].bonus(),
                            BaseAttribute::Agility => attributes[5].bonus(),
                            BaseAttribute::Dexterity => attributes[6].bonus(),
                            BaseAttribute::Intelligence => attributes[7].bonus(),
                            BaseAttribute::Willpower => attributes[8].bonus(),
                            BaseAttribute::Fellowship => attributes[9].bonus(),
                            BaseAttribute::Wounds => {
                                panic!("Cannot have talent dependent on wounds")
                            }
                            BaseAttribute::Movement => {
                                panic!("Cannot have talent dependent on Movement")
                            }
                        }
                    }
                    MaxTalentLevel::SkillDependent(skill_dep) => 1,
                };
                element.level = if element.level + 1 > max_level {
                    max_level
                } else {
                    element.level + 1
                };
            } else {
                talents.push(talent);
            }
        }
        // 2.2 Species' trappings
        for trapping in species.trappings.as_slice() {
            trappings.push(trapping.to_owned());
        }
        // 2.3 Species skills
        // Pick 3 unique positions for skills developed by 5
        // Pick 3 unique positions for skills developed by 3
        let picked_pos = self.unique_dice_values(species.possible_skills.len() as u32, 6);
        for pos in picked_pos {}
        // 3. Apply professions to character spec
        for profession in professions {
            let attr_required_advancement = 5 * profession.profession_level;
            for attr_id in profession.attributes.as_slice() {
                if let Some(attribute) = attributes
                    .iter_mut()
                    .find(|attr| attr.is_attribute(*attr_id))
                {
                    attribute.advance(attr_required_advancement.into());
                } else {
                    panic!("Invalid attribute id={:?} found in profession definition, profession name = {}", attr_id, profession.name);
                }
            }
            for trapping in profession.trappings.as_slice() {
                trappings.push(trapping.to_owned());
            }
            for talent_definition in profession.talents.as_slice() {
                let talent = self.talent_from_talent_definition(&talent_definition);
                if let Some(element) = talents.iter_mut().find(|x| **x == talent) {
                    element.level += 1;
                } else {
                    talents.push(talent);
                }
            }
        }
        Character {
            talents,
            skills,
            trappings,
            weapon_skill: attributes[0].value(),
            ballistic_skill: attributes[1].value(),
            strength: attributes[2].value(),
            toughness: attributes[3].value(),
            initiative: attributes[4].value(),
            agility: attributes[5].value(),
            dexterity: attributes[6].value(),
            intelligence: attributes[7].value(),
            willpower: attributes[8].value(),
            fellowship: attributes[9].value(),
            // wounds = BS + 2BT + BWp
            wounds: attributes[2].bonus() + 2 * attributes[3].bonus() + attributes[8].bonus(),
            movement: species.movement,
        }
    }
}
