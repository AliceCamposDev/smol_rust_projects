use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

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
    pub chunks: Vec<ContentChunk>,
    pub rel_path: PathBuf,
    pub title: String,
    pub wiki_links: Vec<Uuid>,
    pub back_links: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentChunk {
    pub chunk_id: Uuid,
    pub keywords: Vec<String>,
    pub content: String,
    pub size: usize,
}

impl Note {
    pub fn new(raw_note: RawNote, chunks: Vec<ContentChunk>, rel_path: PathBuf) -> Self {
        let title = Self::get_note_title(&raw_note);
        Self {
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

// idk why its reading as dead code but i don't want to investigate it rn
#[allow(dead_code)]
pub fn note_structure_fn() {
    let path = PathBuf::from("./intro.md");
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

    let chunks = vec![chunk];
    let rel_path = path;

    let note = Note::new(raw_note, chunks, rel_path);
    println!("title: {}", note.title);
    println!("path: {}", note.rel_path.to_string_lossy().to_string());
    println!("size: {}", note.raw_note.size.to_string());
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

    let chunks = vec![chunk];
    let rel_path = path;

    let note = Note::new(raw_note, chunks, rel_path.clone());
    println!("title: {}", note.title);
    println!("path: {}", note.rel_path.to_string_lossy().to_string());
    println!("size: {}", note.raw_note.size.to_string());
    note
}
