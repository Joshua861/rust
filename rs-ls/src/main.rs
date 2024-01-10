use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let dir = get_dir()?;
    let contents = fs::read_dir(&dir)?;

    for entry in contents {
        let entry = entry?;
        println!(
            "{}",
            entry
                .path()
                .display()
                .to_string()
                .replace(dir.to_str().unwrap(), "")
                .replace("/", "")
        );
    }

    Ok(())
}

fn get_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}
