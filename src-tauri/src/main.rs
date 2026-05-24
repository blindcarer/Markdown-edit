#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{api::dialog::FileDialogBuilder};
use std::fs;
use std::path::{Path, PathBuf};
use serde::Serialize;

#[derive(Serialize)]
struct FolderResult {
    root: String,
    files: Vec<String>,
}

fn collect_files(dir: &Path, files: &mut Vec<String>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, files)?;
        } else if let Some(s) = path.to_str() {
            files.push(s.to_string());
        }
    }
    Ok(())
}

#[tauri::command]
fn select_folder() -> Result<FolderResult, String> {
    let picked = FileDialogBuilder::new().pick_folder();
    match picked {
        Some(path) => {
            let mut entries = Vec::new();
            if let Err(e) = collect_files(&path, &mut entries) {
                return Err(e.to_string());
            }
            Ok(FolderResult { root: path.to_string_lossy().to_string(), files: entries })
        }
        None => Err("canceled".into())
    }
}

#[tauri::command]
fn read_text_file(path: String) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_text_file(path: String, content: String) -> Result<(), String> {
    fs::write(path, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_text_file_in_dir(dir: String, filename: String, content: String) -> Result<(), String> {
    let mut p = PathBuf::from(dir);
    p.push(filename);
    fs::write(p, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_dir(path: String) -> Result<Vec<String>, String> {
    let mut entries = Vec::new();
    match fs::read_dir(PathBuf::from(path)) {
        Ok(rd) => {
            for entry in rd.flatten() {
                if let Some(s) = entry.path().to_str() {
                    entries.push(s.to_string());
                }
            }
            Ok(entries)
        }
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![select_folder, read_text_file, write_text_file, write_text_file_in_dir, list_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
