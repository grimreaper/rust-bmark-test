extern crate core;

use clap::{arg, command, value_parser, ArgAction, Command};
use clap::{Arg, Parser};
use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;

fn cmd() -> clap::Command {
    command!().arg(
        Arg::new("word")
            .short('w')
            .long("word")
            .action(ArgAction::Set)
            .value_name("WORD"),
    )
}

fn main() {
    let args = argfile::expand_args_from(wild::args_os(), argfile::parse_fromfile, argfile::PREFIX)
        .unwrap();
    let matches = cmd().get_matches_from(args);

    if let Some(word) = matches.get_one::<String>("word") {
        println!("testing for: {}", word);
        if bmark::simple_palindrome(word.as_str()) {
            println!("yes!")
        } else {
            println!("no!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        cmd().debug_assert()
    }
}
