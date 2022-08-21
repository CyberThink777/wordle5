#[cfg(test)]
mod tests {

    #[test]
    fn encode() {
        assert_eq!(crate::representation::encode(""), 0);
        assert_eq!(
            crate::representation::encode("abcdefghijklmnopqrstuvwxyz"),
            0x3FFFFFF_u32
        );
        assert_eq!(crate::representation::encode("zzzzz"), 0x1_u32);
        assert_eq!(crate::representation::encode("manta"), 0x2003040_u32);
        assert_eq!(crate::representation::encode("fjord"), 0x510900_u32);
    }

    #[test]
    fn decode() {
        assert!(str_cmp("", crate::representation::decode(0)));
        assert!(str_cmp(
            "abcdefghijklmnopqrstuvwxyz",
            crate::representation::decode(0x3FFFFFF_u32)
        ));
        assert!(str_cmp("zzzzz", crate::representation::decode(0x1_u32)));
        assert!(str_cmp(
            "manta",
            crate::representation::decode(0x2003040_u32)
        ));
        assert!(str_cmp(
            "fjord",
            crate::representation::decode(0x510900_u32)
        ));
    }

    fn str_cmp(words: &str, bits: String) -> bool {
        for c in bits.chars() {
            if !words.contains(c) {
                return false;
            };
        }
        true
    }
}
