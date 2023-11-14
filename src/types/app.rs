use crate::types::command::Command;

use super::command::CommandError;
const USAGE_MESSAGE: &str = "MagicalName usage <command> [args]";

pub struct App {}

impl App {
    pub fn new()->Self {
        App{}
    }

    pub fn run(&self, args: impl Iterator<Item=String>)->Result<(), CommandError> {
        let args = args.skip(1) //first item is 'most of the time' the name of the executable
                                    .collect();

        match Command::parse_command(&args, None) {
            Ok(_) => {
                todo!("implement acutal app")
            }
            Err(e) => {
                match e {
                    CommandError::EmptyCommand => {
                        Self::print_usage_message();
                        Self::print_available_commands();
                    }
                    CommandError::InvalidCommand => {
                        println!("Command \"{}\" is not recognized", args.first().map_or(" ", |s| s));
                        Self::print_available_commands();
                    }
                    _ => println!("{}", e)
                }
                Err(e)?
            }
        }
        
    }

    fn print_usage_message() {
        println!("{}", USAGE_MESSAGE);
    }

    fn print_available_commands() {
        println!("Available commands");
        println!("{}", Self::get_formatted_command_list());
    }

    fn get_formatted_command_list()->String {
        Command::iterator().map(|cmd| format!("\t-{}\n", cmd.command_description())).collect()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test] 
    fn app_no_args() {
        assert_eq!(App::new().run(Vec::<String>::new().into_iter()).unwrap_err(), CommandError::EmptyCommand)
    }

    #[test] 
    fn app_invalid_command() {
        let invalid_command:Vec<String> =vec!["app_name".into(), "generate_npc".into()];
        assert_eq!(App::new().run(invalid_command.into_iter()).unwrap_err(), CommandError::InvalidCommand)
    }

    #[test] 
    fn app_invalid_arguments() {
        let invalid_command:Vec<String> =vec!["app_name".into(), "generate-npc".into(), "proffesions".into(), "species".into()];
        assert_eq!(App::new().run(invalid_command.into_iter()).unwrap_err(), CommandError::InvalidArguments)
    }

    #[test] 
    fn app_too_few_arguments() {
        let invalid_command:Vec<String> =vec!["app_name".into(), "generate-npc".into(), "proffesions".into()];
        assert_eq!(App::new().run(invalid_command.into_iter()).unwrap_err(), CommandError::TooFewArguments(1, 2))
    }

    #[test] 
    fn app_too_many_arguments() {
        let invalid_command:Vec<String> =vec!["app_name".into(), "generate-npc".into(), "proffesions".into(), "species".into(), "mine_is_bigger".into()];
        assert_eq!(App::new().run(invalid_command.into_iter()).unwrap_err(), CommandError::TooManyArguments(3, 2))
    }
}
