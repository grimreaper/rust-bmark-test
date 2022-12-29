extern crate core;

use clap::{command, Arg, ArgAction};

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

    #[test]
    fn verify_cli() {
        cmd().debug_assert()
    }
}
