# todosh

A terminal-based todo list application written in Rust.

## About

todosh is a simple, efficient command-line todo list manager that stores your tasks in a CSV file. It provides a clean, modern table interface for viewing your todos and supports basic todo management operations.

## Features

- **List todos**: Display all your todos in a beautifully formatted table
- **CSV-based storage**: Simple, portable data storage in `data/db.csv`
- **Modern table display**: Clean, modern styling for easy reading
- **Command-line interface**: Quick and efficient terminal-based interaction

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Building from source

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd todosh
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The executable will be available at `target/release/todosh`

## Usage

### Basic Commands

Currently implemented:
- `list` - Display all todos in a formatted table

Planned commands (framework ready):
- `create` - Add a new todo
- `update` - Modify an existing todo
- `delete` - Remove a todo
- `complete` - Mark a todo as completed

### Examples

```bash
# List all todos
./todosh list

# Other commands (coming soon)
./todosh create "Buy groceries"
./todosh complete 1
./todosh delete 2
```

## Data Format

Todos are stored in `data/db.csv` with the following structure:

| Field | Type | Description |
|-------|------|-------------|
| ID | String | Unique identifier for the todo |
| TASK | String | Description of the task |
| COMPLETED | Boolean | Whether the task is completed |

Example:
```csv
ID, TASK, COMPLETED
1, Take out trash, false
2, Cook dinner, false
3, Learn rust, true
```

## Dependencies

- **clap**: Command-line argument parsing
- **csv**: CSV file reading and writing
- **serde**: Serialization and deserialization
- **tabled**: Beautiful table formatting for terminal output

## Development

### Project Structure

```
├── Cargo.toml          # Project configuration and dependencies
├── data/
│   └── db.csv         # Todo data storage
├── src/
│   ├── main.rs        # Main application logic
│   └── structs.rs     # Data structures (if needed)
└── target/            # Build artifacts
```

### Running in Development

```bash
# Run the application
cargo run -- list

# Run with debug output
RUST_LOG=debug cargo run -- list

# Build and run tests
cargo test
```

## Contributing

N/A

## Roadmap

- [x] Implement `create` command for adding new todos
- [ ] Implement `update` command for modifying existing todos
- [ ] Implement `delete` command for removing todos
- [x] Implement `complete` command for marking todos as done
- [ ] Add filtering and search capabilities
- [ ] Add due dates and priority levels
- [ ] Add color coding for different todo states
- [ ] Add configuration file support
- [ ] Add data backup and restore functionality

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Version

Current version: 1.0.0

---

*Built with ❤️ and Rust*
