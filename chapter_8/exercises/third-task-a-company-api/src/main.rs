use command::*;

mod command;

// Using a hash map and vectors, create a text interface to
// allow a user to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::fmt::Display;
use std::io;

// Add a user to a chosen departament
fn add_a_user_to_a_departament(
    user: &str,
    department: &str,
    company: &mut HashMap<String, Vec<String>>,
) -> CommandResult {
    // If there is no Vec for ppl, add it.
    company
        .entry(department.to_string())
        .or_default()
        .push(user.to_string());
    let res_message = format!("Added {user} to {department} department.");

    CommandResult(Ok(res_message))
}

// list of all people in a department sorted alphabetically
fn list_ppl_in_a_department(
    company: &HashMap<String, Vec<String>>,
    department: &str,
) -> CommandResult {
    let department_people = company.get(department);
    match department_people {
        Some(people) => {
            let s_list = people.join("\n");
            CommandResult(Ok(s_list))
        }
        None => {
            let e = format!("There is no {department} department.");
            CommandResult(Err(e))
        }
    }
}

// all people in the company by department sorted alphabetically
fn list_people_in_the_company(company: &HashMap<String, Vec<String>>) -> CommandResult {
    let mut list = String::new();

    // Sort keys
    let mut read_company: Vec<(&String, &Vec<String>)> = company.iter().collect();
    read_company.sort_by_key(|(department, _)| department.as_str());

    for (index, (department, people)) in read_company.into_iter().enumerate() {
        if index > 0 {
            list.push('\n');
        }

        let mut people_sorted = people.clone();
        people_sorted.sort();
        let ppl = people_sorted.join(", ");

        list.push_str(&format!("{department}: {ppl}"));
    }

    CommandResult(Ok(list))
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

fn main() {
    println!(
        "App using commands to communicate. \nUsage: \nadd `user` to `departament`\nlist people from `departament`\nlist people from `company`\n\n"
    );

    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    let modifier_company = &mut company;

    loop {
        let input = read_input();
        let command = Command::parse_command(input);
        let command_result = command.command_handler(modifier_company);
        match command_result {
            CommandResult(Ok(result)) => println!("{}", CommandResult(Ok(result))),
            CommandResult(Err(error)) => println!("{}", CommandResult(Err(error))),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_add_a_user_to_a_department() {
        let mut company: HashMap<String, Vec<String>> = HashMap::new();

        let users_to_be_added_to_sales = ["Pracomił", "Dobromił", "Władysław"];
        let users_to_be_added_to_engineering = ["Bolesław", "Mściwój", "Wojtek"];

        for user in users_to_be_added_to_sales {
            add_a_user_to_a_departament(user, "Sales", &mut company);
        }
        for user in users_to_be_added_to_engineering {
            add_a_user_to_a_departament(user, "Engineering", &mut company);
        }

        let result = add_a_user_to_a_departament("Mądromira", "Engineering", &mut company);
        assert_eq!(
            result,
            CommandResult(Ok("Added Mądromira to Engineering department.".to_string()))
        );

        let result = &company.get("Sales");
        assert_eq!(
            result,
            &Some(&vec![
                "Pracomił".to_string(),
                "Dobromił".to_string(),
                "Władysław".to_string()
            ])
        );
    }

    #[test]
    fn test_list_people_in_department() {
        let mut company: HashMap<String, Vec<String>> = HashMap::new();
        let department = "Sales";

        let users = [
            "Mściwój",
            "Bolesław",
            "Wojtek",
            "Pracomił",
            "Dobromił",
            "Władysław",
        ];

        for user in users {
            add_a_user_to_a_departament(user, department, &mut company);
        }

        let result = list_ppl_in_a_department(&company, department);
        assert_eq!(
            result,
            CommandResult(Ok(
                "Bolesław\nDobromił\nMściwój\nPracomił\nWładysław\nWojtek".to_string()
            ))
        )
    }

    #[test]
    fn test_list_people_in_the_company() {
        let mut company: HashMap<String, Vec<String>> = HashMap::new();
        let users_to_be_added_to_sales = ["Strzeżymir", "Dobromił", "Władysław"];
        let users_to_be_added_to_engineering = ["Bolesław", "Mściwój", "Wojtek"];

        for user in users_to_be_added_to_sales {
            add_a_user_to_a_departament(user, "Sales", &mut company);
        }
        for user in users_to_be_added_to_engineering {
            add_a_user_to_a_departament(user, "Engineering", &mut company);
        }

        let result = list_people_in_the_company(&company);

        // The departament should be printed also alphabetically.
        assert_eq!(
            result,
            CommandResult(Ok(
                "Engineering: Bolesław, Mściwój, Wojtek\nSales: Dobromił, Strzeżymir, Władysław"
                    .to_string()
            ))
        )
    }
}
