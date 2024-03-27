use std::fs::{File, OpenOptions};
use std::io::{self, Read};

use crate::note_managment::Note;

pub fn save_notes(notes: &[Note]) -> io::Result<()> {
    let file = File::create("notes.json")?;
    serde_json::to_writer(file, &notes)?;
    Ok(())
}

pub fn load_notes() -> io::Result<Vec<Note>> {
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
