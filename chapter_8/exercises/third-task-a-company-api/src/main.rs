// Using a hash map and vectors, create a text interface to
// allow a user to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::fmt::{Display, Write};
use std::io;

fn add_a_user_to_a_departament(
    user: &str,
    department: &str,
    company: &mut HashMap<&str, Vec<&str>>,
) -> CommandResult {
    todo!()
}

// list of all people in a department sorted alphabetically
fn list_ppl_in_a_department(company: &HashMap<&str, Vec<&str>>, department: &str) -> CommandResult {
    todo!()
}

// all people in the company by department sorted alphabetically
fn list_people_in_the_company(company: &HashMap<&str, Vec<&str>>) -> CommandResult {
    todo!()
}

enum Action {
    Add,
    List,
}
enum Object {
    User(String),
    People,
}
enum Operator {
    To,
    From,
}
enum Destination {
    Department(String),
    Company,
}

struct Command {
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
    fn parse_command(input: &str) -> Self {
        let mut cmd = Command::default();

        for (index, word) in input.split_whitespace().enumerate() {
            match (index, word) {
                (0, "Add") => {
                    cmd.action = Some(Action::Add);
                }
                (0, "List") => {
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
                (3, "Company") => {
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
    fn command_handler(&self, company: &mut HashMap<&str, Vec<&str>>) -> CommandResult {
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
                "Incorrect command. Try again. \nHelp: use \"Add `user` To `department_name`\" OR List people from `department_name`/`company`.".to_string(),
            )),
        }
    }
}

impl Default for Command {
    fn default() -> Self {
        Self {
            action: None,
            object: None,
            operator: None,
            destination: None,
        }
    }
}

struct CommandResult(Result<String, String>);

impl Display for CommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandResult(Ok(s)) => {
                write!(f, "Wynik: {s}")
            }
            CommandResult(Err(e)) => {
                write!(f, "Error: {e}")
            }
        }
    }
}

fn read_input() -> String {
    loop {
        let mut user_input = String::new();
        let result = io::stdin().read_line(&mut user_input);
        match result {
            Ok(_) => return user_input,
            Err(_) => continue,
        }
    }
}

fn run_command(company: &mut HashMap<&str, Vec<&str>>) {
    let input = read_input();
    let command = Command::parse_command(&input);
}

fn main() {
    let mut company: HashMap<&str, Vec<&str>> = HashMap::new();
    run_command(&mut company);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_add_a_user_to_a_department() {
        let mut company: HashMap<&str, Vec<&str>> = HashMap::new();

        let users_to_be_added_to_sales = ["Pracomił", "Dobromił", "Władysław"];
        let users_to_be_added_to_engineering = ["Bolesław", "Mściwój", "Wojtek"];

        for user in users_to_be_added_to_sales {
            add_a_user_to_a_departament(user, "Sales", &mut company);
        }
        for user in users_to_be_added_to_engineering {
            add_a_user_to_a_departament(user, "Engineering", &mut company);
        }

        let result = &company.get("Sales");
        assert_eq!(result, &Some(&vec!["Pracomił", "Dobromił", "Władysław"]));
    }

    #[test]
    fn test_list_of_all_people_in_a_department_sorted_alphabetically() {
        let mut company: HashMap<&str, Vec<&str>> = HashMap::new();
        let departament_to_test = "Sales";
        let other_departament_to_test = "Engineering";

        let tested_users_to_be_added = ["Strzeżymir", "Dobromił", "Bolesław"];
        for user in tested_users_to_be_added {
            add_a_user_to_a_departament(user, departament_to_test, &mut company);
        }

        // add a user to a different departament
        add_a_user_to_a_departament("Mściwój", other_departament_to_test, &mut company);

        let result =
            list_of_all_people_in_a_department_sorted_alphabetically(&company, departament_to_test);

        assert_eq!("Bolesław, Dobromił, Strzeżymir", result)
    }

    #[test]
    fn test_all_people_in_the_company_by_department_sorted_alphabetically() {
        let mut company: HashMap<&str, Vec<&str>> = HashMap::new();
        let users_to_be_added_to_sales = ["Strzeżymir", "Dobromił", "Władysław"];
        let users_to_be_added_to_engineering = ["Bolesław", "Mściwój", "Wojtek"];

        for user in users_to_be_added_to_sales {
            add_a_user_to_a_departament(user, "Sales", &mut company);
        }
        for user in users_to_be_added_to_engineering {
            add_a_user_to_a_departament(user, "Engineering", &mut company);
        }

        let result = all_people_in_the_company_by_department_sorted_alphabetically(&company);

        // The departament should be printed also alphabetically.
        assert_eq!(
            "Engineering: Bolesław, Mściwój, Wojtek\nSales: Dobromił, Strzeżymir, Władysław",
            result
        )
    }

    #[test]
    fn test_command_passer_for_add_user_command() {
        //Test if the correct command works
        let user_command = "Add Mądromira to Sales";
        let result = Command::parse_command(user_command);

        assert!(matches!(result, CommandHandler::AddUserToDep));
        let bad_cases = [
            // Test if the command with incorrect the command ending doesn't works
            "Adder Mściwój to Sales",
            // Test if the command with incorrect command does't work
            "Gdd Strzeżymir to Sales",
            // Test if mixed commadd doesn't works
            "Sales Addes to Mściwój",
            // Test if the departament name is incorrect it doesn't work
            "Add Bolesław to Lublin",
        ];

        for case in bad_cases {
            let user_command = case;
            let result = parse_command(user_command);
            assert!(matches!(result, CommandHandler::IncorrectCommand));
        }
    }

    #[test]
    fn test_command_passer_for_add_list_users_command() {
        let good_cases_dep = ["List people from Sales", "List people from Engineering"];

        for case in good_cases_dep {
            let user_command = case;
            let result = parse_command(user_command);
            assert!(matches!(result, CommandHandler::ListPeopleInDep));
        }

        let user_command = "List people from Sales";
        let result = parse_command(user_command);
        assert!(matches!(result, CommandHandler::ListPeopleInDep));

        // A bad case
        let user_command = "List triple from Sales";
        let result = parse_command(user_command);
        assert!(matches!(result, CommandHandler::IncorrectCommand));
    }
}
