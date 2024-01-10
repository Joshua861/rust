use clap::Parser;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use glob::glob;
use std::env::{self, current_dir};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author = "Josh", version = "0.0.1", about = "find rewritten in rust.", long_about = None)]
struct Args {
    /// Path of file
    #[arg()]
    path: String,
}

fn main() -> std::io::Result<()> {
    let matcher = SkimMatcherV2::default();
    let args = Args::parse();
    let dir = get_dir()?;
    let dir_str = dir.to_str().unwrap();
    let pattern = format!("{}/**", dir_str);

    for entry in glob(&pattern).expect("Failed to read glob.") {
        match entry {
            Ok(path) => {
                let local_path = path
                    .display()
                    .to_string()
                    .replace(current_dir().unwrap().to_str().unwrap(), "");
                if matcher
                    .fuzzy_match(&path.display().to_string(), &args.path)
                    .is_some()
                {
                    println!("{:?}", local_path)
                }
            }
            Err(e) => println!("{}", e),
        }
    }

    Ok(())
}

fn get_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}
