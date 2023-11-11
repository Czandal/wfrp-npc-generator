use crate::models::character::Character;

pub trait CharacterGenerator{
    fn generate(&self) -> Character;
}
