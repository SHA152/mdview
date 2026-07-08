// MDView - lightweight markdown viewer
// Prevents an extra console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::Path;

/// Returns (path, content) of the file passed on the command line,
/// i.e. when the user double-clicks an .md file associated with MDView.
#[tauri::command]
fn get_cli_file() -> Option<(String, String)> {
    let arg = std::env::args().nth(1)?;
    let p = Path::new(&arg);
    if !p.is_file() {
        return None;
    }
    let content = fs::read_to_string(p).ok()?;
    Some((arg, content))
}

/// Reads any markdown file by path (used for reloading).
#[tauri::command]
fn read_md(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_cli_file, read_md])
        .run(tauri::generate_context!())
        .expect("error while running MDView");
}
