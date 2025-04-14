use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

// Define the structure for a single note
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Note {
    id: usize,
    title: String,
    content: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

// Define the structure for our notes database
#[derive(Debug, Serialize, Deserialize)]
struct NotesDb {
    notes: HashMap<usize, Note>,
    next_id: usize,
}

impl NotesDb {
    // Create a new database or load an existing one
    fn new(filename: &str) -> Self {
        if Path::new(filename).exists() {
            match fs::read_to_string(filename) {
                Ok(data) => match serde_json::from_str(&data) {
                    Ok(db) => return db,
                    Err(e) => {
                        eprintln!("Error parsing database: {}", e);
                    }
                },
                Err(e) => {
                    eprintln!("Error reading database file: {}", e);
                }
            }
        }

        // Return a new empty database if file doesn't exist or can't be read
        Self {
            notes: HashMap::new(),
            next_id: 1,
        }
    }

    // Save the database to disk
    fn save(&self, filename: &str) -> io::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write(filename, data)?;
        Ok(())
    }

    // Add a new note
    fn add_note(&mut self, title: String, content: String) -> usize {
        let now = Local::now();
        let id = self.next_id;

        let note = Note {
            id,
            title,
            content,
            created_at: now,
            updated_at: now,
        };

        self.notes.insert(id, note);
        self.next_id += 1;
        id
    }

    // Get a note by ID
    fn get_note(&self, id: usize) -> Option<&Note> {
        self.notes.get(&id)
    }

    // List all notes
    fn list_notes(&self) -> Vec<&Note> {
        self.notes.values().collect()
    }

    // Update a note
    fn update_note(&mut self, id: usize, title: Option<String>, content: Option<String>) -> bool {
        if let Some(note) = self.notes.get_mut(&id) {
            if let Some(new_title) = title {
                note.title = new_title;
            }

            if let Some(new_content) = content {
                note.content = new_content;
            }

            note.updated_at = Local::now();
            true
        } else {
            false
        }
    }

    // Delete a note
    fn delete_note(&mut self, id: usize) -> bool {
        self.notes.remove(&id).is_some()
    }
}

// Function to get user input
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    let db_file = "notes.json";
    let mut db = NotesDb::new(db_file);

    loop {
        println!("\n===== Rust Notes =====");
        println!("1. List all notes");
        println!("2. View a note");
        println!("3. Add a new note");
        println!("4. Edit a note");
        println!("5. Delete a note");
        println!("0. Exit");

        let choice = get_input("Enter your choice: ");

        match choice.as_str() {
            "1" => {
                let notes = db.list_notes();
                if notes.is_empty() {
                    println!("No notes found.");
                } else {
                    println!("\nID | Title | Created At");
                    println!("------------------");
                    for note in notes {
                        println!(
                            "{} | {} | {}",
                            note.id,
                            note.title,
                            note.created_at.format("%Y-%m-%d %H:%M")
                        );
                    }
                }
            }
            "2" => {
                let id_str = get_input("Enter note ID: ");
                if let Ok(id) = id_str.parse::<usize>() {
                    if let Some(note) = db.get_note(id) {
                        println!("\nTitle: {}", note.title);
                        println!("Created: {}", note.created_at.format("%Y-%m-%d %H:%M"));
                        println!("Updated: {}", note.updated_at.format("%Y-%m-%d %H:%M"));
                        println!("\n{}", note.content);
                    } else {
                        println!("Note not found.");
                    }
                } else {
                    println!("Invalid ID.");
                }
            }
            "3" => {
                let title = get_input("Enter note title: ");
                println!("Enter note content (finish with an empty line):");

                let mut content = String::new();
                loop {
                    let line = get_input("");
                    if line.is_empty() {
                        break;
                    }
                    content.push_str(&line);
                    content.push('\n');
                }

                let id = db.add_note(title, content);
                if let Err(e) = db.save(db_file) {
                    eprintln!("Error saving database: {}", e);
                } else {
                    println!("Note added with ID: {}", id);
                }
            }
            "4" => {
                let id_str = get_input("Enter note ID: ");
                if let Ok(id) = id_str.parse::<usize>() {
                    if let Some(note) = db.get_note(id) {
                        println!("Current title: {}", note.title);
                        println!("Current content: \n{}", note.content);

                        let new_title =
                            get_input("Enter new title (leave empty to keep current): ");
                        let title_option = if new_title.is_empty() {
                            None
                        } else {
                            Some(new_title)
                        };

                        println!("Enter new content (finish with an empty line, leave completely empty to keep current):");
                        let mut new_content = String::new();
                        let mut first_line = true;
                        loop {
                            let line = get_input("");
                            if line.is_empty() && (first_line || !new_content.is_empty()) {
                                break;
                            }
                            if !first_line {
                                new_content.push('\n');
                            }
                            new_content.push_str(&line);
                            first_line = false;
                        }

                        let content_option = if new_content.is_empty() {
                            None
                        } else {
                            Some(new_content)
                        };

                        if db.update_note(id, title_option, content_option) {
                            if let Err(e) = db.save(db_file) {
                                eprintln!("Error saving database: {}", e);
                            } else {
                                println!("Note updated successfully.");
                            }
                        } else {
                            println!("Failed to update note.");
                        }
                    } else {
                        println!("Note not found.");
                    }
                } else {
                    println!("Invalid ID.");
                }
            }
            "5" => {
                let id_str = get_input("Enter note ID: ");
                if let Ok(id) = id_str.parse::<usize>() {
                    if db.delete_note(id) {
                        if let Err(e) = db.save(db_file) {
                            eprintln!("Error saving database: {}", e);
                        } else {
                            println!("Note deleted successfully.");
                        }
                    } else {
                        println!("Note not found.");
                    }
                } else {
                    println!("Invalid ID.");
                }
            }
            "0" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
