// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

mod grid;
mod letters_prob;

use std::collections::HashMap;
use csv::Reader;
use std::error::Error;
use crate::grid::Grid;
use tauri::{Window, LogicalSize, Manager, Size};

#[tauri::command]
fn get_words() -> Vec<String> {
    let mut grid = Grid::new();
    grid.init();
    let words = grid.words;
    //println!("Words: {:?}", words);
    // let w = lex.get(&(6, 'e')).cloned();
    // w.unwrap_or_else(|| vec![])
    words
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let windows = app.webview_windows();

            let window = windows
                .get("main")
                .expect("window not found");

            window.set_size(Size::Logical(LogicalSize {
                width: 1920.0,
                height: 1080.0,
            }))?;

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_words])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
