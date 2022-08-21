/*
Matt Parker
Can you find: five five-letter words with twenty-five unique letters?
FJORD
GUCKS
NYMPH
VIBEX
WALTZ
Q
Constraints:
- No duplicate letters (valid words have 5 unique characters)
- Order of letters irrelevant (ignore anagrams during search)
- i.e. Word is Set of Characters
*/

mod tests;
/// 32-bit representation of word
/// This would exclude anagrams and duplicate char
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

    pub fn decode(bin: u32) -> String {
        "TEST".to_string()
    }
}

fn main() {
    println!("------ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    println!("{:032b}, {}", representation::encode("testing"), "testing");
}
