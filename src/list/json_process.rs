use crate::list::List;
use std::fs::{File};
use std::io::{Read, Write};
use serde_json;

pub fn check_existance(list: &List) -> bool {
    // Open the file or create it if it doesn't exist
    let mut file = match File::open("todo.json") {
        Ok(file) => file,
        Err(_) => {
            // If the file doesn't exist, create it and write an empty JSON array
            let mut new_file = File::create("todo.json").expect("Unable to create file");
            new_file
                .write_all(b"[]")
                .expect("Unable to write initial empty JSON array");
            new_file
        }
    };

    // Read the file's contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    // Deserialize the JSON into a Vec<List>
    let mut existing_lists: Vec<List> = serde_json::from_str(&contents).unwrap_or_else(|_| vec![]);

    // Check if the given list already exists
    let exists = existing_lists.iter().any(|existing| {
        existing.title == list.title && existing.description == list.description
    });

    if !exists {
        // If it doesn't exist, add it to the list and write back to the file
        existing_lists.push(list.clone()); // Clone the referenced List to create an owned value
        let json = serde_json::to_string(&existing_lists).expect("Unable to serialize JSON");
        let mut file = File::create("todo.json").expect("Unable to open file for writing");
        file.write_all(json.as_bytes()).expect("Unable to write to file");
    }

    exists
}