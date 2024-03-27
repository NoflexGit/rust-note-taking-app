use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub content: String,
}

impl Note {
    pub fn new(title: String, content: String) -> Note {
        Note { title, content }
    }
}

pub fn add_note(notes: &mut Vec<Note>, title: String, content: String) {
    let note = Note::new(title, content);

    notes.push(note)
}

pub fn list_notes(notes: &[Note]) {
    for note in notes {
        println!("{}", note.title);
    }
}

pub fn view_note(notes: &[Note], title: &str) {
    if let Some(note) = notes.iter().find(|&n| n.title == title) {
        println!("{}\n{}", note.title, note.content)
    } else {
        println!("Note not found")
    }
}

pub fn delete_note(notes: &mut Vec<Note>, title: &str) {
    if let Some(index) = notes.iter().position(|n| n.title == title) {
        notes.remove(index);
    } else {
        println!("Note not found")
    }
}
