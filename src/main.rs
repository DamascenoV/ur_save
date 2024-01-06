use clap::Parser;

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

    println!("Hello {}", ur_url.name);
    println!("Url {}", ur_url.url);
}
