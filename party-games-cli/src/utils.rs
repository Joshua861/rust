use rand::seq::SliceRandom;

pub fn choose_random(text: String) -> String {
    let lines: Vec<&str> = text.lines().collect();
    (*lines.choose(&mut rand::thread_rng()).unwrap()).to_string()
}
