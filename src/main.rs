mod readdata;
use std::fs;

fn main() {
    //put stdin in 1 MB files in the same databucket directory:

    //Mutable global vector:
    let mut global_vec = Vec::new();

    //Scan for files in databucket directory:
    let paths = fs::read_dir("databucket").unwrap();

    //Check whether there is data
    //let is_empty = paths?.next().is_none();
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

        println!("{:?}",global_vec[1]);

        //Assert final vector is not empty:
        assert!(!global_vec.is_empty());

        //Assert that final number of hashmaps in vec equals number of files in directory:
        //assert!(global_vec.len(),)
    }
}
