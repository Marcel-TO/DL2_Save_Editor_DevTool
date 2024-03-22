mod save_logic;
mod commands;
mod logger;
mod application_logic;

use dotenv::dotenv;
use application_logic as logic;

pub fn main() {
    dotenv().ok();
    logic::main();
}