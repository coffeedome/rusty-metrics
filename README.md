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
cargo run "<name of directory>" > validate_output.txt
```
You can then view the program output in the validate_output.txt file.

*Note:* the repo already contains a "/databucket" directory with Moby Dick and Origin of Species for test purposes.

#### Getting word-sequence counts from stdin
- Run:
```
cat <filename> | cargo run > validate_output.txt
```

You can then view the program output in the validate_output.txt file.

## Open Items:
### Upcoming features:
- **Rustlang is a compiled language** and as such the compiled binary can be run on any machine without needing to have rust installed. The next feature will include a binary that can take arguments to perform the same tasks without needing cargo/rustc.
- **N string sequence**: The program is currently hard-coded to capture 3-word sequences. With parameters this can be extended to "N" count of words in sequence.
- **Run in container**: Dockerfile included to run on Alpine Linux container. This will include a small set RESTful APIs for accessibility via multiple types of clients (e.g. Postman, ReactJS, etc.)

### Known issues:
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