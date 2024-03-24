use term;
use std::io;
use std::io::prelude::*;
use termion::clear;

pub struct ConsoleLogger {}

pub trait LoggerFunctions {
    fn log_message(&self, message: &str, attributes: Vec<term::Attr>);
    fn log_message_no_linebreak(&self, message: &str,attributes: Vec<term::Attr>);
    fn log_error(&self, message: &str);
    fn log_break(&self);
    fn wait_for_input(&self);
    fn get_user_input(&self) -> String;
    fn log_title_page(&self);
}

impl LoggerFunctions for ConsoleLogger {
    fn log_message(&self, message: &str, attributes: Vec<term::Attr>) {
        let mut terminal = term::stdout().unwrap();
        for attr in attributes {
            terminal.attr(attr).unwrap();
        }

        println!("{}", message);

        terminal.reset().unwrap();

    }

    fn log_message_no_linebreak(&self, message: &str, attributes: Vec<term::Attr>) {
        let mut terminal = term::stdout().unwrap();
        for attr in attributes {
            terminal.attr(attr).unwrap();
        }

        print!("{:?}", message);

        terminal.reset().unwrap();
    }

    fn log_error(&self, message: &str) {
        let mut terminal = term::stdout().unwrap();
        terminal.fg(term::color::BRIGHT_RED).unwrap();

        print!("{:?}", message);

        terminal.reset().unwrap();
    }

    fn log_break(&self) {
        println!("");
    }

    fn wait_for_input(&self) {
        let mut stdout = io::stdout();
        stdout.write(b"Press Enter to continue...").unwrap();
        stdout.flush().unwrap();
        io::stdin().read(&mut [0]).unwrap();

        print!("\x1B[1A");
        print!("{}", clear::CurrentLine);
    }

    fn get_user_input(&self) -> String {
        let mut stdout = io::stdout();
        stdout.write(b"[devtool]>>>").unwrap();
        stdout.flush().unwrap();
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    fn log_title_page(&self) {
        println!(
        "
         _____                                                              _____ 
        ( ___ )                                                            ( ___ )
         |   |~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~|   | 
         |   | ███████╗ █████╗ ██╗   ██╗███████╗                            |   | 
         |   | ██╔════╝██╔══██╗██║   ██║██╔════╝                            |   | 
         |   | ███████╗███████║██║   ██║█████╗                              |   | 
         |   | ╚════██║██╔══██║╚██╗ ██╔╝██╔══╝                              |   | 
         |   | ███████║██║  ██║ ╚████╔╝ ███████╗                            |   | 
         |   | ╚══════╝╚═╝  ╚═╝  ╚═══╝  ╚══════╝                            |   | 
         |   | ███████╗██████╗ ██╗████████╗ ██████╗ ██████╗                 |   | 
         |   | ██╔════╝██╔══██╗██║╚══██╔══╝██╔═══██╗██╔══██╗                |   | 
         |   | █████╗  ██║  ██║██║   ██║   ██║   ██║██████╔╝                |   | 
         |   | ██╔══╝  ██║  ██║██║   ██║   ██║   ██║██╔══██╗                |   | 
         |   | ███████╗██████╔╝██║   ██║   ╚██████╔╝██║  ██║                |   | 
         |   | ╚══════╝╚═════╝ ╚═╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝                |   | 
         |   | ██████╗ ███████╗██╗   ██╗████████╗ ██████╗  ██████╗ ██╗      |   | 
         |   | ██╔══██╗██╔════╝██║   ██║╚══██╔══╝██╔═══██╗██╔═══██╗██║      |   | 
         |   | ██║  ██║█████╗  ██║   ██║   ██║   ██║   ██║██║   ██║██║      |   | 
         |   | ██║  ██║██╔══╝  ╚██╗ ██╔╝   ██║   ██║   ██║██║   ██║██║      |   | 
         |   | ██████╔╝███████╗ ╚████╔╝    ██║   ╚██████╔╝╚██████╔╝███████╗ |   | 
         |   | ╚═════╝ ╚══════╝  ╚═══╝     ╚═╝    ╚═════╝  ╚═════╝ ╚══════╝ |   | 
         |   |  Author: Marcel McHawk                                       |   | 
         |   |  License: MIT                                                |   | 
         |___|~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~|___| 
        (_____)                                                            (_____)
        "
        );     
    }
}

impl ConsoleLogger {
    pub fn new() -> Self {
        ConsoleLogger {}
    }
}