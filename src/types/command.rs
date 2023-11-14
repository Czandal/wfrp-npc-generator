use std:: slice::Iter;

use super::generate_npc_command_args::GenerateNpcCommandArgs;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandError {
    InvalidCommand,
    InvalidArguments,
    EmptyCommand,
    TooFewArguments(usize, usize),
    TooManyArguments(usize, usize),
}


impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::EmptyCommand => {
                write!(f, "Command was empty")
            }
            &Self::InvalidArguments => {
                write!(f, "Command Arguments Invalid")
            }
            &Self::InvalidCommand => {
                write!(f, "Command not defined")
            }
            &Self::TooFewArguments(given, expected) => {

                write!(f, "Command needs {}, and got {} arguments", given, expected)
            }
            &Self::TooManyArguments(given, expected) => {

                write!(f, "Command needs {}, and got {} arguments", given, expected)
            }
        } 
    }
}
#[derive(Debug, Clone, Copy,  PartialEq, Eq)]
pub enum Command {
    GenerateNpc,
}



impl Command {
    pub fn iterator() -> Iter<'static, Command> {
        const COMMANDS: [Command; 1] = [Command::GenerateNpc];
        COMMANDS.iter()
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Command::GenerateNpc => "generate-npc",
        }
    }

    pub fn from_str(command: &str) -> Result<Self, CommandError> {
        match Command::iterator().find(|command_enum| command_enum.to_str() == command) {
            Some(command) => {
                Ok(command.clone())
            }
            None => {
                Err(CommandError::InvalidCommand)?       
            }
        }
    }
    
    pub fn parse_command(args: &Vec<String>, arg_offset: Option<usize>)->Result<Command, CommandError> {
        let arg_offset: usize = if let Some(offset) = arg_offset { offset } else { 0 };
        match args.get(arg_offset) {
            Some(command) => {
                Ok(*Command::from_str(command.as_str())?.validate_command_arguments(args, arg_offset + 1)?)
            }
            None => {
                Err(CommandError::EmptyCommand)
            }
        }
    }
    
    pub fn command_description(&self) -> String {
        let args_desc = match self {
            Command::GenerateNpc => "<professions> <species>",
        };
        
        format!("{} [{}]", self.to_str(), args_desc)
    }

    fn command_params_count(&self) -> usize {
        match self {
            Command::GenerateNpc => 2,
        }
    }

    fn validate_command_arguments(&self, args: &Vec<String>, arg_offset: usize)->Result<&Self, CommandError> {
        if args.len() > self.command_params_count() + arg_offset {
            Err(CommandError::TooManyArguments(args.len().max(arg_offset) - arg_offset, self.command_params_count()))?
        }
        else if args.len() < self.command_params_count() + arg_offset {
            Err(CommandError::TooFewArguments(args.len().max(arg_offset) - arg_offset, self.command_params_count()))?
        };

        match self {
            Command::GenerateNpc => {
                let arg_offset = Some(1);

                GenerateNpcCommandArgs(args).validate_proffessions(arg_offset)?;
                GenerateNpcCommandArgs(args).validate_species(arg_offset)?;
            }
        }
        Ok(self)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test] 
    fn parse_generate_npc_command() {
        let args:Vec<String> =vec!["generate-npc".into(), "your_mother_3 your_father_2".into(), "thebest".into()];
        assert_eq!(Command::parse_command(&args, None),Ok(Command::GenerateNpc));
    }

    #[test] 
    fn parse_generate_npc_command_invalid_arguments() {
        let args:Vec<String> =vec!["generate-npc".into(), "your_father_8 your_mother_2".into(), "thebest".into()];
        assert_eq!(Command::parse_command(&args, None), Err(CommandError::InvalidArguments));
    }

    #[test] 
    fn parse_generate_npc_command_too_few_arguments() {
        let args:Vec<String> =vec!["generate-npc".into(), "thebest".into()];
        assert_eq!(Command::parse_command(&args, None), Err(CommandError::TooFewArguments(1, 2)));
    }

    #[test] 
    fn generate_npc_command_too_many_arguments() {
        let args:Vec<String> =vec!["generate-npc".into(), "warrior_1".into(), "warrior_1".into(), "thebest".into()];
        assert_eq!(Command::parse_command(&args, None), Err(CommandError::TooManyArguments(3, 2)));
    }

    #[test] 
    fn parse_command_command_missing() {
        let args:Vec<String> =vec![];
        assert_eq!(Command::parse_command(&args, None), Err(CommandError::EmptyCommand));
    }

    #[test] 
    fn parse_command_invalid_command() {
        let args:Vec<String> =vec!["nan-is-a-great-return-type-for-division-in-dynamically-typed-languages-especially-if-it-is-not-directly-comparable-with-integers-nor-itself-and-you-detect-it-at-runtime".into()];
        assert_eq!(Command::parse_command(&args, None), Err(CommandError::InvalidCommand));
    }
}
