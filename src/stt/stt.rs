use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TranscriptionResult {
    text: Option<String>,
    error: Option<String>,
}
pub fn stt_fn(){
    let audio_file = "./audio.wav";
    let transcription = transcribe_with_python(audio_file);
    match transcription {
        Ok(text) => println!("Transcription: {}", text),
        Err(e) => eprintln!("Error: {}", e),
    }
}


fn transcribe_with_python(audio_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let script_path = "./src/stt/py_stt.py";

    let output = Command::new("python")
        .arg(script_path)
        .arg(audio_path)
        .output()?;

    if !output.status.success() {
        eprintln!("Python script stderr: {}", String::from_utf8_lossy(&output.stderr));
        return Err("Python script failed".into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    let result: TranscriptionResult = serde_json::from_str(&stdout)?;

    if let Some(err) = result.error {
        return Err(err.into());
    }

    result.text.ok_or_else(|| "No text in response".into())
}