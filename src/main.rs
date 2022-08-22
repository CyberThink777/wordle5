mod tests;
/// 32-bit representation of word
/// 26 bits for A-Z, 6bits unused
/// 26                        1
/// A BCDEFGHIJKLMNOPQRSTUVWXYZ
///    1 1   1    1  1         fjord
/// ---D-F---J----O--R-------- fjord
mod representation {
    //Operation
    // -------------------------- fjord AND gucks (INTERSECTION)
    // --CD-FG--JK---O--RS-U----- fjord OR gucks (UNION)

    pub fn encode(word: &str) -> u32 {
        let word = word.to_ascii_lowercase();
        let mut bits = 0u32;
        for ch in word.chars() {
            bits |= 1 << 25 >> ((ch as u32 - 19) % 26);
        }
        bits
    }
}

const WORDS: &str = include_str!("../wordlist.txt");

fn main() {
    use itertools::Itertools;
    use std::time::Instant;
    let time = Instant::now();

    let useable_words: Vec<(u32, &str)> = WORDS
        .split_whitespace()
        .map(|w| (representation::encode(w), w))
        //remove dups
        .filter(|(w, _)| w.count_ones() == 5)
        //remove anagram
        .unique_by(|(w, _)| *w)
        .collect();
    let useable_words_len = useable_words.len();

    for (i, a) in useable_words.iter().enumerate() {
        for j in i + 1..useable_words_len {
            let b = useable_words[j];
            if a.0 & b.0 != 0 {
                continue;
            };
            let ab = a.0 | b.0;

            for k in j + 1..useable_words_len {
                let c = useable_words[k];
                if ab & c.0 != 0 {
                    continue;
                };
                let abc = ab | c.0;

                for l in k + 1..useable_words_len {
                    let d = useable_words[l];
                    if abc & d.0 != 0 {
                        continue;
                    };
                    let abcd = abc | d.0;

                    for m in l + 1..useable_words_len {
                        let e = useable_words[m];
                        if abcd & e.0 != 0 {
                            continue;
                        };
                        println!(
                            "{:?} [{:.3?}]",
                            vec![a.1, b.1, c.1, d.1, e.1],
                            time.elapsed()
                        );
                    }
                }
            }
        }
    }

    println!("Time: {:.3?}", time.elapsed());
}
