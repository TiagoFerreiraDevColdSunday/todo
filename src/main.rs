use chrono::NaiveDate;
use std::env;
mod list;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: todo run -- <name> <description> <deadline (YYYY-MM-DD)>");
        return;
    }

    let name = &args[1];
    let description = &args[2];
    let deadline_as_string = &args[3];

    println!("Creation of a new to-do.");

    let date_parts: Vec<u32> = deadline_as_string
        .trim()
        .split('-')
        .map(|x| x.parse::<u32>().expect("Invalid date format"))
        .collect();

    let my_list = list::List {
        title: String::from(name),
        description: String::from(description),
        deadline: NaiveDate::from_ymd_opt(date_parts[0] as i32, date_parts[1], date_parts[2])
            .unwrap(),
    };

    // Access and print the fields of the List struct
    println!("Task: {}", my_list.title);
    println!("Description: {}", my_list.description);
    println!("Deadline: {}", my_list.deadline);
}
