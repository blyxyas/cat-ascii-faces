# cat-ascii-faces & cargo-cat

This crate provides two packages, a library and a binary.

## The library

You can install the library on your project using the following command:

```terminal
cargo add cat_ascii_faces
```

Then, you can use it however you'd like:

```rs
use cat_ascii_faces::Cats;

fn main() {
    let cats = Cats::new();
    // Print some random cats
    println!("{}", cats.cat()); // (=^･ｪ･^=)
    println!("{}", cats.cat()); // ฅ(⌯͒• ɪ •⌯͒)ฅ❣
    println!("{}", cats.cat()); // o(=・ω・=o)

    // Print all cats (will print explicit Unicode because we're using the debug print "{:#?}")
    println!("{:#?}", Cats::all()); // [ ... ]
}
```

## The binary

The binary is even simpler :sparkles:

### Installation

```
cargo install cargo-cat
```

### Usage

```terminal
$ cargo cat --help
```

```terminal
cargo-cat is a tool to print cats to the terminal (=^･ｪ･^=)

Usage:
    cargo cat [OPTIONS]

Options:
    --help, -h                  Print this help message
    --all, -a                   Print all cats

If no option is provided, `cargo cat` will print a random cat face.
Note that the cat may render slightly different depending on the terminal.
```

