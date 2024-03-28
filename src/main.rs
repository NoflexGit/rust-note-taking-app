use std::io;

use crate::ui::promt_user_for_input;

mod file_ops;
mod note_managment;
mod ui;

fn main() {
    let mut notes = match file_ops::load_notes() {
        Ok(notes) => notes,
        Err(_) => Vec::new(),
    };

    loop {
        println!("Enter command (Add, List, View, Delete, Exit, Filter):");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        match command.trim().to_lowercase().as_str() {
            "add" => {
                let title = promt_user_for_input("Enter note title");
                let content = promt_user_for_input("Enter note content");
                let tags_str = promt_user_for_input("Enter tags (comma-separated");
                let tags = tags_str
                    .split(",")
                    .map(|tag| tag.trim().to_string())
                    .collect();
                note_managment::add_note(&mut notes, title, content, tags);
                println!("Note has been added successfully");
            }
            "list" => note_managment::list_notes(&notes),
            "view" => {
                let title = ui::promt_user_for_input("Enter note title");
                note_managment::view_note(&notes, &title)
            }
            "delete" => {
                let title = ui::promt_user_for_input("Enter note title");
                note_managment::delete_note(&mut notes, &title)
            }
            "exit" => {
                println!("Exiting and saving notes...");
                if let Err(e) = file_ops::save_notes(&notes) {
                    println!("Error saving notes {}", e)
                }
                break;
            }
            "filter" => {
                let tag = promt_user_for_input("Enter tag to filter by");

                let filtered_notes = note_managment::filter_by_tag(&notes, &tag);

                if filtered_notes.is_empty() {
                    println!("No notes found with the tag: {}", tag)
                } else {
                    println!("Notes with the tag: {}", tag);

                    for note in filtered_notes {
                        println!("Title: {}", note.title)
                    }
                }
            }
            _ => println!("Unknown command"),
        }
    }

    if let Err(e) = file_ops::save_notes(&notes) {
        println!("Error saving notes: {}", e);
    }
}
