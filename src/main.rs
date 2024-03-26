#[derive(Debug, Clone)]
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
    let mut notes: Vec<Note> = Vec::new();

    add_note(
        &mut notes,
        "My first note".to_string(),
        "This is content for the first note".to_string(),
    );
    add_note(
        &mut notes,
        "My second note".to_string(),
        "This is content for the second note".to_string(),
    );

    // List all notes
    list_notes(&notes);

    // View specific note
    view_note(&notes, "My first note");

    // Delete note
    delete_note(&mut notes, "My first note");

    list_notes(&notes);
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
