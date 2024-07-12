#![feature(test)] extern crate test;

mod candiate {#![allow(unused)]
    pub fn hexize_1(n: usize) -> String {
        n.to_be_bytes()
            .map(|byte| [byte>>4, byte&(8+4+2+1)]
                .map(|h| match h {
                    0=>"0",1=>"1", 2=>"2", 3=>"3", 4=>"4", 5=>"5", 6=>"6", 7=>"7",
                    8=>"8",9=>"9",10=>"a",11=>"b",12=>"c",13=>"d",14=>"e",15=>"f",
                    _=>unreachable!()
                }).concat()
            ).concat()
    }

    pub fn hexize_2(n: usize) -> String {
        use std::mem::{size_of, transmute};

        unsafe {transmute::<_, [u8; size_of::<usize>() * 2]>(
            n.to_be_bytes().map(|byte| [byte>>4, byte&(8+4+2+1)])
        )}.map(|h| match h {
            0=>"0",1=>"1", 2=>"2", 3=>"3", 4=>"4", 5=>"5", 6=>"6", 7=>"7",
            8=>"8",9=>"9",10=>"a",11=>"b",12=>"c",13=>"d",14=>"e",15=>"f",
            _=>unreachable!()
        }).concat()
    }

    pub fn hexize_3(n: usize) -> String {
        use std::mem::{size_of, transmute};

        unsafe {transmute::<_, [u8; size_of::<usize>() * 2]>(
            n.to_be_bytes().map(|byte| [byte>>4, byte&(8+4+2+1)])
        )}.map(|h| match h {
            0=>'0',1=>'1', 2=>'2', 3=>'3', 4=>'4', 5=>'5', 6=>'6', 7=>'7',
            8=>'8',9=>'9',10=>'a',11=>'b',12=>'c',13=>'d',14=>'e',15=>'f',
            _=>unreachable!()
        }).into_iter().collect()
    }

    pub fn hexize_4(n: usize) -> String {
        use std::mem::{size_of, transmute};

        let mut hex = String::with_capacity(size_of::<usize>() * 2);
        for ch in unsafe {transmute::<_, [u8; size_of::<usize>() * 2]>(
            n.to_be_bytes().map(|byte| [byte>>4, byte&(8+4+2+1)])
        )}.map(|h| match h {
            0=>'0',1=>'1', 2=>'2', 3=>'3', 4=>'4', 5=>'5', 6=>'6', 7=>'7',
            8=>'8',9=>'9',10=>'a',11=>'b',12=>'c',13=>'d',14=>'e',15=>'f',
            _=>unreachable!()
        }) {
            hex.push(ch)
        }
        hex
    }

    pub fn hexize_5(n: usize) -> String {
        use std::mem::{size_of, transmute};

        let mut hex = String::with_capacity(size_of::<usize>() * 2);
        for ch in unsafe {transmute::<_, [u8; size_of::<usize>() * 2]>(
            n.to_be_bytes().map(|byte| [byte>>4, byte&(8+4+2+1)])
        )}.map(|h| match h {
            0=>'0',1=>'1', 2=>'2', 3=>'3', 4=>'4', 5=>'5', 6=>'6', 7=>'7',
            8=>'8',9=>'9',10=>'a',11=>'b',12=>'c',13=>'d',14=>'e',15=>'f',
            _ => unsafe {std::hint::unreachable_unchecked()}
        }) {
            unsafe {hex.as_mut_vec().push(ch as u8)}
        }
        hex
    }

    pub fn hexize_6(n: usize) -> String {
        use std::mem::{size_of, transmute};

        let mut hex: Vec<u8> = Vec::with_capacity(size_of::<usize>() * 2);
        for ch in unsafe {transmute::<_, [u8; size_of::<usize>() * 2]>(
            n.to_be_bytes().map(|byte| [byte>>4, byte&(8+4+2+1)])
        )}.map(|h| match h {
            0=>'0',1=>'1', 2=>'2', 3=>'3', 4=>'4', 5=>'5', 6=>'6', 7=>'7',
            8=>'8',9=>'9',10=>'a',11=>'b',12=>'c',13=>'d',14=>'e',15=>'f',
            _ => unsafe {std::hint::unreachable_unchecked()}
        }) {
            unsafe {
                let len = hex.len();
                std::ptr::write(hex.as_mut_ptr().add(len), ch as u8);
                hex.set_len(len + 1);
            }
        }
        unsafe {String::from_utf8_unchecked(hex)}
    }

    pub fn hexize_7(n: usize) -> String {
        use std::mem::{size_of, transmute};

        let mut hex = String::with_capacity(size_of::<usize>() * 2);
        for ch in unsafe {transmute::<_, [u8; size_of::<usize>() * 2]>(
            n.to_be_bytes().map(|byte| [byte>>4, byte&(8+4+2+1)])
        )}.map(|h| match h {
            0=>'0',1=>'1', 2=>'2', 3=>'3', 4=>'4', 5=>'5', 6=>'6', 7=>'7',
            8=>'8',9=>'9',10=>'a',11=>'b',12=>'c',13=>'d',14=>'e',15=>'f',
            _ => unsafe {std::hint::unreachable_unchecked()}
        }) {
            unsafe {let hex = hex.as_mut_vec();
                let len = hex.len();
                std::ptr::write(hex.as_mut_ptr().add(len), ch as u8);
                hex.set_len(len + 1);
            }
        }
        hex
    }

    pub fn hexize_8(n: usize) -> String {
        use std::mem::{size_of, transmute};

        let mut hex = String::with_capacity(size_of::<usize>() * 2);
        for ch_byte in unsafe {transmute::<_, [u8; size_of::<usize>() * 2]>(
            n.to_be_bytes().map(|byte| [byte>>4, byte&(8+4+2+1)])
        )}.map(|h| match h {
            0=>b'0',1=>b'1', 2=>b'2', 3=>b'3', 4=>b'4', 5=>b'5', 6=>b'6', 7=>b'7',
            8=>b'8',9=>b'9',10=>b'a',11=>b'b',12=>b'c',13=>b'd',14=>b'e',15=>b'f',
            _ => unsafe {std::hint::unreachable_unchecked()}
        }) {
            unsafe {let hex = hex.as_mut_vec();
                let len = hex.len();
                std::ptr::write(hex.as_mut_ptr().add(len), ch_byte);
                hex.set_len(len + 1);
            }
        }
        hex
    }

    pub fn hexize_9(n: usize) -> String {
        use std::mem::{size_of, transmute};

        let mut hex = String::with_capacity(size_of::<usize>() * 2);
        for ch_byte in unsafe {transmute::<_, [u8; size_of::<usize>() * 2]>(
            n.to_be_bytes().map(|byte| [byte>>4, byte&(8+4+2+1)])
        )}.map(|h| h + match h {
            0..=9  => b'0'-0,
            10..=15=> b'a'-10,
            _ => unsafe {std::hint::unreachable_unchecked()}
        }) {
            unsafe {let hex = hex.as_mut_vec();
                let len = hex.len();
                std::ptr::write(hex.as_mut_ptr().add(len), ch_byte);
                hex.set_len(len + 1);
            }
        }
        hex
    }

    pub fn hexize_9_bytes(n: usize) -> Vec<u8> {
        use std::mem::{size_of, transmute};

        unsafe {transmute::<_, [u8; size_of::<usize>() * 2]>(
            n.to_be_bytes().map(|byte| [byte>>4, byte&(8+4+2+1)])
        )}.map(|h| h + match h {
            0..=9  => b'0'-0,
            10..=15=> b'a'-10,
            _ => unsafe {std::hint::unreachable_unchecked()}
        }).into()
    }

    pub fn hexize_10(n: usize) -> String {
        use std::mem::{size_of, transmute};

        unsafe {
            String::from_utf8_unchecked(
                transmute::<_, [u8; size_of::<usize>() * 2]>(
                    n.to_be_bytes().map(|byte| [byte>>4, byte&(8+4+2+1)])
                ).map(|h| h + match h {
                    0..=9  => b'0'-0,
                    10..=15=> b'a'-10,
                    _ => unsafe {std::hint::unreachable_unchecked()}
                }).into()
            )
        }
    }
}

macro_rules! benchmark {
    ($( $target:ident )*) => {$(
        #[bench]
        fn $target(b: &mut test::Bencher) {
            b.iter(|| for n in 0..314 {
                let _ = candiate::$target(test::black_box(n));
            })
        }
    )*};
} benchmark! {
    hexize_1
    hexize_2
    hexize_3
    hexize_4
    hexize_5
    hexize_6
    hexize_7
    hexize_8
    hexize_9
    hexize_9_bytes
    hexize_10
}
