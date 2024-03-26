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
    let note = Note::new("My first note".to_string(), "This is content".to_string());
    println!("Created a new note: {:?}", note)
}
