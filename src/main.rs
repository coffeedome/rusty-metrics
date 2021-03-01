mod files_handler;
mod stdin_handler;

use std::env;

fn main() {
    let input = env::args().nth(1);
    //if input is nothing then look in stdin:
    if input.is_none() {
        println!("Note: You are running with default stdin input option. The program will run indefinitely if no stdin is provided (via '|')")
        stdin_handler::read_stdin();
    } else {
        //otherwise take input as name of directory (e.g. "databucket")
        files_handler::handle_files(&input.unwrap());
    }
}
