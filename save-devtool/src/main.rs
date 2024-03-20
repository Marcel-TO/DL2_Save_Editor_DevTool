mod save_logic;
mod commands;
mod logger;
mod application_logic;

use commands::analyse_command::AnalyseSaveCommand;
use commands::command_functions::CommandFunctions;
use dotenv::dotenv;
use save_logic::file_analizer::{change_items_amount, change_items_durability, create_backup_from_file, edit_inventory_item_chunk, edit_skill, export_save_for_pc, get_contents_from_file, load_save_file, load_save_file_pc, remove_inventory_item};
use logger::{ConsoleLogger, LoggerFunctions};
use save_logic::struct_data::{SaveFile, InventoryChunk};
use save_logic::id_fetcher::{fetch_ids, update_ids};

pub fn main() {
    dotenv().ok();

    // // Uncomment the following line to if .env file should be selected.
    // let file_path = std::env::var("FILE_PATH").expect("FILE_PATH must be set.");
    // let file_content: Vec<u8> = get_contents_from_file(&file_path).unwrap();
    // let save_file = load_save_file(&file_path, file_content);
    // println!("{:?}", save_file);

    let command = AnalyseSaveCommand::new(); // Use the type
    command.log_help_documentation();   
    
    let logger = ConsoleLogger::new();
    logger.wait_for_input();
}