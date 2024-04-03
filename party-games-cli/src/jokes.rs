use crate::utils::choose_random;
use reqwest::blocking;
use reqwest::Error;
use std::io;

fn fetch_jokes() -> Result<String, Error> {
    let url =
        "https://raw.githubusercontent.com/yesinteractive/dadjokes/master/controllers/jokes.txt";
    let body = blocking::get(url)?.text()?;

    Ok(body)
}

pub fn jokes() -> () {
    let jokes: String;

    let joke_response: Result<String, Error> = fetch_jokes();

    match joke_response {
        Ok(j) => jokes = j,
        Err(e) => {
            println!("There was an error fetching the jokes: {}", e);
            return;
        }
    }

    let joke: String = choose_random(jokes);

    // with the repo I chose, they are seperated by '<>'
    let joke_peices: Vec<&str> = joke.split("<>").collect();
    let setup: &str = joke_peices[0].trim();
    let punchline: &str = joke_peices[1].trim();

    println!("{}", setup);
    // dumbass way of pausing until they press enter lmao
    let mut _value = String::new();
    io::stdin()
        .read_line(&mut _value)
        .expect("Something went wrong...");
    println!("{}", punchline);

    return ();
}
