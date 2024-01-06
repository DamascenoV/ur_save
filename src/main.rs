use clap::Parser;

mod database;
mod models;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Urls {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    url: String
}

fn main() {
    let ur_url = Urls::parse();

    println!("Name: {}", ur_url.name);
    println!("Url: {}", ur_url.url);

    let database = database::check_database();
    match database {
        Ok(_) => (),
        Err(err) => println!("Error creating database, {}", err)
    }

    let url = models::Url::new(0, ur_url.name, ur_url.url);
    let insert = database::insert(&url);
    match insert {
        Ok(_) => (),
        Err(err) => println!("Error inserting data, {}", err)
    }
}
