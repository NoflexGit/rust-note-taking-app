use std::io;

mod file_ops;
mod note_managment;
mod ui;

fn main() {
    let mut notes = match file_ops::load_notes() {
        Ok(notes) => notes,
        Err(_) => Vec::new(),
    };

    loop {
        println!("Enter command (Add, List, View, Delete, Exit):");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        match command.trim().to_lowercase().as_str() {
            "add" => {
                let title = ui::promt_user_for_input("Enter note title");
                let content = ui::promt_user_for_input("Enter note content");
                note_managment::add_note(&mut notes, title, content)
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
            _ => println!("Unknown command"),
        }
    }

    if let Err(e) = file_ops::save_notes(&notes) {
        println!("Error saving notes: {}", e);
    }
}
