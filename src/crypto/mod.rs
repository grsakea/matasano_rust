extern crate hex;
extern crate base64;
extern crate rand;

use self::rand::{thread_rng, Rng};
use self::rand::distributions::Uniform;
use std::f64;
use textproc;
pub mod aes;

pub fn aes_encrypt_ecb(data: &Vec<u8>, key :&Vec<u8>) -> Vec<u8> {
    let mut out = vec![];
    for block in data.chunks(16) {
        let block_data = block.to_vec();
        let mut temp = aes::encrypt_block(&block_data, key);
        out.append(&mut temp);
    }
    out
}

pub fn aes_decrypt_ecb(data: &Vec<u8>, key :&Vec<u8>) -> Vec<u8> {
    let mut out = vec![];
    for block in data.chunks(16) {
        let block_data = block.to_vec();
        let mut temp = aes::decrypt_block(&block_data, key);
        out.append(&mut temp);
    }
    out
}

pub fn pkcs7_padding(data: &Vec<u8>, block_size: usize) -> Vec<u8> {
    let padder: u8 =
        if data.len() < block_size {
            (block_size % data.len()) as u8
        } else {
            (data.len() % block_size) as u8
        };
    let mut out = data.clone();

    while out.len() % block_size != 0 {
        out.push(padder);
    }
    out
}

pub fn aes_decrypt_cbc(data: &Vec<u8>, key: &Vec<u8>, iv: &Vec<u8>) -> Vec<u8> {
    let mut out = vec![];

    let mut to_xor = iv.clone();
    for block in data.chunks(16) {
        let block = block.to_vec();
        let temp = aes::decrypt_block(&block, &key);
        let temp = xor_repeating(&temp, &to_xor);
        out.append(&mut temp.clone());
        to_xor = block.clone();
    }
    out
}

pub fn aes_encrypt_cbc(data: &Vec<u8>, key: &Vec<u8>, iv: &Vec<u8>) -> Vec<u8> {
    let mut out = vec![];

    let mut temp = xor_repeating(&data[0..16].to_vec(), iv);
    temp = aes::encrypt_block(&temp, key);
    out.append(&mut temp.clone());
    for block in data[16..data.len()].chunks(16) {
        temp = xor_repeating(&temp, &block.to_vec());
        temp = aes::encrypt_block(&temp, key);
        out.append(&mut temp.clone());
    }
    out
}


#[test]
fn test_encrypt_cbc() {
    let cleartext = hex::decode("49276d206261636b20616e642049276d2072696e67696e27207468652062656c").unwrap().to_vec();
    let iv = vec![0;16];
    let key = "YELLOW SUBMARINE".as_bytes().to_vec();

    let my_encrypted = aes_encrypt_cbc(&cleartext, &key, &iv);
    let expected = hex::decode("091230aade3eb330dbaa4358f88d2a6cd5cf8355cb6823397ad43906df434455").unwrap().to_vec();
    assert_eq!(my_encrypted, expected);
}

#[test]
fn test_decrypt_cbc() {
    let encrypted = hex::decode("091230aade3eb330dbaa4358f88d2a6cd5cf8355cb6823397ad43906df434455").unwrap().to_vec();
    let iv = vec![0;16];
    let key = "YELLOW SUBMARINE".as_bytes().to_vec();

    let my_clear = aes_decrypt_cbc(&encrypted, &key, &iv);
    let expected = hex::decode("49276d206261636b20616e642049276d2072696e67696e27207468652062656c").unwrap().to_vec();
    assert_eq!(my_clear, expected);
}

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

fn xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
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
    let mut key: Vec<u8> = vec![];
    for strip in strips {
        let (key_byte, _) = find_single_xor(&strip);
        key.push(key_byte);
    }
    key
}

pub fn number_repetition(data: &Vec<u8>, chunk_size: usize) -> usize {
    let temp = data.chunks(chunk_size).collect::<Vec<&[u8]>>();
    let mut to_process = vec![];
    for i in &temp {
        to_process.push(i.to_vec());
    }

    to_process.sort_unstable();
    to_process.dedup();

    temp.len() - to_process.len()
}

fn junk_text() -> Vec<u8> {
    let mut rng = thread_rng();
    let number_char: usize = rng.gen_range(5, 10);

    let textrange = Uniform::new_inclusive('A' as u8, 'z' as u8);
    let v: Vec<u8> = thread_rng().sample_iter(&textrange).take(number_char).collect();
    v
}


pub fn encryption_oracle(input: Vec<u8>) -> Vec<u8> {
    let key = aes::random_key();
    let start_junk = junk_text();
    let end_junk = junk_text();

    let mut data = start_junk;
    data.extend(&input);
    data.extend(&end_junk);
    let data = pkcs7_padding(&data, 16);

    let encrypted = {
        if rand::random::<bool>() {
            println!("this is cbc");
            aes_encrypt_cbc(&data, &key, &vec![0;16])
        } else {
            println!("this is ecb");
            aes_encrypt_ecb(&data, &key)
        }
    };
    encrypted
}

pub fn is_ecb(input: Vec<u8>) -> bool {
    number_repetition(&input, 16) != 0
}
