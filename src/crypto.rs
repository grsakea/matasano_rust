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
