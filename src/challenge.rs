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

    let output = crypto::xor(&input1, &input2);
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
