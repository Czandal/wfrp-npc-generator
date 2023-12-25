use crate::models::{character::Character, profession::Profession, species::Species};

pub trait CharacterGenerator{
    fn generate<'a>(&self, professions: &Vec<Profession<'a>>, species: &'a Species) -> Character<'a>;
}
