use std::{
    error::Error,
    fs::File,
    process::{ExitCode, exit},
};

use clap::{App, Arg};
use csv::{Reader, ReaderBuilder};
use serde::Deserialize;
use tabled::{Table, Tabled, settings::Style};

static DATABASE_PATH: &str = "data/db.csv";

#[derive(Debug, Deserialize, Tabled, Clone)]
struct Todo {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "TASK")]
    task: String,
    #[serde(rename = "COMPLETED")]
    completed: bool,
}

fn list_todos() -> Result<(), Box<(dyn Error + 'static)>> {
    let file = match File::open(DATABASE_PATH) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open database: {:?}", err);
            exit(1);
        }
    };
    let mut reader = Reader::from_reader(file);
    let records_count = reader.records().count();

    let file = File::open(DATABASE_PATH)?;
    let mut reader = ReaderBuilder::new().trim(csv::Trim::All).from_reader(file);
    let mut table_data: Vec<Todo> = Vec::with_capacity(records_count);

    for result in reader.deserialize() {
        let record: Todo = result?;
        table_data.push(record);
    }

    let mut table = Table::new(table_data);
    table.with(Style::modern());

    println!("{table}");
    Ok(())
}

fn main() -> ExitCode {
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
                if let Err(err) = list_todos() {
                    println!("{}", err);
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
