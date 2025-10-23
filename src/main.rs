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

    if args.value_of("command").is_none() {
        println!("<command> is required");
        exit(1)
    }

    let pattern = args.value_of("command").unwrap();

    if !matches!(
        pattern,
        "create" | "update" | "delete" | "list" | "complete"
    ) {
        println!("Invalid command");
        exit(1)
    }

    match pattern {
        "list" => list_todos(),
        "create" => {
            let mut input = String::new();

            if let Some(input_str) = args.value_of("input") {
                input = input_str.to_string();
            } else {
                println!("Enter new task (press enter to submit):");

                if let Err(error) = io::stdin().read_line(&mut input) {
                    println!("error: {error}");
                    exit(1);
                }
            }

            create_todo(input);
            list_todos();
        }
        "complete" => {
            let value = args.value_of("input");

            if let Some(id) = value {
                let id: usize = id.parse().expect("error: Invalid Todo id supplied");
                let mut reader = get_reader();

                if id > reader.records().count() {
                    println!("error: No Todo with ID {id}");
                    exit(1)
                };

                complete_todo(id.to_string());
            } else {
                println!("error: Id is expected");
                exit(1);
            }
        }
        "update" => {
            let value = args.value_of("input");

            if value.is_none() {
                println!("error: Id is expected");
                exit(1)
            }

            let id: usize = value.unwrap().parse().expect("Invalid ID passed");
            let mut reader = get_reader();

            if id > reader.records().count() {
                println!("error: No Todo with ID {id}");
                exit(1)
            };

            update_todo(id.to_string());
        }
        "delete" => {
            let value = args.value_of("input");

            if value.is_none() {
                println!("error: Id is expected");
                exit(1);
            }

            let id: u16 = value.unwrap().parse().expect("Invalid Todo id supplied");

            delete_todo(id.to_string())
        }
        _ => println!("{pattern} ran successfully"),
    }

    ExitCode::SUCCESS
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

fn write_to_database(records: Vec<Todo>) {
    let mut reader = get_reader();

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

            for todo in records {
                if let Err(e) = writer.serialize(todo) {
                    println!("Failed to write updated todo to db: {e:?}");
                    exit(1);
                }
            }

            if let Err(e) = writer.flush() {
                println!("Failed to flush writer: {e:?}");
                exit(1);
            }
        }
        Err(e) => {
            println!("Failed to flush reader: {e:?}");
            exit(1);
        }
    }
}

fn complete_todo(id: String) {
    let mut reader = get_reader();
    let mut updated = false;

    let updated_records = reader
        .deserialize()
        .map(|row| {
            let mut record: Todo = row.unwrap();

            if id == record.id && !record.completed {
                println!("Updating todo with id {id}...");
                updated = true;
                record.completed = true;
            }

            record
        })
        .collect::<Vec<Todo>>();

    if !updated {
        println!("Todo with ID '{id}' not found or already completed.");
        list_todos();
        return;
    };

    write_to_database(updated_records);
    list_todos();
}

fn delete_todo(id: String) {
    let mut reader = get_reader();
    let mut updated = false;
    let mut deleted = String::new();

    let mut updated_records: Vec<Todo> = reader
        .deserialize::<Todo>()
        .filter_map(|row| {
            if let Ok(record) = row {
                if record.id == id {
                    updated = true;
                    deleted = record.task
                } else {
                    return Some(record);
                }
            }

            None
        })
        .collect();

    if updated {
        // update ids
        updated_records = updated_records
            .into_iter()
            .enumerate()
            .map(|(idx, mut record)| {
                let id = record.id.parse::<usize>().unwrap();
                if id != idx + 1 {
                    record.id = (idx + 1).to_string();
                }

                return record;
            })
            .collect()
    } else {
        println!("Todo with ID '{id}' not found");
        exit(1)
    }

    write_to_database(updated_records);
    list_todos();
    println!("Deleted task \"{deleted}\" with ID \"{id}\"");
}

fn update_todo(id: String) {
    let mut reader = get_reader();
    let mut updated = false;

    let updated_records = reader
        .deserialize()
        .map(|row| {
            let mut record: Todo = row.unwrap();

            if id == record.id {
                let mut input = String::new();
                println!("Update todo ({}):", record.task);

                if let Err(error) = io::stdin().read_line(&mut input) {
                    println!("error: {error}");
                    exit(1);
                }

                if !input.trim().is_empty() && input.ne(&record.task) {
                    record.task = input;
                    updated = true;
                }
            }

            record
        })
        .collect::<Vec<Todo>>();

    if updated {
        write_to_database(updated_records);
    }
    list_todos();
}
