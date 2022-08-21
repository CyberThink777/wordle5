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
    assert_eq!(crate::representation::decode(0), "");
    assert_eq!(
        crate::representation::decode(0x3FFFFFF_u32),
        "abcdefghijklmnopqrstuvwxyz"
    );
    assert_eq!(crate::representation::decode(0x1_u32), "zzzzz");
    assert_eq!(crate::representation::decode(0x2003040_u32), "manta");
    assert_eq!(crate::representation::decode(0x510900_u32), "fjord");
}
