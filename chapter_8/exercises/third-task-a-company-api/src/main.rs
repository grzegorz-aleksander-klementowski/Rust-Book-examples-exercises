// Using a hash map and vectors, create a text interface to
// allow a user to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;

fn add_a_user_to_a_departament(
    user: &str,
    department: &str,
    company: &mut HashMap<&str, Vec<&str>>,
) {
    todo!()
}

fn list_of_all_people_in_a_department_sorted_alphabetically(
    company: &HashMap<&str, Vec<&str>>,
    department: &str,
) -> String {
    todo!()
}

fn all_people_in_the_company_by_department_sorted_alphabetically(
    company: &HashMap<&str, Vec<&str>>,
) -> String {
    todo!()
}

enum CommandHandler {
    AddUserToDep,
    ListPeopleInDep,
    ListPeopleInComByDep,
    IncorrectCommand,
}

impl CommandHandler {}

fn parse_command(input: &str) -> CommandHandler {
    todo!()
}

fn read_input() -> String {
    todo!()
}

fn main() {
    let company: HashMap<&str, Vec<&str>> = HashMap::new();
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
        let result = parse_command(user_command);

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
