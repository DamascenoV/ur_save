extern crate skim;
use database::get_by_name;
use skim::prelude::*;
use std::io::Cursor;
use webbrowser;

use clap::{Parser, Subcommand};

mod database;
mod models;

/*
 * TODO: Need to make the functionalities update, delete
 * Reorganize and Refactor the code.
 * Add tests
 */

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Urls {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Urls {
    fn select_command(&self) {
        match &self.command {
            Some(Commands::Get { name }) => {
                let url = get_url(name.to_string());

                if let Some(url) = url {
                    webbrowser::open(&url.url).unwrap();
                }
            }
            Some(Commands::Insert { name, url }) => {
                let _ = database::insert(&models::Url {
                    id: 0,
                    name: name.to_string(),
                    url: url.to_string(),
                });
            }
            Some(Commands::Update { name, url }) => {
                println!("Update {} {}", name, url);
            }
            Some(Commands::List) => {
                println!("List");
                list_all();
            }
            Some(Commands::Delete { name }) => {
                let _ = database::delete_by_name(name.to_string());
            }
            None => list_commands(),
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    Get { name: String },
    Insert { name: String, url: String },
    Update { name: String, url: String },
    List,
    Delete { name: String },
}

impl Commands {
    fn get_all_commands() -> String {
        return "get\ninsert\nupdate\nlist\ndelete".to_string();
    }
}

fn main() {
    let ur_url = Urls::parse();

    let database = database::check_database();
    match database {
        Ok(_) => (),
        Err(err) => println!("Error creating database, {}", err),
    }

    ur_url.select_command();
}

fn list_commands() {
    let commands = Commands::get_all_commands();

    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(true)
        .reverse(true)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(commands));

    let selected = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| vec![]);

    for item in selected.iter() {
        match item.output().to_string().as_str() {
            "get" => {
                let result = get_by_name(item.output().to_string());

                match result {
                    Ok(result) => {
                        open_url(result.url);
                    }
                    Err(err) => println!("{}", err),
                }
            }
            "insert" => {
                insert_url();
            }
            "update" => {}
            "delete" => {}
            "list" => {
                list_all();
            }

            _ => (),
        }
    }
}

fn list_all() {
    let mut data = String::new();
    let result = database::get_all();
    match result {
        Ok(result) => {
            for url in result {
                data.push_str(&url.name);
                data.push_str("\n");
            }
        }
        Err(_) => (),
    }

    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(true)
        .reverse(true)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(data));

    let selected = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| vec![]);

    for item in selected.iter() {
        let result = get_url(item.output().to_string());
        match result {
            Some(result) => {
                open_url(result.url);
            }
            None => (),
        }
    }
}

fn get_url(name: String) -> Option<models::Url> {
    let result = database::get_by_name(name);

    match result {
        Ok(result) => Some(result),
        Err(_) => None,
    }
}

fn open_url(name: String) {
    webbrowser::open(&name).expect("Failed to open browser");
}

fn insert_url() {
    println!("Enter the name:");
    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Enter the url:");
    let mut url = String::new();
    std::io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");

    let result = database::insert(&models::Url {
        id: 0,
        name: name.trim().to_string(),
        url: url.trim().to_string(),
    });

    match result {
        Ok(_) => println!("Inserted successfully"),
        Err(err) => println!("{}", err),
    }
}
