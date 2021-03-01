# rusty-metrics program

Text analytics using Rust

## How to run program

### Pre-requisites:
Clone this repository:
```
git clone https://github.com/estebanes22/rusty-metrics.git
```

Install rustc and cargo:

For macOS or Linux run:
```
curl https://sh.rustup.rs -sSf | sh
```

For Windows download and run [the installer](https://win.rustup.rs/)

### Running the program:

Once rustc and cargo are installed, go to the root of the cloned repository

#### Getting word-sequence counts on multiple text files:
- Place the text file(s) in a directory
- Run:
```
cargo run "databucket" > validate_output.txt
```
You can then view the program output in the validate_output.txt file.

*Note:* the repo already contains a "/databucket" directory with Moby Dick and Origin of Species for test purposes.

#### Getting word-sequence counts from stdin
- Run:
```
cat <filename> | cargo run "databucket" > validate_output.txt
```

You can then view the program output in the validate_output.txt file.

## Test Cases:

### Test Case 1:
- **Title:** Program works on multiple input files
- **Description:** The user can run the program on more than one input file
- **Precodition:** The user has carried out the pre-steps above
- **Test Steps:**
  1) Place more than one file in the "databucket" directory at the root of the cloned repo
  2)Run:
    ```
    cargo run "databucket" > validate_output.txt
    ```
  3) View the program output in the validate_output.txt file. *Note:* you can also print output directly to stdout by omitting the ```> validate_output.txt``` part of the command.
  4) Paste the contents on an Excel spreadsheet to verify that there are 100 lines, descending by frequency

### Test Case 2:
- **Title:** Program works on stdin
- **Description:** The user can run the program on a piped input
- **Precodition:** The user has carried out the pre-steps above
- **Test Steps:**
  1)Run:
    ```
    cat databucket/mobydick.txt | cargo run "databucket" > validate_output.txt
    ```
  3) View the program output in the validate_output.txt file.
  4) Paste the contents on an Excel spreadsheet to verify that there are 100 lines, descending by frequency
  

## Open Items:
### Upcoming features:
- **Rustlang is a compiled language** and as such the compiled binary can be run on any machine without needing to have rust installed. The next feature will include a binary that can take arguments to perform the same tasks without needing cargo/rustc.
- **N string sequence and N most frequent lines**: The program is currently hard-coded to capture 3-word sequences and print the 100 most common 3-sequence grouprs. With parameters this can be extended to "N" count of words in sequence and "Z" most frequent groups in the set.
- **Run in container**: Dockerfile included to run on Alpine Linux container. This will include a small set RESTful APIs for accessibility via multiple types of thin clients (e.g. Postman, ReactJS, etc.)
- **Code coverage and static analysis**: the current code has a couple of assertions; more assertions will be added and Sonarqube implemented for static analysis.
- **Instrumentation for APM metrics with OpenTracing API**: some key methods will be instrumented with OpenTracing API in order to observe performance and make future performance improvements.

### Known issues:
- **Empty stdin handling** - Stdin validation is still pending. Placeholder note until fix pushed: Note: You are running with default stdin input option. The program will run indefinitely if no stdin is provided (via '|')"

- The program's output format is as follows:
```
("at the present", 13)
```
... whereas the output should be as follows:
```
at the present - 13
```
## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0.txt)