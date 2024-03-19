mod file_analizer;
mod id_fetcher;
mod struct_data;
mod test_saves;
mod arguments;
mod commands;

use commands::{AnalyseSaveCommand, CommandFunctions};
use dotenv::dotenv;
use file_analizer::{change_items_amount, change_items_durability, create_backup_from_file, edit_inventory_item_chunk, edit_skill, export_save_for_pc, get_contents_from_file, load_save_file, load_save_file_pc, remove_inventory_item};
use struct_data::{SaveFile, InventoryChunk};
use id_fetcher::{fetch_ids, update_ids};
use std::io;
use std::io::prelude::*;

pub fn main() {
    dotenv().ok();

    // // Uncomment the following line to if .env file should be selected.
    // let file_path = std::env::var("FILE_PATH").expect("FILE_PATH must be set.");
    // let file_content: Vec<u8> = get_contents_from_file(&file_path).unwrap();
    // let save_file = load_save_file(&file_path, file_content);
    // println!("{:?}", save_file);

    let command = AnalyseSaveCommand::new(); // Use the type
    command.log_help_documentation();   
    pause();
}

fn pause() {
    let mut stdout = io::stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    io::stdin().read(&mut [0]).unwrap();
}