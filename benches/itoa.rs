#![feature(test)] extern crate test;


mod candiate {#![allow(unused)]
    pub fn to_string(n: usize) -> String {
        n.to_string()
    }

    pub fn itoa_01(mut n: usize) -> String {
        let log10 = match usize::checked_ilog10(n) {
            Some(log10) => log10 as usize,
            None        => return String::from("0")
        };
        let len = 1 + log10;
        let mut digits = vec![0u8; len];
        {
            for i in 0..log10 {
                let d = 10_usize.pow((log10 - i) as u32);
                let (div, rem) = (n / d, n % d);
                *unsafe {digits.get_unchecked_mut(i as usize)} = b'0' + div as u8;
                n = rem;
            }
            *unsafe {digits.get_unchecked_mut(log10)} = b'0' + n as u8;
        }
        unsafe {String::from_utf8_unchecked(digits)}
    }

    #[inline]
    pub fn itoa_02(mut n: usize) -> String {
        let mut buf = Vec::<u8>::with_capacity(1 + usize::ilog10(usize::MAX) as usize);

        {
            macro_rules! push_unchecked {
                ($byte:expr) => {{
                    let len = buf.len();
                    std::ptr::write(buf.as_mut_ptr().add(len), $byte);
                    buf.set_len(len + 1);
                }};
            }

            if n >= 10_usize.pow(1) {
                if n >= 10_usize.pow(2) {
                    if n >= 10_usize.pow(3) {
                        if n >= 10_usize.pow(4) {
                            if n >= 10_usize.pow(5) {
                                if n >= 10_usize.pow(6) {
                                    if n >= 10_usize.pow(7) {
                                        if n >= 10_usize.pow(8) {
                                            if n >= 10_usize.pow(9) {
                                                #[cfg(target_pointer_width="64")]
                                                if n >= 10_usize.pow(10) {
                                                    if n >= 10_usize.pow(11) {
                                                        if n >= 10_usize.pow(12) {
                                                            if n >= 10_usize.pow(13) {
                                                                if n >= 10_usize.pow(14) {
                                                                    if n >= 10_usize.pow(15) {
                                                                        if n >= 10_usize.pow(16) {
                                                                            if n >= 10_usize.pow(17) {
                                                                                if n >= 10_usize.pow(18) {
                                                                                    if n >= 10_usize.pow(19) {
                                                                                        let q = n / 10_usize.pow(19);
                                                                                        unsafe {push_unchecked!(b'0' + q as u8)}
                                                                                        n -= 10_usize.pow(19) * q
                                                                                    }
                                                                                    let q = n / 10_usize.pow(18);
                                                                                    unsafe {push_unchecked!(b'0' + q as u8)}
                                                                                    n -= 10_usize.pow(18) * q
                                                                                }
                                                                                let q = n / 10_usize.pow(17);
                                                                                unsafe {push_unchecked!(b'0' + q as u8)}
                                                                                n -= 10_usize.pow(17) * q
                                                                            }
                                                                            let q = n / 10_usize.pow(16);
                                                                            unsafe {push_unchecked!(b'0' + q as u8)}
                                                                            n -= 10_usize.pow(16) * q
                                                                        }
                                                                        let q = n / 10_usize.pow(15);
                                                                        unsafe {push_unchecked!(b'0' + q as u8)}
                                                                        n -= 10_usize.pow(15) * q
                                                                    }
                                                                    let q = n / 10_usize.pow(14);
                                                                    unsafe {push_unchecked!(b'0' + q as u8)}
                                                                    n -= 10_usize.pow(14) * q
                                                                }
                                                                let q = n / 10_usize.pow(13);
                                                                unsafe {push_unchecked!(b'0' + q as u8)}
                                                                n -= 10_usize.pow(13) * q
                                                            }
                                                            let q = n / 10_usize.pow(12);
                                                            unsafe {push_unchecked!(b'0' + q as u8)}
                                                            n -= 10_usize.pow(12) * q
                                                        }
                                                        let q = n / 10_usize.pow(11);
                                                        unsafe {push_unchecked!(b'0' + q as u8)}
                                                        n -= 10_usize.pow(11) * q
                                                    }
                                                    let q = n / 10_usize.pow(10);
                                                    unsafe {push_unchecked!(b'0' + q as u8)}
                                                    n -= 10_usize.pow(10) * q
                                                }
                                                let q = n / 10_usize.pow(9);
                                                unsafe {push_unchecked!(b'0' + q as u8)}
                                                n -= 10_usize.pow(9) * q
                                            }
                                            let q = n / 10_usize.pow(8);
                                            unsafe {push_unchecked!(b'0' + q as u8)}
                                            n -= 10_usize.pow(8) * q
                                        }
                                        let q = n / 10_usize.pow(7);
                                        unsafe {push_unchecked!(b'0' + q as u8)}
                                        n -= 10_usize.pow(7) * q
                                    }
                                    let q = n / 10_usize.pow(6);
                                    unsafe {push_unchecked!(b'0' + q as u8)}
                                    n -= 10_usize.pow(6) * q
                                }
                                let q = n / 10_usize.pow(5);
                                unsafe {push_unchecked!(b'0' + q as u8)}
                                n -= 10_usize.pow(5) * q
                            }
                            let q = n / 10_usize.pow(4);
                            unsafe {push_unchecked!(b'0' + q as u8)}
                            n -= 10_usize.pow(4) * q
                        }
                        let q = n / 10_usize.pow(3);
                        unsafe {push_unchecked!(b'0' + q as u8)}
                        n -= 10_usize.pow(3) * q
                    }
                    let q = n / 10_usize.pow(2);
                    unsafe {push_unchecked!(b'0' + q as u8)}
                    n -= 10_usize.pow(2) * q
                }
                let q = n / 10_usize.pow(1);
                unsafe {push_unchecked!(b'0' + q as u8)}
                n -= 10_usize.pow(1) * q
            }
            unsafe {push_unchecked!(b'0' + n as u8)}
        }

        unsafe {String::from_utf8_unchecked(buf)}
    }

    #[inline]
    pub fn itoa_03(mut n: usize) -> String {
        const MAX_DIGIT: u32 = 1 + usize::ilog10(usize::MAX);

        let mut buf = Vec::<u8>::with_capacity(MAX_DIGIT as usize);

        {
            #[inline(always)]
            fn digit(d: u32, n: &mut usize, buf: &mut Vec<u8>) {
                macro_rules! push_unchecked {
                    ($byte:expr) => {{
                        let len = buf.len();
                        std::ptr::write(buf.as_mut_ptr().add(len), $byte);
                        buf.set_len(len + 1);
                    }};
                }

                if d < MAX_DIGIT && *n >= 10_usize.pow(d) {
                    digit(d+1, n, buf)
                }

                let power = 10_usize.pow(d-1);
                let q = *n / power;
                unsafe {push_unchecked!(b'0' + q as u8)}
                *n -= power * q
            }

            digit(1, &mut n, &mut buf)
        }

        unsafe {String::from_utf8_unchecked(buf)}
    }
}

fn __() {
    use rand::{SeedableRng, Rng, thread_rng};
    let mut rng = rand::rngs::SmallRng::from_rng(&mut thread_rng());
}

macro_rules! benchmark {
    ($( $target:ident )*) => {$(
        #[bench]
        fn $target(b: &mut test::Bencher) {
            b.iter(|| for n in 10_usize.pow(14)..10_usize.pow(14)+314 {
                //assert_eq!(candiate::$target(test::black_box(n)), n.to_string());
                let _ = candiate::$target(test::black_box(n));
            })
        }
    )*};
} benchmark! {
    //to_string
    //itoa_01
    itoa_02
    itoa_03
}
