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

#[derive(Subcommand)]
enum Commands {
    Get { name: String },
    Insert { name: String, url: String },
    Update { name: String, url: String },
    List ,
    Delete { name: String },
}

fn main() {
    let ur_url = Urls::parse();

    let database = database::check_database();
    match database {
        Ok(_) => (),
        Err(err) => println!("Error creating database, {}", err),
    }

    match &ur_url.command {
        Some(Commands::Get { name }) => {
            //println!("Get: {:?}", database::get_by_name(name.to_string()));
            println!("Get")
        },
        Some(Commands::Insert { name, url }) => {
            //println!("Insert: {:?}", database::insert(&models::Url::new(0, name.to_string(), url.to_string())));
            println!("Insert")
        }
        Some(Commands::Update { name, url }) => {
            println!("Update")
        },
        Some(Commands::List) => {
            println!("List")
        },
        Some(Commands::Delete { name }) => {
            println!("Delete")
        },
        None => println!("No command, Go to list Commands"),
    }

}
