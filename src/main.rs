extern crate core;

use clap::{arg, command, value_parser, ArgAction, Command};
use clap::{Arg, Parser};
use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;

pub fn double_iterator_palindrome(word: &str) -> bool {
    word.chars().eq(word.chars().rev())
}

pub fn simple_palindrome(word: &str) -> bool {
    let mut chars = word.chars();

    loop {
        let s = chars.next();
        let e = chars.next_back();
        match (s, e) {
            (Some(s), Some(e)) => {
                if s != e {
                    return false;
                }
            }
            (Some(s), None) => {
                // even string and we're passed the middle
                return true;
            }
            (None, Some(e)) => {
                // odd string and we're at the middle
                return true;
            }
            (None, None) => {
                // an empty string
                return true;
            }
        }
    }
}

fn main() {
    let args = argfile::expand_args_from(wild::args_os(), argfile::parse_fromfile, argfile::PREFIX)
        .unwrap();
    let matches = cmd().get_matches_from(args);

    if let Some(word) = matches.get_one::<String>("word") {
        println!("testing for: {}", word);
        if simple_palindrome(word.as_str()) {
            println!("yes!")
        } else {
            println!("no!")
        }
    }
}

fn cmd() -> clap::Command {
    command!().arg(
        Arg::new("word")
            .short('w')
            .long("word")
            .action(ArgAction::Set)
            .value_name("WORD"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_simple_empty() {
        assert_eq!(simple_palindrome(""), true);
        assert_eq!(double_iterator_palindrome(""), true);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(simple_palindrome("a"), true);
        assert_eq!(double_iterator_palindrome("a"), true);
    }

    #[test]
    fn test_mutli_char() {
        assert_eq!(simple_palindrome("aaaaa"), true);
        assert_eq!(double_iterator_palindrome("aaaaa"), true);
    }

    #[test]
    fn test_mutli_char_obvious_false() {
        assert_eq!(simple_palindrome("abcdef"), false);
        assert_eq!(double_iterator_palindrome("abcdef"), false);
    }

    #[test]
    fn test_mutli_different_true() {
        assert_eq!(simple_palindrome("aba"), true);
        assert_eq!(double_iterator_palindrome("aba"), true);
    }

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        cmd().debug_assert()
    }
}
