extern crate base64;
extern crate hex;

use crypto;
use std::f64;
use std::fs::File;
use std::io::prelude::*;

pub fn challenge_1() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let hex_string = hex::decode(input).expect("ok");
    let b64_string = base64::encode(&hex_string);
    println!("{} {:?}", input, b64_string);
}

pub fn challenge_2() {
    let input1 = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
    let input2 = hex::decode("686974207468652062756c6c277320657965").unwrap();

    let output = crypto::xor_repeating(&input1, &input2);
    println!("{}", hex::encode(output));
}

pub fn challenge_3() {
    let input = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
    let (key, conf) = crypto::find_single_xor(&input);
    println!("{} {} {}", key, String::from_utf8(crypto::xor_repeating(&input, &vec![key])).unwrap(), conf);
}

pub fn challenge_4() {
    let mut file = File::open("data/4.txt").unwrap();
    let mut all_file = String::new();
    file.read_to_string(&mut all_file).unwrap();

    let mut best_guess = (vec![], 0, f64::INFINITY);

    for line in all_file.lines() {
        let data = hex::decode(line).expect("Test");
        let (key, conf) = crypto::find_single_xor(&data);
        if conf < best_guess.2 {
            best_guess = (data, key, conf);
        }
    }
    let output = crypto::xor_repeating(&best_guess.0, &vec![best_guess.1]);
    println!("{} {}", String::from_utf8(output).unwrap(), best_guess.2);
}

pub fn challenge_5() {
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";

    let out = crypto::xor_repeating(&input.as_bytes().to_vec(), &"ICE".as_bytes().to_vec());

    let out_str = hex::encode(out);

    println!("{}", out_str);
}

pub fn challenge_6() {
    let mut file = File::open("data/6good.txt").unwrap();
    let mut all_file = String::new();
    file.read_to_string(&mut all_file).unwrap();
    let data = base64::decode(&all_file).unwrap();
    let key = crypto::break_xor(&data);
    let cleartext = crypto::xor_repeating(&data, &key);
    let cleartext_string = String::from_utf8(cleartext).unwrap();
    println!("{}", cleartext_string);
    println!("{}", hex::encode(&cleartext_string[0..32]));
}

pub fn challenge_7() {
    let mut file = File::open("data/7good.txt").unwrap();
    let mut all_file = String::new();
    file.read_to_string(&mut all_file).unwrap();
    let data = base64::decode(&all_file).unwrap();

    let key = "YELLOW SUBMARINE".as_bytes().to_vec();

    let out = crypto::aes_decrypt_ecb(&data, &key);

    let cleartext = String::from_utf8(out).unwrap();

    println!("{}", cleartext);
}
pub fn challenge_8() {
    let mut file = File::open("data/8.txt").unwrap();
    let mut all_file = String::new();
    file.read_to_string(&mut all_file).unwrap();

    let mut best_guess = ("", 0);
    for line in all_file.lines() {
        let data = hex::decode(line).unwrap();
        let nb_rep = crypto::number_repetition(&data, 16);
        if nb_rep > best_guess.1 {
            best_guess = (line, nb_rep);
        }
    }
    println!("{} {}", best_guess.0, best_guess.1);
}
pub fn challenge_9() {
    let data = "YELLOW SUBMARINE".as_bytes().to_vec();
    let out = crypto::pkcs7_padding(&data, 20);
    println!("{}", hex::encode(&out));
    assert_eq!(out.len(), 20);
    assert_eq!(hex::encode(&out), "59454c4c4f57205355424d4152494e4504040404");
}
pub fn challenge_10() {
    let mut file = File::open("data/10good.txt").unwrap();
    let mut all_file = String::new();
    file.read_to_string(&mut all_file).unwrap();
    let data = base64::decode(&all_file).unwrap();

    let key = "YELLOW SUBMARINE".as_bytes().to_vec();
    let iv = vec![0x00;16];
    let out = crypto::aes_decrypt_cbc(&data, &key, &iv);

    let cleartext = String::from_utf8(out.clone()).unwrap();

    println!("{}", cleartext);
}

pub fn challenge_11() {
    let mut file = File::open("data/11.txt").unwrap();
    let mut all_file = String::new();
    file.read_to_string(&mut all_file).unwrap();
    let data = base64::decode(&all_file).unwrap();

    let it_data = crypto::encryption_oracle(data.clone());
    if crypto::is_ecb(it_data) {
        println!("ECB");
    } else {
        println!("CBC");
    }
}

pub fn challenge_12() {
    println!("TODO");
}

pub fn challenge_13() {
    println!("TODO");
}

pub fn challenge_14() {
    println!("TODO");
}

pub fn challenge_15() {
    println!("TODO");
}
