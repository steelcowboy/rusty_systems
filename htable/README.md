# Htable
This project opens a file, read the contents, builds a Huffman Tree with the
contents, and displays the codes created for each character in the file.

# Building
Because of Rust's awesome build system, called `cargo`, it's really easy to
build this code! Simply run: `cargo build`. 

To run the executable for testing, just run `cargo run`.

# New Things in Rust
In this project I'm using crates and libraries.

## Crate
A *crate* is a computational unit -- it will import other modules, but only
the crate gets compiled. A crate can become a binary or library.

## Library
A *library* is a collection of external definitions for other crates to
import. This has a special compiler flag that tells it that we're making a
library. With this flag, the name of the output file will be
lib<basename>.rlib
