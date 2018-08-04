extern crate hex;

use std::f64;
use textproc;

pub fn find_single_xor(input: &Vec<u8>) -> (u8, f64) {
    let mut best_guess = (0, f64::INFINITY);
    for key in 0..u8::max_value() {
        let decrypted = xor_repeating(&input, &vec![key]);
        let diff = textproc::is_english(&decrypted);
        if diff < best_guess.1 {
            best_guess = (key, diff);
        }
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

fn find_keysize(input: &Vec<u8>) -> usize{
    let mut best_keysize = (0, f64::INFINITY);
    for guess in 3..40 {
        let block1 = input[0..guess*2].to_vec();
        let block2 = input[guess*2..guess*4].to_vec();
        let block3 = input[guess*4..guess*6].to_vec();
        let block4 = input[guess*6..guess*8].to_vec();
        let dist1 = textproc::hamming_distance(&block1, &block2);
        let dist2 = textproc::hamming_distance(&block3, &block4);
        let distance: f64 = (dist1 + dist2) as f64 / guess as f64;

        if distance < best_keysize.1 {
            best_keysize = (guess, distance);
        }
    }
    best_keysize.0
}

fn split_block(input: &Vec<u8>, nb_block: usize) -> Vec<Vec<u8>> {
    let mut out = vec![Vec::new(); nb_block];

    for byte in 0..input.len() {
        out[byte % nb_block].push(input[byte]);
    }

    out
}

#[test]
fn test_split_block() {
    let input = vec![1,2,3,1,2,3,1,2,3,1,2,3];
    let output = vec![vec![1,1,1,1], vec![2,2,2,2], vec![3,3,3,3]];
    assert_eq!(split_block(&input, 3), output);
}

/// breaks a repeating xor and returns the key
pub fn break_xor(input: &Vec<u8>) -> Vec<u8> {
    let keysize = find_keysize(input);

    break_xor_with_keylen(input, keysize)
}


fn break_xor_with_keylen(input: &Vec<u8>, keylen: usize) -> Vec<u8> {
    let strips = split_block(input, keylen);
    println!("strips.len {}", strips.len());
    let mut key: Vec<u8> = vec![];
    for strip in strips {
        let (key_byte, _) = find_single_xor(&strip);
        key.push(key_byte);
    }
    key
}
