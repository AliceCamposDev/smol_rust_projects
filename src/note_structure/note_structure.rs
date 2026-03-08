use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug)]
pub struct NoteMap {
    pub notes: HashMap<Uuid, Note>,
    pub nodes: usize,
    pub version: String,
}

impl NoteMap{
    pub fn new() -> Self{
        Self{
            notes: HashMap::new(),
            nodes: 0,
            version: String::from("1.0"),
        }
    }
    pub fn add_note(&mut self, note: Note) -> bool{
        self.notes.insert(note.note_id, note);
        self.nodes = self.nodes + 1;
        true
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawNote {
    pub raw_id: Uuid,
    pub raw_content: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub note_id: Uuid,
    pub raw_note: RawNote,
    pub chunks: HashMap<Uuid, ContentChunk>,
    pub rel_path: PathBuf,
    pub title: String,
    pub wiki_links: Vec<Uuid>,
    pub back_links: Vec<Uuid>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentChunk {
    pub chunk_id: Uuid,
    pub keywords: Vec<String>,
    pub content: String,
    pub size: usize,
}

impl Note {
    pub fn new(raw_note: RawNote, chunks: HashMap<Uuid, ContentChunk>, rel_path: PathBuf) -> Self {
        let title = Self::get_note_title(&raw_note);
        Self {
            tags: vec![String::from("none")],
            note_id: Uuid::new_v4(),
            raw_note,
            chunks: chunks,
            rel_path: rel_path,
            title,
            wiki_links: Vec::new(),
            back_links: Vec::new(),
        }
    }
    fn get_note_title(raw: &RawNote) -> String {
        let first_line = raw.raw_content.lines().next().unwrap_or("no title");
        first_line.to_string()
    }
}


pub fn parse_note(path: &PathBuf) -> Note {
    let content = fs::read_to_string(&path).expect("err reading file");
    let now = Utc::now();

    let raw_note = RawNote {
        raw_id: Uuid::new_v4(),
        raw_content: content.clone(),
        created_at: now,
        modified_at: now,
        size: content.len(),
    };

    let chunk = ContentChunk {
        chunk_id: Uuid::new_v4(),
        keywords: Vec::new(),
        content: String::from("content"),
        size: content.len(),
    };

    let mut chunks: HashMap<Uuid, ContentChunk> = HashMap::new();
    chunks.insert(chunk.chunk_id, chunk);

    let rel_path = path;

    let note = Note::new(raw_note, chunks, rel_path.clone());
    // println!("title: {}", note.title);
    // println!("path: {}", note.rel_path.to_string_lossy().to_string());
    // println!("size: {}", note.raw_note.size.to_string());
    note
}
