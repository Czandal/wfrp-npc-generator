use crate::models::character::Character;

#[derive(Debug, Clone)]
pub struct CharacterAttributesIter<'a>{
    character: &'a Character,
    index: usize,
}

impl <'a> CharacterAttributesIter<'a> {
    pub fn new(character: &'a Character)->Self {
        Self{
            character,
            index: 0
        }
    }
}

pub struct CharacterAttribute {
    pub name: &'static str,
    pub level: u32
}

// In the future it should be safeguarded by a macro which checks that all fields are indeed in here
impl <'a> Iterator for CharacterAttributesIter<'a> {
    type Item = CharacterAttribute;
    fn next(&mut self) -> Option<Self::Item> {
        let field = match self.index {
            0 =>  ("Weapon Skill",      self.character.weapon_skill),
            1 =>  ("Ballistic Skill",   self.character.ballistic_skill),
            2 =>  ("Strength",          self.character.strength),
            3 =>  ("Toughness",         self.character.toughness),
            4 =>  ("Initiative",        self.character.initiative),
            5 =>  ("Agility",           self.character.agility),
            6 =>  ("Dexterity",         self.character.dexterity),
            7 =>  ("Intelligence",      self.character.intelligence),
            8 =>  ("Willpower",         self.character.willpower),
            9 =>  ("FellowShip",        self.character.fellowship),
            10 => ("Wounds",            self.character.wounds),
            11 => ("Movement",          self.character.movement),
            _ => return None,
        };

        self.index += 1;
        return Some(CharacterAttribute{name: field.0, level: field.1});
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn character_attribute_names() {
        let character = Character {
            talents: vec![],
            skills: vec![],
            trappings: vec![],
            ballistic_skill: 1,
            weapon_skill: 1,
            strength: 1,
            toughness: 1,
            initiative: 1,
            agility: 1,
            dexterity: 2,
            intelligence: 1,
            willpower: 3,
            fellowship: 3,
            wounds: 2,
            movement: 1
        };

        assert_eq!(CharacterAttributesIter::new(&character).map(|s| s.name).collect::<Vec<&str>>(),
                   vec!["Weapon Skill",
                        "Ballistic Skill",
                        "Strength",
                        "Toughness",
                        "Initiative",
                        "Agility",
                        "Dexterity",
                        "Intelligence",
                        "Willpower",
                        "FellowShip",
                        "Wounds",
                        "Movement",]);
    }
}

