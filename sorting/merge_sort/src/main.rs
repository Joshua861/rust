use rand::Rng;
use std::time::Instant;

fn main() {
    let array: Vec<u8> = gen_arr(20);

    let start = Instant::now();
    let sorted = merge_sort(&array);
    let duration = start.elapsed();

    println!("Unsorted: {:#?}", &array);
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

fn merge_sort(vec: &Vec<u8>) -> Vec<u8> {
    if vec.len() < 2 {
        vec.to_vec()
    } else {
        let size = vec.len() / 2;
        let left = merge_sort(&vec[0..size].to_vec());
        let right = merge_sort(&vec[size..].to_vec());
        let merged = merge(&left, &right);

        merged
    }
}

fn merge(left: &Vec<u8>, right: &Vec<u8>) -> Vec<u8> {
    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<u8> = Vec::new();

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i += 1;
        }
    }

    if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j += 1;
        }
    }

    merged
}
