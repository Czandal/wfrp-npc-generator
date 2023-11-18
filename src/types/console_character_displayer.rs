use std::fmt::Display;
use crate::interfaces::character_displayer::CharacterDisplayer;
use crate::models::{character::Character, talent::Talent, skill::Skill, trapping::Trapping};
use crate::types::character_attributes_iter::CharacterAttributesIter;

const TABLE_DELIMETER: char = '|';
const MAX_CONSOLE_DISPLAY_LENGTH: usize = 60;
const TALENT_NAME: &str         = "Talent        ";
const TALENT_LEVEL: &str        = "Levels        ";
const SKILL_NAME: &str          = "Skill         ";
const SKILL_VALUE: &str         = "Values        ";
const TRAPPING_NAME: &str       = "Trapping      ";
const TRAPPING_COUNT: &str      = "Count         ";
const GENERAL_SKILL_NAME: &str  = "General Skill ";
const GENERAL_SKILL_VALUE: &str = "Level         ";

macro_rules! create_table {
    ($iter: ident, $field_top: ident, $field_bottom: ident, $top_prefix: ident, $bottom_prefix: ident) => {{
        let iter = $iter;
        let top_row_elements = iter.clone().map(|el| el.$field_top.to_string());
        let bottom_row_elements = iter.clone().map(|el| el.$field_bottom.to_string());

        let min_top = top_row_elements.clone().max_by_key(|el| el.len()).expect("create_table_failed top_row empty").len();
        let min_bottom = bottom_row_elements.clone().max_by_key(|el| el.len()).expect("create_table_failed bottom_row empty").len();
        let min_width = min_bottom.max(min_top); 
        
        let elem_count = ((MAX_CONSOLE_DISPLAY_LENGTH - $top_prefix.len().max($bottom_prefix.len()))/ min_width).max(1);
        
        let zipped_strings_iter = top_row_elements.zip(bottom_row_elements);
        let mut peekable_data_iter = zipped_strings_iter.peekable();
        let peekable_data_iter = &mut peekable_data_iter;

        let mut ret = String::new();
        while peekable_data_iter.peek().is_some() {
            let table_chunk = peekable_data_iter.take(elem_count);
            let (top, bottom) =  table_chunk.fold((
                                                                format!("{}{}", $top_prefix, TABLE_DELIMETER),
                                                                format!("{}{}", $bottom_prefix, TABLE_DELIMETER)),
                                                                |(top, bottom), (top_el,bottom_el)| 
                                                                (top  + Self::create_table_record(top_el, min_width).as_str(),
                                                                bottom + Self::create_table_record(bottom_el, min_width).as_str()));
            ret += format!("{}\n{}\n", top, bottom).as_str();
            for _ in 0..elem_count {
                peekable_data_iter.peek();
            }
        }
        ret
    }

    };
}

pub struct ConsoleCharacterDisplayer {}

impl ConsoleCharacterDisplayer {
    pub fn new() ->Self {
        ConsoleCharacterDisplayer{}
    }
    fn create_table_record(displayed_data: impl Display, min_width: usize)->String {
        let displayed_string = displayed_data.to_string(); 
        let justify_length = min_width - displayed_string.len();
        let justify_string: String = std::iter::repeat(" ").take(justify_length).collect();
        
        format!("{}{}{}", displayed_string, justify_string, TABLE_DELIMETER)
    }

    fn create_talent_table(data: &Vec<Talent>)->String {

        let data_iter= data.iter();
        create_table!(data_iter, name, level, TALENT_NAME, TALENT_LEVEL)
    }
    fn create_skill_table(data: &Vec<Skill>)->String {

        let data_iter= data.iter();
        create_table!(data_iter, name, value, SKILL_NAME, SKILL_VALUE)
    }
    fn create_trappings_table(data: &Vec<Trapping>)->String {

        let data_iter= data.iter();
        create_table!(data_iter, name, count, TRAPPING_NAME, TRAPPING_COUNT)
    }
    fn create_general_skills_table(character: &Character)->String {

        let data_iter = CharacterAttributesIter::new(character);
        create_table!(data_iter, name, level, GENERAL_SKILL_NAME, GENERAL_SKILL_VALUE)
    }
}

impl CharacterDisplayer<String> for ConsoleCharacterDisplayer {
    fn print(&self, character: &Character)->String {
        const TALENTS_HEADER: &str = "---Talents---";
        const SKILLS_HEADER: &str = "---Skills---";
        const TRAPPINGS_HEADER: &str = "---Trappings---";
        const GENERAL_SKILLS_HEADER: &str = "---General Skills---";
        format!("{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}",
                TALENTS_HEADER,
                Self::create_talent_table(&character.talents),
                SKILLS_HEADER,
                Self::create_skill_table(&character.skills),
                TRAPPINGS_HEADER,
                Self::create_trappings_table(&character.trappings),
                GENERAL_SKILLS_HEADER,
                Self::create_general_skills_table(&character))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test] 
    fn create_table_record_test() {
        assert_eq!(ConsoleCharacterDisplayer::create_table_record(1,2), "1 |") 
    }
    #[test] 
    fn create_talents_table_test() {
        let talents = vec![Talent{name: "Tal".into(), level: 1, max_level: 3, description: "Desc".into()}];
        assert_eq!(ConsoleCharacterDisplayer::create_talent_table(&talents), "Talent        |Tal|\nLevels        |1  |\n") 
    }
}

