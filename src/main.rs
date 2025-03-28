use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use list::json_process::check_existance;

mod list;

/// Command-line arguments parser
#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple to-do list manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Subcommands for the `todo` program
#[derive(Subcommand)]
enum Commands {
    /// Add a new to-do item
    Add {
        /// Name of the to-do item
        name: String,
        /// Description of the to-do item
        description: String,
        /// Deadline for the to-do item (YYYY-MM-DD)
        deadline: String,
    },
}

fn add() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add {
            name,
            description,
            deadline,
        } => {
            println!("Adding a new to-do...");

            let date_parts: Vec<u32> = deadline
                .trim()
                .split('-')
                .map(|x| x.parse::<u32>().expect("Invalid date format"))
                .collect();

            let my_list = list::List {
                title: name.clone(),
                description: description.clone(),
                deadline: NaiveDate::from_ymd_opt(
                    date_parts[0] as i32,
                    date_parts[1],
                    date_parts[2],
                )
                .unwrap(),
            };

            if check_existance(&my_list) {
                println!("This task already exists in the list.");
            } else {
                println!("Task added to the list.");
            }
        }
    }
}

fn main() {
    add();
}