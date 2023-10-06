#![allow(non_camel_case_types)]
#![feature(let_chains)]

use std::env;
use cat_ascii_faces::Cats;

const HELP_MESSAGE: &str =
"cargo-cat is a tool to print cats to the terminal (=^･ｪ･^=)

Usage:
    cargo cat [OPTIONS]

Options:
    --help, -h                  Print this help message
    --all, -a                   Print all cats

If no option is provided, `cargo cat` will print a random cat face.
Note that the cat may render slightly different depending on the terminal.
";


fn main() {
    let maybe_arg = env::args().nth(2);
    if let Some(arg) = maybe_arg {
        if arg == "-a" || arg == "--all" {
            let cats = Cats::all();
            for cat in cats {
                println!("{}", cat);
            }
        } else {
            println!("{}", HELP_MESSAGE);
        }
    } else {
        let mut cats = Cats::new();
        println!("{}", cats.cat());
    }
}
