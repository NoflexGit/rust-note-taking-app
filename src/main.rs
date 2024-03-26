use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, stdin, stdout, Read, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Note {
    title: String,
    content: String,
}

impl Note {
    fn new(title: String, content: String) -> Note {
        Note { title, content }
    }
}

fn main() {
    let mut notes = match load_notes() {
        Ok(notes) => notes,
        Err(_) => Vec::new(),
    };

    loop {
        println!("Enter command (Add, List, View, Delete, Exit):");
        let mut command = String::new();
        stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        match command.trim().to_lowercase().as_str() {
            "add" => {
                let title = promt_user_for_input("Enter note title");
                let content = promt_user_for_input("Enter note content");

                add_note(&mut notes, title, content)
            }
            "list" => list_notes(&notes),
            "view" => {
                let title = promt_user_for_input("Enter note title");
                view_note(&notes, &title)
            }
            "delete" => {
                let title = promt_user_for_input("Enter note title");

                delete_note(&mut notes, &title)
            }
            "exit" => {
                println!("Exiting and saving notes...");
                if let Err(e) = save_notes(&notes) {
                    println!("Error saving notes {}", e)
                }
                break;
            }
            _ => println!("Unknown command"),
        }
    }

    if let Err(e) = save_notes(&notes) {
        println!("Error saving notes: {}", e);
    }
}

fn promt_user_for_input(message: &str) -> String {
    print!("{}:", message);
    stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}

fn add_note(notes: &mut Vec<Note>, title: String, content: String) {
    let note = Note::new(title, content);

    notes.push(note)
}

fn list_notes(notes: &[Note]) {
    for note in notes {
        println!("{}", note.title);
    }
}

fn view_note(notes: &[Note], title: &str) {
    if let Some(note) = notes.iter().find(|&n| n.title == title) {
        println!("{}\n{}", note.title, note.content)
    } else {
        println!("Note not found")
    }
}

fn delete_note(notes: &mut Vec<Note>, title: &str) {
    if let Some(index) = notes.iter().position(|n| n.title == title) {
        notes.remove(index);
    } else {
        println!("Note not found")
    }
}

fn save_notes(notes: &[Note]) -> io::Result<()> {
    let file = File::create("notes.json")?;
    serde_json::to_writer(file, &notes)?;
    Ok(())
}

fn load_notes() -> io::Result<Vec<Note>> {
    let file = OpenOptions::new().read(true).open("notes.json");

    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let notes: Vec<Note> = serde_json::from_str(&contents)?;
            Ok(notes)
        }

        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                Ok(Vec::new())
            } else {
                Err(e)
            }
        }
    }
}
