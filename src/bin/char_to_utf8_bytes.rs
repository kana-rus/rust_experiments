/// ref: `encode_utf8_raw` in core/src/char/method.rs
fn to_utf8_bytes(c: char) -> Vec<u8> {
    let code = c as u32;

    const TAG_TWO_B:   u8 = 0xC0;
    const TAG_CONT:    u8 = 0x80;
    const TAG_THREE_B: u8 = 0xE0;
    const TAG_FOUR_B:  u8 = 0xF0;

    match c.len_utf8() {
        1 => vec![
            code as u8,
        ],
        2 => vec![
            (code >> 6 & 0x1F) as u8 | TAG_TWO_B,
            (code & 0x3F) as u8 | TAG_CONT,
        ],
        3 => vec![
            (code >> 12 & 0x0F) as u8 | TAG_THREE_B,
            (code >> 6 & 0x3F) as u8 | TAG_CONT,
            (code & 0x3F) as u8 | TAG_CONT,
        ],
        4 => vec![
            (code >> 18 & 0x07) as u8 | TAG_FOUR_B,
            (code >> 12 & 0x3F) as u8 | TAG_CONT,
            (code >> 6 & 0x3F) as u8 | TAG_CONT,
            (code & 0x3F) as u8 | TAG_CONT,
        ],
        _ => unsafe {std::hint::unreachable_unchecked()}
    }
}

fn main() {
    println!("{:?}", "a".as_bytes());
    println!("{:?}", to_utf8_bytes('a'));

    println!("{:?}", "あ".as_bytes());
    println!("{:?}", to_utf8_bytes('あ'));

    println!("{:?}", "錆".as_bytes());
    println!("{:?}", to_utf8_bytes('錆'));
}