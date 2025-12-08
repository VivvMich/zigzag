// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

mod grid;

use std::collections::HashMap;
use csv::Reader;
use std::error::Error;
use crate::grid::Grid;

#[tauri::command]
fn get_words() -> Vec<String> {
    let mut grid = Grid::new();
    let lex : HashMap<(u8, char) ,Vec<String>> = grid.separate_words_by_size_and_letter();
    let w = lex.get(&(6, 'e')).cloned();
    w.unwrap_or_else(|| vec![])
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_words])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
