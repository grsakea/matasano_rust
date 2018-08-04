extern crate hex;

use std::f64;
use textproc;

pub fn find_single_xor(input: &Vec<u8>) -> (u8, f64) {
    let mut key = 0_u8;
    let mut best_guess = (0, f64::INFINITY);
    while key < u8::max_value() {
        let decoded = match String::from_utf8(xor_repeating(&input, &vec![key])) {
            Ok(data) => data,
            Err(_) => String::from("hello"),
        };
        let diff = textproc::is_english(&decoded);
        if diff < best_guess.1 {
            best_guess = (key, diff);
        }
        key += 1;
    }
    best_guess
}

pub fn xor_repeating(input: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let mut key_array : Vec<u8> = vec![];
    for i in 0..input.len() {
        key_array.push(key[i % key.len()]);
    }
    xor(input, &key_array)
}

pub fn xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut out : Vec<u8> = Vec::new();
    for (ba, bb) in a.iter().zip(b) {
        out.push(ba ^ bb)
    }
    out
}

pub fn find_keysize(input: &Vec<u8>) -> usize{
    let mut best_keysize = (0, f64::INFINITY);
    for guess in 3..40 {
        let block1 = input[0..guess].to_vec();
        let block2 = input[guess..guess*2].to_vec();
        let distance: f64 = textproc::hamming_distance(&block1, &block2) as f64 / guess as f64;

        if distance < best_keysize.1 {
            best_keysize = (guess, distance);
        }
    }
    best_keysize.0
}
