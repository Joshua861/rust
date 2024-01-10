use {clap::Parser, std::fs};

#[derive(Parser, Debug)]
#[command(author = "Josh", version = "0.0.1", about = "cat rewritten in rust.", long_about = None)]
struct Args {
    /// Path of file
    #[arg()]
    path: String,
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.path).expect("Could not read file.");

    println!("{}", contents)
}
