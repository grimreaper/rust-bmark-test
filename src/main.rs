
use clap::Parser;
use clap::{arg, command, value_parser, ArgAction, Command};



fn main() {
    let args = argfile::expand_args_from(wild::args_os(), argfile::parse_fromfile, argfile::PREFIX).unwrap();
    let matches = cmd()
        .get_matches_from(args);

    println!("Hello, world!");
}

fn cmd() -> clap::Command {
    command!().arg(
        arg!(
                -w --word "word"
            ))
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
