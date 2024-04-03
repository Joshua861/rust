use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // The command; what you want to do.
    #[arg(short, long)]
    command: String,

    // Argument for command.
    #[arg(short, long)]
    arguement: String,
}

fn main() {
    let args = Args::parse();
}
