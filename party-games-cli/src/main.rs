use clap::Parser;
mod jokes;
use jokes::jokes;
use titlecase::titlecase;
mod hangman;
mod utils;
use hangman::hangman;

/// Play simple party games from the cli.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Game to play (or use 'list' to list all games).
    #[arg(index = 1)]
    pub game: String,
}

const GAMES: [&str; 2] = ["jokes", "hangman"];

fn main() {
    let args: Args = Args::parse();
    let game: String = args.game.to_lowercase();

    match game.as_str() {
        "jokes" => jokes(),
        "list" => list_games(),
        "hangman" => hangman(),
        _ => println!("Game not found."),
    }
}

fn list_games() {
    println!("--- GAMES ---");
    for game in GAMES {
        println!("- {}", titlecase(game));
    }
}
