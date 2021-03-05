mod files_handler;
mod stdin_handler;

use std::env;

fn main() {
    //Source directory:
    let source = env::args().nth(1);

    //Word sequence count:
    let wordseqcount = env::args().nth(2).unwrap().parse().unwrap();

    //if input is nothing then look in stdin:
    if source.is_none() {
        println!("Note: You are running with default stdin input option. The program will run indefinitely if no stdin is provided (via '|')");
        stdin_handler::read_stdin();
    }

    //Look for all files in databucket directory, including stdin buffer:
    files_handler::handle_files("databucket", wordseqcount);
}
