use crate::models::generation_base::GenerationBase;
use crate::models::{character::Character, talent::Talent, skill::Skill, trapping::Trapping};
use crate::types::console_character_displayer::ConsoleCharacterDisplayer;
use crate:: interfaces::character_displayer::CharacterDisplayer;

mod interfaces;
mod models;
mod types;

fn main() {
    let character = Character {
        talents: vec![Talent{name: "OtherTalent1".into(), level: 1, max_level: 2, description: "Desc".into(),},
                      Talent{name: "Talent31".into(), level: 1, max_level: 2, description: "Desc".into()},],
        skills: vec![Skill{name: "Skill1".into(), value: 32, base_attribute: models::base_attribute::BaseAttribute::Agility},
                     Skill{name: "OtherSkill2".into(), value: 32, base_attribute: models::base_attribute::BaseAttribute::BallisticSkill},],
        trappings: vec![Trapping{name: "Tropp1".into(), count: 2}, Trapping{name: "Tropp2".into(), count: 2},],
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
    println!("{}", ConsoleCharacterDisplayer::new().print(&character));
    let generation_base = GenerationBase {
        base_value: 10,
        dices: vec![],
    };
    println!("Hello, world! {:?}", generation_base);
}
