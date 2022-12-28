use clap::Parser;
use clap::{arg, command, value_parser, ArgAction, Command};

fn simple_palindrome(word: &str) -> bool {
    true
}

fn main() {
    let args = argfile::expand_args_from(wild::args_os(), argfile::parse_fromfile, argfile::PREFIX)
        .unwrap();
    let matches = cmd().get_matches_from(args);

    if let Some(word) = matches.get_one::<String>("word") {
        println!("testing for: {}", word);
        if simple_palindrome(word) {
            println!("yes!")
        } else {
            println!("no!")
        }
    }

    println!("Hello, world!");
}

fn cmd() -> clap::Command {
    command!().arg(arg!(
        -w --word "word"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_simple_empty() {
        assert_eq!(simple_palindrome(""), true);
    }

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        cmd().debug_assert()
    }
}
