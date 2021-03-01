mod stdin_handler;
mod files_handler;

use std::env;

fn main() {

    //put stdin in file in directory:
    let input = env::args().nth(1).unwrap();
    if input == "-" {
        let _ = stdin_handler::read_stdin();
    }
    //process all data:
    files_handler::handle_files("databucket");

}
