#![feature(test)] extern crate test;

const CASES: &[(&[u8], &str)] = &[
    (b"76245dbf96f661bd221046197ab8b9f063f11bad", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n"),
    (b"da39a3ee5e6b4b0d3255bfef95601890afd80709", ""),
    (b"86f7e437faa5a7fce15d1ddcb9eaeaea377667b8", "a"),
    (b"da23614e02469a0d7c7bd1bdab5c9c474b1904dc", "ab"),
    (b"a9993e364706816aba3e25717850c26c9cd0d89d", "abc"),
    (b"81fe8bfe87576c3ecb22426f8e57847382917acf", "abcd"),
    (b"03de6c570bfe24bfc328ccd7ca46b76eadaf4334", "abcde"),
    (b"1f8ac10f23c5b5bc1167bda84b833e5c057a77d2", "abcdef"),
    (b"2fb5e13419fc89246865e7a324f476ec624e8740", "abcdefg"),
    (b"425af12a0743502b322e93a015bcf868e324d56a", "abcdefgh"),
    (b"c63b19f1e4c8b5f76b25c49b8b87f57d8e4872a1", "abcdefghi"),
    (b"d68c19a0a345b7eab78d5e11e991c026ec60db63", "abcdefghij"),
    (b"ebf81ddcbe5bf13aaabdc4d65354fdf2044f38a7", "Discard medicine more than two years old."),
    (b"e5dea09392dd886ca63531aaa00571dc07554bb6", "He who has a shady past knows that nice guys finish last."),
    (b"45988f7234467b94e3e9494434c96ee3609d8f8f", "I wouldn't marry him with a ten foot pole."),
    (b"55dee037eb7460d5a692d1ce11330b260e40c988", "Free! Free!/A trip/to Mars/for 900/empty jars/Burma Shave"),
    (b"b7bc5fb91080c7de6b582ea281f8a396d7c0aee8", "The days of the digital watch are numbered.  -Tom Stoppard"),
    (b"c3aed9358f7c77f523afe86135f06b95b3999797", "Nepal premier won't resign."),
    (b"6e29d302bf6e3a5e4305ff318d983197d6906bb9", "For every action there is an equal and opposite government program."),
    (b"597f6a540010f94c15d71806a99a2c8710e747bd", "His money is twice tainted: 'taint yours and 'taint mine."),
    (b"6859733b2590a8a091cecf50086febc5ceef1e80", "There is no reason for any individual to have a computer in their home. -Ken Olsen, 1977"),
    (b"514b2630ec089b8aee18795fc0cf1f4860cdacad", "It's a tiny change to the code and not completely disgusting. - Bob Manchek"),
    (b"c5ca0d4a7b6676fc7aa72caa41cc3d5df567ed69", "size:  a.out:  bad magic"),
    (b"74c51fa9a04eadc8c1bbeaa7fc442f834b90a00a", "The major problem is with sendmail.  -Mark Horton"),
    (b"0b4c4ce5f52c3ad2821852a8dc00217fa18b8b66", "Give me a rock, paper and scissors and I will move the world.  CCFestoon"),
    (b"3ae7937dd790315beb0f48330e8642237c61550a", "If the enemy is within range, then so are you."),
    (b"410a2b296df92b9a47412b13281df8f830a9f44b", "It's well we cannot hear the screams/That we create in others' dreams."),
    (b"841e7c85ca1adcddbdd0187f1289acb5c642f7f5", "You remind me of a TV show, but that's all right: I watch it anyway."),
    (b"163173b825d03b952601376b25212df66763e1db", "C is as portable as Stonehedge!!"),
    (b"32b0377f2687eb88e22106f133c586ab314d5279", "Even if I could be Shakespeare, I think I should still choose to be Faraday. - A. Huxley"),
    (b"0885aaf99b569542fd165fa44e322718f4a984e0", "The fugacity of a constituent in a mixture of gases at a given temperature is proportional to its mole fraction.  Lewis-Randall Rule"),
    (b"6627d6904d71420b0bf3886ab629623538689f45", "How can you write a big system without C++?  -Paul Glick"),
];
#[inline(always)] fn decode_hex(hex: &[u8]) -> [u8; 20] {
    let hex_bytes = hex;

    std::array::from_fn(|i| i).map(|i|
        [hex_bytes[2*i], hex_bytes[2*i+1]].map(|b| match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'f' => 10 + b - b'a',
            _ => unreachable!()
        }).into_iter().fold(0, |byte, b| byte * 2u8.pow(4) + b)
    )
}


#[bench] fn ohkami_sha1(b: &mut test::Bencher) {
    b.iter(|| 
        for (expected_hex, src) in CASES {
            let expected = decode_hex(expected_hex);
            let actual = {
                let mut s = ohkami::Sha1::new();
                s.write(std::hint::black_box(src.as_bytes()));
                s.sum()
            };

            assert_eq!(expected, actual);
        }
    );
}

#[bench] fn sha1_sha1(b: &mut test::Bencher) {
    use ::sha1::Digest as _;

    b.iter(|| 
        for (expected_hex, src) in CASES {
            let expected = decode_hex(expected_hex);
            let actual = {
                let mut s = <::sha1::Sha1 as ::sha1::Digest>::new();
                s.update(std::hint::black_box(src.as_bytes()));
                s.finalize()
            };

            assert_eq!(expected, actual);
        }
    );
}


mod ohkami {
    const CHANK: usize = 64;
    const SIZE:  usize = 20; // bytes; 160 bits

    #[derive(Debug)]
    pub struct Sha1 {
        h:   [u32; 5],
        x:   [u8; CHANK],
        nx:  usize,
        len: usize,
    }

    const K0: u32 = 0x5A827999;
    const K1: u32 = 0x6ED9EBA1;
    const K2: u32 = 0x8F1BBCDC;
    const K3: u32 = 0xCA62C1D6;

    // https://github.com/golang/go/blob/master/src/crypto/sha1/sha1.go
    impl Sha1 {
        pub const fn new() -> Self {
            Self {
                h:   [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0],
                x:   [0; CHANK],
                nx:  0,
                len: 0,
            }
        }

        #[inline] pub fn write(&mut self, mut p: &[u8]) {
            self.len += p.len();
            if self.nx > 0 {
                let n = (CHANK - self.nx).min(p.len());
                self.x[self.nx..(self.nx + n)].copy_from_slice(&p[..n]);
                self.nx += n;
                if self.nx == CHANK {
                    self.block(&self.x.clone());
                    self.nx = 0;
                }
                p = &p[n..]
            }
            if p.len() >= CHANK {
                let n = p.len() & (!(CHANK - 1));
                self.block(&p[..n]);
                p = &p[n..]
            }
            if p.len() > 0 {
                let n = (self.x.len()).min(p.len());
                self.nx = n;
                self.x[..n].copy_from_slice(&p[..n]);
            }
        }

        #[inline] pub fn sum(mut self) -> [u8; SIZE] {
            let mut len = self.len;

            let mut tmp = [0; 64 + 8];
            tmp[0] = 0x80;
            let t = if len%64 < 56 {
                56 - len%64
            } else {
                64 + 56 - len%64
            };

            len <<= 3;
            tmp[t..(t + 8)].copy_from_slice(&len.to_be_bytes());
            self.write(&tmp[..(t + 8)]);

            #[cfg(debug_assertions)] assert_eq!(self.nx, 0);

            let mut digest = [0; SIZE];
            digest[0..  4].copy_from_slice(&self.h[0].to_be_bytes());
            digest[4..  8].copy_from_slice(&self.h[1].to_be_bytes());
            digest[8.. 12].copy_from_slice(&self.h[2].to_be_bytes());
            digest[12..16].copy_from_slice(&self.h[3].to_be_bytes());
            digest[16..20].copy_from_slice(&self.h[4].to_be_bytes());
            digest
        }
    }

    // https://github.com/golang/go/blob/master/src/crypto/sha1/sha1block.go
    impl Sha1 {
        fn block(&mut self, mut p: &[u8]) {
            fn wrapping_sum(u32_1: u32, u32_2: u32, u32_3: u32, u32_4: u32, u32_5: u32) -> u32 {
                u32_1.wrapping_add(
                    u32_2.wrapping_add(
                        u32_3.wrapping_add(
                            u32_4.wrapping_add(
                                u32_5
                            )
                        )
                    )
                )
            }

            let mut w = [0u32; 16];

            let (mut h0, mut h1, mut h2, mut h3, mut h4) = (self.h[0], self.h[1], self.h[2], self.h[3], self.h[4]);
            while p.len() >= CHANK {
                for i in 0..16 {
                    let j = i * 4;
                    w[i] = (p[j] as u32) << 24 | (p[j+1] as u32) << 16 | (p[j+2] as u32) << 8 | (p[j+3] as u32);
                }

                let (mut a, mut b, mut c, mut d, mut e) = (h0, h1, h2, h3, h4);

                for i in 0..16 {
                    let f = (b & c) | ((!b) & d);
                    let t = wrapping_sum(a.rotate_left(5), f, e, w[i&0xf], K0);
                    (a, b, c, d, e) = (t, a, b.rotate_left(30), c, d)
                }
                for i in 16..20 {
                    let tmp = w[(i-3)&0xf] ^ w[(i-8)&0xf] ^ w[(i-14)&0xf] ^ w[(i)&0xf];
                    w[i&0xf] = tmp.rotate_left(1);

                    let f = (b & c) | ((!b) & d);
    			    let t = wrapping_sum(a.rotate_left(5), f, e, w[i & 0xf], K0);
    			    (a, b, c, d, e) = (t, a, b.rotate_left(30), c, d)
                }
                for i in 20..40 {
                    let tmp = w[(i-3)&0xf] ^ w[(i-8)&0xf] ^ w[(i-14)&0xf] ^ w[(i)&0xf];
    			    w[i&0xf] = tmp.rotate_left(1);

    			    let f = b ^ c ^ d;
    			    let t = wrapping_sum(a.rotate_left(5), f, e, w[i&0xf], K1);
    			    (a, b, c, d, e) = (t, a, b.rotate_left(30), c, d);
                }
                for i in 40..60 {
                    let tmp = w[(i-3)&0xf] ^ w[(i-8)&0xf] ^ w[(i-14)&0xf] ^ w[(i)&0xf];
    			    w[i&0xf] = tmp.rotate_left(1);

    			    let f = ((b | c) & d) | (b & c);
    			    let t = wrapping_sum(a.rotate_left(5), f, e, w[i&0xf], K2);
    			    (a, b, c, d, e) = (t, a, b.rotate_left(30), c, d);
                }
                for i in 60..80 {
                    let tmp = w[(i-3)&0xf] ^ w[(i-8)&0xf] ^ w[(i-14)&0xf] ^ w[(i)&0xf];
    			    w[i&0xf] = tmp.rotate_left(1);

    			    let f = b ^ c ^ d;
    			    let t = wrapping_sum(a.rotate_left(5), f, e, w[i&0xf], K3);
    			    (a, b, c, d, e) = (t, a, b.rotate_left(30), c, d);
                }

                h0 = h0.wrapping_add(a);
                h1 = h1.wrapping_add(b);
                h2 = h2.wrapping_add(c);
                h3 = h3.wrapping_add(d);
                h4 = h4.wrapping_add(e);

                p = &p[CHANK..]
            }

            (self.h[0], self.h[1], self.h[2], self.h[3], self.h[4]) = (h0, h1, h2, h3, h4)
        }    
    }
}
