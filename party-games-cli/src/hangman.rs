use crate::utils::choose_random;
use reqwest::blocking;
use reqwest::Error;
use std::io;

const MAX_LIVES: usize = 6;

pub fn hangman() {
    let words: String;
    let words_response = fetch_words();

    match words_response {
        Ok(w) => words = w,
        Err(e) => {
            println!("There was an error fetching the words: {}", e);
            return;
        }
    }

    let word: String = choose_random(words);
    let letters_in_word: Vec<char> = word.chars().collect();
    let lenth = word.len();
    let mut hidden = "_ ".repeat(lenth);
    let mut lives = MAX_LIVES;
    let mut guesses: Vec<char> = Vec::new();
    let mut won: bool = false;

    dbg!(&hidden);

    while won == false {
        println!(
            "{}\nGuessed letters: {}\n{}\n",
            get_pic(lives),
            format!("{:?}", guesses).replace("'", ""),
            hidden
        );

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong when reading input. Sorry");
        println!("\n");

        let guess = input.chars().next().unwrap();

        if !guess.is_alphabetic() {
            println!("That isn't a letter.")
        } else {
            if guesses.contains(&guess) {
                println!("You've already guessed that letter. Try again.");
            } else {
                if letters_in_word.contains(&guess) {
                    println!("Nice! That was in it.");

                    let indexes: Vec<usize> = letters_in_word
                        .iter()
                        .enumerate()
                        .filter_map(|(index, &item)| if item == guess { Some(index) } else { None })
                        .collect();

                    let mut hidden_chars: Vec<char> = hidden.chars().collect();
                    for i in indexes {
                        hidden_chars[i * 2] = guess;
                    }
                    hidden = hidden_chars.into_iter().collect();
                } else {
                    println!("Uh oh, it looks like you suck at this game. Pity, that.");

                    lives -= 1;
                }

                guesses.push(guess);

                println!("{}", "\n".repeat(3));

                won = word == hidden.replace(" ", "") && lives > 0;

                if lives <= 0 {
                    println!("You lose. Loser.");
                    get_pic(1);
                    println!("\nThe word was: {}", word);
                    return ();
                }
            }
        }
    }

    println!("You win!");

    return ();
}

fn fetch_words() -> Result<String, Error> {
    let url = "https://raw.githubusercontent.com/bitcoin/bips/master/bip-0039/english.txt";
    let body: String = blocking::get(url)?.text()?;

    Ok(body)
}

fn get_pic(lives: usize) -> String {
    let hangman_pics = vec![
        " +---+\n |   |\n      |\n      |\n      |\n=========",
        " +---+\n |   |\n O   |\n      |\n      |\n      |\n=========",
        " +---+\n |   |\n O   |\n |   |\n      |\n      |\n=========",
        " +---+\n |   |\n O   |\n /|   |\n      |\n      |\n=========",
        " +---+\n |   |\n O   |\n /|\\ |\n      |\n      |\n=========",
        " +---+\n |   |\n O   |\n /|\\ |\n /    |\n      |\n=========",
        " +---+\n |   |\n O   |\n /|\\ |\n / \\ |\n      |\n=========",
    ];

    hangman_pics[hangman_pics.len() - lives].to_string()
}
