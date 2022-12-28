use clap::Parser;
use clap::{arg, command, value_parser, ArgAction, Command};
use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;

fn simple_palindrome(word: &str) -> bool {
    let forward = word.chars();
    let backwards = word.chars().rev();

    for set in forward.zip_longest(backwards) {
        match set {
            Both(s, e) => {
                if s != e {
                    return false;
                }
            }
            Left(_) => {
                // even string and we're passed the middle
                return true;
            }
            Right(_) => {
                // odd string and we're at the middle
                return true;
            }
        }
    }
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
    fn test_single_char() {
        assert_eq!(simple_palindrome("a"), true);
    }

    #[test]
    fn test_mutli_char() {
        assert_eq!(simple_palindrome("aaaaa"), true);
    }

    #[test]
    fn test_mutli_char_obvious_false() {
        assert_eq!(simple_palindrome("abcdef"), false);
    }

    #[test]
    fn test_mutli_different_true() {
        assert_eq!(simple_palindrome("aba"), true);
    }

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        cmd().debug_assert()
    }
}
