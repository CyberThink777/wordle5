#[cfg(test)]
mod representation {

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
}
