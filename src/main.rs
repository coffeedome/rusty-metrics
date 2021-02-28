mod readdata;
use walkdir::WalkDir;

fn main() {
    //put stdin in a file and read from there:

    //iterate over all files in a directory: https://www.reddit.com/r/rust/comments/gel00y/how_to_read_multiple_files_into_a_vec/
    for entry in WalkDir::new("databucket") {
        readdata::process_data(entry.unwrap().file_name().to_str().unwrap());
        //println!("{}", entry.unwrap().file_name().to_str().unwrap());
    }
}
