mod stdin_handler;
mod files_handler;

use std::env;

fn main() {

    let input = env::args().nth(1);
    //if input is nothing then look in stdin:
    if input.is_none() {
        input.unwrap(); //if no stdin this will throw an error
        stdin_handler::read_stdin();
    } else {
    //otherwise take input as name of directory (e.g. "databucket")
        files_handler::handle_files(&input.unwrap());
    }

}
