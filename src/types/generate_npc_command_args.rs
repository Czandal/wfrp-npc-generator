use std::process::Command;

use crate::models::{consts::{MIN_PROFESSION_LEVEL, MAX_PROFESSION_LEVEL}, profession};

use super::command::CommandError;


pub struct GenerateNpcCommandArgs<'a>(pub &'a Vec<String>);

impl<'a> std::ops::Deref for GenerateNpcCommandArgs<'a> {
    type Target = Vec<String>;
    fn deref(&self) -> &Vec<String> {
        &self.0
    }
}

impl<'a> GenerateNpcCommandArgs<'a> {
    pub fn proffessions(&self, arg_offset: Option<usize>)->Option<std::str::Split<char>> {
        let arg_offset = if let Some(arg_offset) = arg_offset { arg_offset } else { 0 };

        if let Some(proffessions) = self.0.iter().skip(arg_offset).next() {
            return Some(proffessions.split(' '))
        }
        else  {
            return None
        }
    }

    pub fn species(&self, arg_offset: Option<usize>)->Option<&String> {
        let arg_offset = if let Some(arg_offset) = arg_offset { arg_offset } else { 0 };

        if let Some(species) = self.0.iter().skip(arg_offset + 1).next() {
            return Some(species)
        }
        else  {
            return None
        }
    }

    pub fn validate_proffessions(&self, arg_offset: Option<usize>)->Result<&Self, CommandError> {
        if let Some(_invalid_argument) = self.proffessions(arg_offset)
                                                   .unwrap() // Should be checked before calling the function
                                                   .find(|profession| !Self::is_valid_profession(profession)) {
                                                        Err(CommandError::InvalidArguments)?
                                                   }
        Ok(self)
    }

    pub fn validate_species(&self, arg_offset: Option<usize>)->Result<&Self, CommandError> {
        if !Self::is_valid_species(self.species(arg_offset)
                                         .unwrap()) {
                                                 Err(CommandError::InvalidArguments)?
                                         };
        Ok(self)
    }

    fn is_valid_species(species: &str)->bool {
        species.len() > 1 &&
        species.chars().find(|c| !c.is_lowercase()).is_none()
    }
    fn is_valid_profession(profession: &str)->bool {
        profession.len() > 2 &&
        profession.chars().rev().skip(2).find(|c: &char| !c.is_lowercase() && *c != '_').is_none() &&
        profession.chars().rev().nth(1).unwrap() == '_' &&
        profession.ends_with(|c: char| c.to_digit(10).is_some_and(|level| (MIN_PROFESSION_LEVEL..MAX_PROFESSION_LEVEL).contains(&level)))
    }
}



#[test] 
fn is_valid_profession() {
    assert_eq!(GenerateNpcCommandArgs::is_valid_profession("a_b_3"),true);
}

#[test] 
fn is_not_valid_profession() {
    assert_eq!(GenerateNpcCommandArgs::is_valid_profession("a_b_4"),false);
}

#[test] 
fn proffessions() {
    let proffessions_string: Vec<String> = vec!["aaaa bb_bb".into()];
    let generate_npc_command_args = GenerateNpcCommandArgs(&proffessions_string);
    let proffessions: Vec<&str> = generate_npc_command_args.proffessions(None).unwrap().collect();
    assert_eq!(proffessions,(vec!["aaaa", "bb_bb"]));
}

#[test] 
fn validate_proffessions() {
    let proffessions_string: Vec<String> = vec!["generate-npc".into(), "aaaa_1 bb_bb_2".into(), "aaaa".into()];
    let generate_npc_command_args = GenerateNpcCommandArgs(&proffessions_string);
    
    assert_eq!(generate_npc_command_args.validate_proffessions(Some(1)).is_ok(), true);
}

#[test] 
fn species() {
    let proffessions_string: Vec<String> = vec!["aaaa bb_bb".into(), "asdas".into()];
    let generate_npc_command_args = GenerateNpcCommandArgs(&proffessions_string);
    let proffessions: Vec<&str> = generate_npc_command_args.proffessions(None).unwrap().collect();
    assert_eq!(proffessions,(vec!["aaaa", "bb_bb"]));
}

#[test] 
fn validate_species() {
    let proffessions_string: Vec<String> = vec!["generate-npc".into(), "aaaa_1 bb_bb_2".into(), "aaaa".into()];
    let generate_npc_command_args = GenerateNpcCommandArgs(&proffessions_string);
    
    assert_eq!(generate_npc_command_args.validate_species(Some(1)).is_ok(), true);
}