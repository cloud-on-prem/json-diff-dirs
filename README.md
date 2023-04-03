# JSON Diff Dirs

`json-diff-dirs` is a command-line tool written in Rust that compares JSON files between two directories. It highlights differences in JSON structure and content, while ignoring files that only exist in the first directory.

## Requirements

- Rust 1.51 or higher

## Installation

Clone the repository:

```sh
    git clone https://github.com/yourusername/json-diff-dirs.git
    cd json-diff-dirs
    cargo build --release
```

```sh
    ./target/release/json-diff-dirs /path/to/dir1 /path/to/dir2
```

Replace /path/to/dir1 and /path/to/dir2 with the actual paths to your directories containing JSON files.

The output will display the differences between JSON files with the same name in both directories, using colors to highlight the changes.

## License

This project is released under the [MIT License](./LICENSE).
