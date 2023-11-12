use crate::models::character::Character;

pub trait CharacterDisplayer<T> {
	fn print(&self, character: &Character) -> T;
}
