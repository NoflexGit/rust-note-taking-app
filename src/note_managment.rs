use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub content: String,
    tags: Vec<String>,
}

impl Note {
    pub fn new(title: String, content: String, tags: Vec<String>) -> Note {
        Note {
            title,
            content,
            tags,
        }
    }
}

pub fn add_note(notes: &mut Vec<Note>, title: String, content: String, tags: Vec<String>) {
    let note = Note::new(title, content, tags);

    notes.push(note)
}

pub fn list_notes(notes: &[Note]) {
    for note in notes {
        println!("{}", note.title);
    }
}

pub fn filter_by_tag<'a>(notes: &'a [Note], tag: &str) -> Vec<&'a Note> {
    notes
        .iter()
        .filter(|note| note.tags.contains(&tag.to_string()))
        .collect()
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
