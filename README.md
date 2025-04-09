# mg
Manage Local Files

`mg` is a command-line tool designed to help you manage local files efficiently. It provides functionality to save, list, remove, and reset progress using labeled checkpoints.

## Features

- **Save Progress**: Save the current state of your files under a specific label.
- **List Progress**: View all saved labels along with their timestamps.
- **Remove Progress**: Delete a saved label and its associated data.
- **Reset Progress**: Restore files to a specific label or the latest saved progress.

## Installation

`mg` requires Rust and Cargo to be installed. You can install them by following the instructions at [https://rust-lang.org/tools/install](https://rust-lang.org/tools/install).

Once Rust and Cargo are installed, run the following command to install `mg`:
```
cargo install --git https://github.com/kaedehito/mg.git
```

## Usage

`mg` supports the following commands:

### Initialize mg Control
```sh
mg init <label_name>
```
Initialize a new `mg` control with the specified label. This sets up the necessary file structure and saves the initial state under the given label.

### Save Progress
```sh
mg save <label_name>
```
Save the current progress under the specified label. If no label is provided, it saves to the latest label.

### List Progress
```sh
mg list
```
List all saved progress with their timestamps.

### Remove Progress
```sh
mg remove <label_name>
```
Remove a saved label and its associated data.

### Reset Progress
```sh
mg reset <label_name>
```
Reset to a specific label. If no label is provided, it resets to the latest saved progress.

## Configuration

- **Ignore Files**: Use a `.mgignore` file to specify files or directories to exclude from saving. Place this file in the root of your project.

## File Structure

- **Saves Directory**: Saved progress is stored in the `.mg/saves` directory.
- **Labels**: Labels and metadata are managed in the `saves.json` file.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the [Apache 2.0 License](LICENSE).
