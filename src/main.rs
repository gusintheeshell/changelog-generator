pub mod cli;
pub mod git;
pub mod changelog;
pub mod file_utils;

fn main() {
  let matches = cli::run_cli();
    
    match git::get_commits(".") {
        Ok(commits) => {
            let changelog;
            if matches.args_present() {
                changelog = changelog::generate_markdown_changelog(commits);
            } else {
                changelog = changelog::generate_changelog(commits);
            }

            if let Some(output) = matches.get_one::<String>("output") {
                if let Err(e) = file_utils::save_to_file(&changelog, output) {
                    eprintln!("Error saving changelog: {}", e);
                } else {
                    println!("Changelog saved to {}", output);
                }
            } else {
                println!("{}", changelog);
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
