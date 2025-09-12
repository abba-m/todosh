use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::Path,
    process::{ExitCode, exit},
};

use clap::{App, Arg};
use csv::{Reader, ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};
use tabled::{Table, Tabled, settings::Style};

static DATABASE_PATH: &str = "data/db.csv";
static DATABASE_DIR: &str = "data";

#[derive(Debug, Serialize, Deserialize, Tabled, Clone)]
struct Todo {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "TASK")]
    task: String,
    #[serde(rename = "COMPLETED")]
    completed: bool,
}

impl Todo {
    fn new(id: usize, task: &str) -> Todo {
        Todo {
            id: id.to_string(),
            task: task.to_owned(),
            completed: false,
        }
    }
}

fn main() -> ExitCode {
    create_db_if_not_exists();

    let args = App::new("todosh.rs")
        .version("1.0.0")
        .about("Terminal based todo list app")
        .arg(
            Arg::with_name("command")
                .help("The command to run")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("The input for the command")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let pattern = args.value_of("command");

    match pattern {
        None => {
            println!("<command> is required");
            ExitCode::FAILURE
        }
        Some(cmd) if matches!(cmd, "create" | "update" | "delete" | "list" | "complete") => {
            if cmd == "list" {
                list_todos()
            } else if cmd == "create" {
                let mut input = String::new();
                println!("Enter new task (press enter to submit):");

                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        create_todo(input);

                        list_todos();
                    }
                    Err(error) => {
                        println!("error: {error}");
                        exit(1);
                    }
                }
            } else if cmd == "complete" {
                let value = args.value_of("input");

                if let Some(id) = value {
                    let id: u16 = id.parse().expect("Invalid Todo id supplied");

                    complete_todo(id.to_string());
                } else {
                    println!("error: Id is expected");
                    exit(1);
                }
            } else {
                println!("{} ran successfully", cmd);
            }
            ExitCode::SUCCESS
        }
        Some(_) => {
            println!("Invalid command");
            ExitCode::FAILURE
        }
    }
}

fn create_db_if_not_exists() {
    if !Path::new(DATABASE_DIR).exists() {
        if let Err(e) = fs::create_dir_all(DATABASE_DIR) {
            eprintln!("Failed to create database directory: {e}");
            exit(1);
        }
    }

    let db_exists = Path::new(DATABASE_PATH).is_file();

    if db_exists {
        return;
    }

    match File::create(DATABASE_PATH) {
        Ok(_) => {
            println!("Database created...")
        }
        Err(e) => {
            eprintln!("Failed to create database: {e:?}");
            exit(1);
        }
    }
}

fn get_reader() -> Reader<File> {
    match ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(DATABASE_PATH)
    {
        Ok(rdr) => rdr,
        Err(e) => {
            eprintln!("Failed to create csv reader: {e:?}");
            exit(1);
        }
    }
}

fn list_todos() {
    let mut reader = get_reader();
    let mut table_data: Vec<Todo> = Vec::new();

    for result in reader.deserialize() {
        let record: Todo = match result {
            Ok(row) => row,
            Err(e) => {
                println!("Failed to parse csv row: {e:?}");
                Todo {
                    id: String::new(),
                    task: String::new(),
                    completed: false,
                }
            }
        };
        table_data.push(record);
    }

    let mut table = Table::new(table_data);
    table.with(Style::modern());

    println!("{table}");
}

fn create_todo(input: String) {
    let mut reader = get_reader();
    let next_id = reader.records().count() + 1;
    let new_task = Todo::new(next_id, input.trim_end());

    let file = match OpenOptions::new()
        .append(true)
        .create(true)
        .open(DATABASE_PATH)
    {
        Ok(w) => w,
        Err(e) => {
            println!("Failed to open db.csv: {e:?}");
            exit(1);
        }
    };

    println!("NEXT_ID: {next_id}");

    let has_headers = next_id == 1;
    let mut writer = WriterBuilder::new()
        .has_headers(has_headers)
        .from_writer(file);

    match writer.serialize(new_task) {
        Ok(_) => {
            writer.flush().unwrap();
        }
        Err(e) => {
            println!("Failed to write new todo to db: {e:?}");
        }
    };
}

fn complete_todo(id: String) {
    let mut reader = get_reader();
    let mut updated = false;

    let updated_records = reader
        .deserialize()
        .map(|row| {
            let mut record: Todo = row.unwrap();

            if id == record.id {
                if !record.completed {
                    println!("Updating todo with id {}...", id);
                    updated = true;
                    record.completed = true;
                }
            }

            record
        })
        .collect::<Vec<Todo>>();

    if !updated {
        println!("Todo with ID '{}' not found or already completed.", id);
        list_todos();
        return;
    }

    match reader.get_mut().flush() {
        Ok(_) => {
            let file = match OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(DATABASE_PATH)
            {
                Ok(w) => w,
                Err(e) => {
                    println!("Failed to open db.csv: {e:?}");
                    exit(1);
                }
            };

            let mut writer = WriterBuilder::new().has_headers(true).from_writer(file);

            for todo in updated_records {
                if let Err(e) = writer.serialize(todo) {
                    println!("Failed to write updated todo to db: {e:?}");
                    exit(1);
                }
            }

            if let Err(e) = writer.flush() {
                println!("Failed to flush writer: {e:?}");
                exit(1);
            }

            list_todos();
        }
        Err(e) => {
            println!("Failed to flush reader: {e:?}");
            exit(1);
        }
    };
}
