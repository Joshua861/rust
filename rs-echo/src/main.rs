use clap::Parser;

/// Simple program to print a string
#[derive(Parser, Debug)]
#[command(author = "Josh", version = "0.0.1", about = "Echo rewritten in rust.", long_about = None)]
struct Args {
    /// String to print
    #[arg()]
    string: String,

    /// Number of times to print
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("{}", args.string)
    }
}
