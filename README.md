# Changelog Generator

This is a command-line utility for generating changelogs from Git commits. It allows generating changelogs in different formats and saving the result to a file or displaying it in the terminal.

## Features

- Generate changelogs from Git commits.
- Support for different changelog formats (Markdown, plain text, etc.).
- Option to save the generated changelog to a file.

## How to Use

### Prerequisites

- Rust installed on your machine. You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/).

### Steps

1. Clone the repository:

   ```sh
   git clone https://github.com/your-username/changelog-generator.git
   cd changelog-generator
   ```

2. Build the project:

   ```sh
   cargo build --release
   ```

3. Run the utility:
   ```sh
   ./target/release/changelog-generator --output changelog.md
   ```

### Arguments

- `--output <file>`: Specifies the file where the changelog will be saved. If not specified, the changelog will be displayed in the terminal.

## Libraries Used

- `clap`: For command-line argument parsing.
- `git2`: For interacting with Git repositories.
- `serde`: For data serialization and deserialization.
- `serde_json`: For working with JSON data.
- `chrono`: For date and time manipulation.

## Project Structure

- `src/cli.rs`: Module responsible for command-line argument parsing.
- `src/git.rs`: Module responsible for interacting with Git and obtaining commits.
- `src/changelog.rs`: Module responsible for generating the changelog from commits.
- `src/file_utils.rs`: Module responsible for saving the changelog to a file.

## Contributing

Contributions are welcome! Feel free to open issues and pull requests.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

This README provides an overview of the utility, how to use it, the libraries used, and the project structure. Feel free to customize it as needed.
