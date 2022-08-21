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

    pub fn decode(bits: u32) -> String {
        let mut chars: Vec<char> = vec![];

        for i in 1..=26 {
            let cur_bit_set = bits >> (i - 1) & 1;
            if cur_bit_set == 1 {
                if let Some(c) = char::from_u32((27 - i) + 96) {
                    chars.push(c);
                };
            }
        }

        String::from_iter(chars)
    }
}

fn main() {}
