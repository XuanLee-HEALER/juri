use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    words: String,
}

fn main() {
    let args = Args::parse();
    println!("words => {}", args.words);
}
