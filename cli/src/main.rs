use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    address: String,
}

fn main() {
    println!("LENZ CLI v0.1.0");
}