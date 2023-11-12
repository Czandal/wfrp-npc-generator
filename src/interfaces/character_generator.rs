use crate::models::{character::Character, profession::Profession, species::Species};

pub trait CharacterGenerator{
    fn generate(&self, professions: &Vec<Profession>, species: &Species) -> Character;
}
