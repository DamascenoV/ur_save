extern crate skim;
use skim::prelude::{*};
use std::io::Cursor;

use clap::{Parser, Subcommand};

mod database;
mod models;

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
                //println!("Get: {:?}", database::get_by_name(name.to_string()));
                println!("Get")
            }
            Some(Commands::Insert { name, url }) => {
                //println!("Insert: {:?}", database::insert(&models::Url::new(0, name.to_string(), url.to_string())));
                println!("Insert")
            }
            Some(Commands::Update { name, url }) => {
                println!("Update")
            }
            Some(Commands::List) => {
                println!("List")
            }
            Some(Commands::Delete { name }) => {
                println!("Delete")
            }
            None => list(),
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
        return "get\ninsert\nupdate\ndelete".to_string();
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

fn list() {
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
        print!("{}{}", item.output(), "\n");
    }
}
