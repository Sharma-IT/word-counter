# Word Counter

This Rust program counts the total number of words in text input, either from files specified as command-line arguments or from standard input.

## Features

- Counts words from multiple input sources:
  - Files specified as command-line arguments
  - Standard input (keyboard)
- Handles whitespace and punctuation
- Case-insensitive word counting
- Error handling for file reading

## Usage

1. Develop executable of Rust file:

```sh
rustc word_counter.rs
```
2. Run the executable:
```sh
./word_counter
```

## How it works

1. The program checks for command-line arguments.
2. If arguments are provided, it reads the content of each specified file.
3. If no arguments are provided, it reads from standard input.
4. The input is processed to count words, ignoring whitespace and punctuation.
5. The total word count is displayed.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [GNU V.3 License](LICENSE).



