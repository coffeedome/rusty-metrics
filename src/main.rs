mod readdata;
use std::fs;

fn main() {
    //put stdin in 1 MB files in the same databucket directory:

    //Mutable global vector:
    let mut global_vec = Vec::new();

    //Scan for files in databucket directory:
    let paths = fs::read_dir("databucket").unwrap();

    //Check whether there is data
    //let is_empty = paths.next().is_none();
    let is_empty = false;

    if is_empty {
        println!("WARNING: No data to analyze. Verify stdin is nonempty and/or data files are in /databucket")
    } else {

        //Loop through files in directory:
        for path in paths {
            //Push resulting hashmap onto vector
            global_vec.push(
                //fn- input:file str; output:hashmap
                readdata::process_data(path.unwrap().path().to_str().unwrap()),
            );
        }

        //Assert final vector is not empty:
        assert!(!global_vec.is_empty());

        //Explicit re-scan of directory to validate post-run integrity:
        let integrity_val = fs::read_dir("databucket").unwrap().count();
        assert!(global_vec.len()==integrity_val, true);
    }

    //merge all hashmaps
    //https://docs.rs/object-merge/0.1.0-alpha1/object_merge/

}
