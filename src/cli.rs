use clap::{Arg, ArgMatches, Command};

pub fn run_cli() -> ArgMatches {
  Command::new("changelog-cli")
      .version("1.0")
      .about("Generates changelogs following Conventional Commits")
      .arg(Arg::new("output")
           .short('o')
           .long("output")
           .value_name("FILE")
           .help("Output file for the changelog"))
      .arg(Arg::new("markdown")
           .short('m')
           .long("markdown")
           .help("Generate the changelog in Markdown format"))
      .get_matches()
}