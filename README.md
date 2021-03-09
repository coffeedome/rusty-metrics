# rusty-metrics program

Text analytics using Rust

![build](https://github.com/estebanes22/rusty-metrics/actions/workflows/rust.yml/badge.svg)

### Running the program:

Download the latest release via https://github.com/estebanes22/rusty-metrics/releases/new

#### Directory use case:
./rusty-metrics <directory containing file(s)> <word count sequence>

#### Pipe input:
cat somefile.txt | ./rusty-metrics

## Open Items:
### Upcoming features:)
- **Automated testing**: Include automated tests

### Known issues:
- **Empty stdin handling** - Stdin validation is still pending. Placeholder note until fix pushed: Note: You are running with default stdin input option. The program will run indefinitely if no stdin is provided (via '|')"

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0.txt)