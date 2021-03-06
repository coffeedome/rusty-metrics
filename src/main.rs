mod files_handler;
mod stdin_handler;
mod process_data;
mod postprocess_data;

use std::env;

fn main() {
    //Source directory:
    let source = env::args().nth(1);

    //Word sequence count:
    let wordseqcount = env::args().nth(2).unwrap().parse().unwrap();

    let final_vec;

    if source.is_none() {
        final_vec = stdin_handler::handle_stdin(wordseqcount);
    } else {
        final_vec = files_handler::handle_files(source.unwrap().as_str(), wordseqcount);
    }

    postprocess_data::post_process(final_vec);
    
}
