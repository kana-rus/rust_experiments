use std::ops::BitXor;
use fxhash::hash32;

const CASES: &'static [&'static str] = &[
    "a",
    "abc",
    "hash",
    "fxhash",
    "src/bin/debuginfo.rs",
];

fn main() {
    for case in CASES {
        assert_eq!(
            hash32(case),
            my_hash(case),
            "in {case:?}"
        );
    }
}

fn my_hash(s: &str) -> u32 {
    let mut hash: u32 = 0;
    let mut bytes = s.as_bytes();

    // write()
    while bytes.len() >= 4 {
        hash_word(&mut hash, u32::from_le_bytes(bytes[..4].try_into().unwrap()));
        // bytes = &bytes[4..]
        bytes = bytes.split_at(4).1
    }
    for b in bytes {
        hash_word(&mut hash, *b as u32)
    }

    // write_u8
    hash_word(&mut hash, 0xff);

    hash
}
#[inline] fn hash_word(hash: &mut u32, word: u32) {
    *hash = hash.rotate_left(5).bitxor(word).wrapping_mul(0x27220A95)
}
