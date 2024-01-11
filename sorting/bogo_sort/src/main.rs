use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::time::Instant;

fn main() {
    let array: Vec<u8> = gen_arr(10);

    let start = Instant::now();
    let sorted = bogo_sort(array);
    let duration = start.elapsed();

    println!("Sorted: {:#?}", &sorted);
    println!("Time: {:#?}", &duration);
}

fn gen_arr(length: u8) -> Vec<u8> {
    let mut array = Vec::new();
    let mut rnd = rand::thread_rng();

    for _ in 0..length {
        let n: u8 = rnd.gen();
        array.push(n);
    }

    array
}

fn bogo_sort(mut arr: Vec<u8>) -> Vec<u8> {
    while !arr.windows(2).all(|window| window[0] <= window[1]) {
        arr.shuffle(&mut thread_rng());
    }
    arr
}
