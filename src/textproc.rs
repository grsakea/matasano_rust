pub fn is_english(input: &String) -> f64 {
    let cleaned_up = input.to_lowercase().replace(is_not_alphabetic, "");
    let mut freqs: Vec<LetterFreq> = Vec::new();
    freqs.push(LetterFreq{letter:'a', freq: 8.12} );
    freqs.push(LetterFreq{letter:'b', freq: 1.49} );
    freqs.push(LetterFreq{letter:'c', freq: 2.782} );
    freqs.push(LetterFreq{letter:'d', freq: 4.253} );
    freqs.push(LetterFreq{letter:'e', freq: 12.702} );
    freqs.push(LetterFreq{letter:'f', freq: 2.228} );
    freqs.push(LetterFreq{letter:'g', freq: 4.253} );
    freqs.push(LetterFreq{letter:'h', freq: 6.094} );
    freqs.push(LetterFreq{letter:'i', freq: 6.966} );
    freqs.push(LetterFreq{letter:'j', freq: 0.153} );
    freqs.push(LetterFreq{letter:'k', freq: 0.772} );
    freqs.push(LetterFreq{letter:'l', freq: 4.025} );
    freqs.push(LetterFreq{letter:'m', freq: 4.253} );
    freqs.push(LetterFreq{letter:'n', freq: 6.749} );
    freqs.push(LetterFreq{letter:'o', freq: 7.507} );
    freqs.push(LetterFreq{letter:'p', freq: 1.929} );
    freqs.push(LetterFreq{letter:'q', freq: 0.095} );
    freqs.push(LetterFreq{letter:'r', freq: 5.987} );
    freqs.push(LetterFreq{letter:'s', freq: 6.327} );
    freqs.push(LetterFreq{letter:'t', freq: 9.056} );
    freqs.push(LetterFreq{letter:'u', freq: 2.758} );
    freqs.push(LetterFreq{letter:'v', freq: 0.978} );
    freqs.push(LetterFreq{letter:'w', freq: 2.360} );
    freqs.push(LetterFreq{letter:'x', freq: 0.150} );
    freqs.push(LetterFreq{letter:'y', freq: 1.974} );
    freqs.push(LetterFreq{letter:'z', freq: 0.074} );

    let banned_letter = ['$', '#', '"', '(', ')', '&', '%'];

    for i in input.bytes() {
        if !i.is_ascii() {
            return 200.0;
        }
    }

    let mut diff = 0.0;

    for i in freqs {
        diff += (freq_letter(&cleaned_up, i.letter) - i.freq).abs();
    }
    for letter in banned_letter.iter() {
        if freq_letter(&input, *letter) != 0.0 {
            return 1000.0
        }
    }
    if freq_letter(&input, ' ') == 0.0 {
        return 1000.0
    }
    diff
}

pub fn hamming_distance(text1: &Vec<u8>, text2: &Vec<u8>) -> u32 {
    let data = (text1, text2);
    let mut dist = 0;
    for (i,j) in data.0.iter().zip(data.1) {
        let mut diff = i ^ j;
        while diff != 0 {
            dist += 1;
            diff = diff & diff-1;
        }
    }
    dist
}

fn freq_letter(text: &String, letter: char) -> f64 {
    100.0 * text.matches(letter).count() as f64 / text.len() as f64
}

struct LetterFreq {
    letter: char,
    freq: f64,
}

fn is_not_alphabetic(c: char) -> bool {
    !c.is_alphabetic()
}
