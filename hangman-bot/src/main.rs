use std::collections::HashMap;
use std::{fs, io};

fn main() {
    let mut words: Vec<String> = read_words();
    println!("Starting with {:?} words.", words.len());

    println!("What is your word?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read text from stdin.");
    let user_word = input.trim().to_lowercase();

    words.retain(|word| word.len() == user_word.len());
    println!(
        "Removed words the wrong length: {:?} words left.",
        words.len()
    );

    let mut used_letters: Vec<char> = vec![];

    let mut guess = " ".repeat(user_word.len());
    let mut guesses: i32 = 0;
    while guess != user_word {
        let letter = choose_letter(&used_letters, &words).unwrap();
        used_letters.push(letter);
        println!("I guess {}", letter);
        guesses += 1;

        if user_word.contains(&letter.to_string()) {
            let pos = user_word.find(letter).unwrap();
            guess.remove(pos);
            guess.insert(pos, letter);
            words.retain(|word| word.chars().nth(pos).unwrap() == letter);

            println!("{:?} words left.", words.len());

            if words.len() == 1 {
                guess = words[0].clone();
                println!("Correctly guessed {:?} after {:?} tries!", guess, guesses);
            }
        }

        if words.is_empty() {
            println!("Couldn't guess word. It probably isn't in the dictionary.");
            break;
        }
    }
}

fn read_words() -> Vec<String> {
    let contents = fs::read_to_string("static/words.txt").expect("Couldn't find words file.");
    let words: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();

    words
}

fn choose_letter(used: &Vec<char>, words: &Vec<String>) -> Option<char> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for word in words {
        let word_counts = count_letters(word);
        for (c, count) in word_counts {
            if !used.contains(&c) {
                *counts.entry(c).or_default() += count;
            }
        }
    }

    let max_count = counts.values().cloned().max()?;
    counts
        .into_iter()
        .filter(|(c, count)| count == &max_count && !used.contains(c))
        .next()
        .map(|(c, _)| c)
}

fn count_letters(word: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for c in word.chars() {
        *counts.entry(c).or_default() += 1;
    }
    counts
}
