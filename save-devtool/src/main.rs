mod save_logic;
mod commands;
mod logger;
mod application_logic;
mod arguments;

use dotenv::dotenv;
use application_logic as logic;

pub fn main() {
    dotenv().ok();
    logic::main();
}

/*
analyse-save --path "C:\Users\MarcelTurobin-Ort\Github\_Privat\DL2_Save_Editor_DevTool\save-devtool\saves\save_main_8.sav" --debug true
*/