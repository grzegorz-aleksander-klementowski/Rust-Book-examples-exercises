use crate::*;

#[derive(Debug, PartialEq)]
pub enum Action {
    Add,
    List,
}
#[derive(Debug, PartialEq)]
pub enum Object {
    User(String),
    People,
}
#[derive(Debug, PartialEq)]
pub enum Operator {
    To,
    From,
}
#[derive(Debug, PartialEq)]
pub enum Destination {
    Department(String),
    Company,
}

#[derive(Debug, PartialEq, Default)]
pub struct Command {
    action: Option<Action>,
    object: Option<Object>,
    operator: Option<Operator>,
    destination: Option<Destination>,
}

// Make possibili of creating custom extendable commands.
// For example: "Add Procowój to Sales" will add the user to „Sales” departament, while
// "List people from Sales" will list people from departament „Sales” but
// "List people from Company" will list people from all of the departaments
// Every needed keywors is typed while the rest return „None” exept the places when it's taing the
// string like „user”, „departament”.
impl Command {
    pub fn parse_command(input: String) -> Self {
        let mut cmd = Command::default();

        for (index, word) in input.split_whitespace().enumerate() {
            match (index, word) {
                (0, "add") => {
                    cmd.action = Some(Action::Add);
                }
                (0, "list") => {
                    cmd.action = Some(Action::List);
                }
                (0, _) => {
                    cmd.action = None;
                }
                (1, "people") => {
                    cmd.object = Some(Object::People);
                }
                (1, "") => {
                    cmd.object = None;
                }
                (1, _) => {
                    cmd.object = Some(Object::User(word.to_string()));
                }
                (2, "to") => {
                    cmd.operator = Some(Operator::To);
                }
                (2, "from") => {
                    cmd.operator = Some(Operator::From);
                }
                (2, _) => {
                    cmd.operator = None;
                }
                (3, "company") => {
                    cmd.destination = Some(Destination::Company);
                }
                (3, "") => {
                    cmd.destination = None;
                }
                (3, _) => {
                    cmd.destination = Some(Destination::Department(word.to_string()));
                }
                (_, _) => return Command::default(),
            }
        }
        cmd
    }

    // It could reutrn a needed action, however, it return CommandHandler because of ability of testing
    pub fn command_handler(&self, company: &mut HashMap<String, Vec<String>>) -> CommandResult {
        match self {
            Self {
                action: Some(Action::Add),
                object: Some(Object::User(user)),
                operator: Some(Operator::To),
                destination: Some(Destination::Department(department)),
            } => add_a_user_to_a_departament(user, department, company),
            Self {
                action: Some(Action::List),
                object: Some(Object::People),
                operator: Some(Operator::From),
                destination: Some(Destination::Department(department)),
            } => list_ppl_in_a_department(company, department),
            Self {
                action: Some(Action::List),
                object: Some(Object::People),
                operator: Some(Operator::From),
                destination: Some(Destination::Company),
            } => list_people_in_the_company(company),
            _ => CommandResult(Err(
                "Incorrect command. Try again. \nHelp: use \"add `user` to `department_name`\" OR list people from `department_name`/`company`.".to_string(),
            )),
        }
    }
}

// Struct for better handling the results of the commands, making it also more testable
#[derive(Debug, PartialEq, Eq)]
pub struct CommandResult(pub Result<String, String>);

// Printing the finall result of the commands.
impl Display for CommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandResult(Ok(s)) => {
                write!(f, "{s}")
            }
            CommandResult(Err(e)) => {
                write!(f, "Error: {e}")
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parsing_command() {
        // Test if the correct command works
        let corr_add_user_cmd = Command {
            action: Some(Action::Add),
            object: Some(Object::User(String::from("Mądromira"))),
            operator: Some(Operator::To),
            destination: Some(Destination::Department("Sales".to_string())),
        };
        let user_input = String::from("add Mądromira to Sales");
        let result = Command::parse_command(user_input);
        assert_eq!(result, corr_add_user_cmd);

        let corr_list_ppl_dpt = Command {
            action: Some(Action::List),
            object: Some(Object::People),
            operator: Some(Operator::From),
            destination: Some(Destination::Department("Sales".to_string())),
        };
        let user_input = String::from("list people from Sales");
        let result = Command::parse_command(user_input);
        assert_eq!(result, corr_list_ppl_dpt);

        let corr_list_ppl_cmp = Command {
            action: Some(Action::List),
            object: Some(Object::People),
            operator: Some(Operator::From),
            destination: Some(Destination::Company),
        };
        let user_input = String::from("list people from company");
        let result = Command::parse_command(user_input);
        assert_eq!(result, corr_list_ppl_cmp);

        // Test if the command does't work
        let bad_cases = [
            // Test if the command with incorrect the command ending doesn't works
            String::from("Adder Mściwój to Sales"),
            // Test if the command with incorrect command does't work
            String::from("Gdd Strzeżymir to Sales"),
            // Test if mixed commadd doesn't works
            String::from("Sales Addes to Mściwój"),
            // Test if the departament name is incorrect it doesn't work
            String::from("Add Bolesław to Lublin"),
        ];

        let mut company: HashMap<String, Vec<String>> = HashMap::new();
        for case in bad_cases {
            let command = Command::parse_command(case);
            let result = command.command_handler(&mut company);
            let expected = CommandResult(Err(
                "Incorrect command. Try again. \nHelp: use \"add `user` to `department_name`\" OR list people from `department_name`/`company`.".to_string()));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_wrong_handling_commands() {
        let mut company: HashMap<String, Vec<String>> = HashMap::new();

        let incorr_list_usr = Command {
            action: Some(Action::List),
            object: Some(Object::User("Wrong".to_string())),
            operator: Some(Operator::To),
            destination: Some(Destination::Company),
        };
        let result = Command::command_handler(&incorr_list_usr, &mut company);
        assert_eq!(result, CommandResult(Err("Incorrect command. Try again. \nHelp: use \"add `user` to `department_name`\" OR list people from `department_name`/`company`.".to_string())));

        let incorr_add_ppl_cmp = Command {
            action: Some(Action::Add),
            object: Some(Object::People),
            operator: Some(Operator::To),
            destination: Some(Destination::Company),
        };
        let result = Command::command_handler(&incorr_add_ppl_cmp, &mut company);
        assert_eq!(result, CommandResult(Err("Incorrect command. Try again. \nHelp: use \"add `user` to `department_name`\" OR list people from `department_name`/`company`.".to_string())));

        let incorr_list_ppl_cmp = Command {
            action: Some(Action::List),
            object: Some(Object::People),
            operator: Some(Operator::To),
            destination: Some(Destination::Company),
        };
        let result = Command::command_handler(&incorr_list_ppl_cmp, &mut company);
        assert_eq!(result, CommandResult(Err("Incorrect command. Try again. \nHelp: use \"add `user` to `department_name`\" OR list people from `department_name`/`company`.".to_string())));
    }
}
