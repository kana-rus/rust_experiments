#![feature(test)]
extern crate test;
use test::Bencher;

const CASES: &[(&str, (usize, usize), &str)] = &[
    (
        "fuga",
        (0, 2),
        "gufa"
    ),
    (
        "Rustでは文字列に対してUTF-8(Unicodeというべきか)を基準としているようです。",
        (3, 4),
        "Rusでtは文字列に対してUTF-8(Unicodeというべきか)を基準としているようです。"
    ),
    (
        "バイト単位で考えたいのであれば、 as_bytes()を使って強制的にバイト単位にぶった切るとかしないといけないみたいです。",
        (15, 19),
        "バイト単位で考えたいのであれば_ as、bytes()を使って強制的にバイト単位にぶった切るとかしないといけないみたいです。"
    ),
    (
        "なお日本語はUTF-8では基本3バイトのようです。",
        (5, 9),
        "なお日本語-UTFは8では基本3バイトのようです。"
    ),
    (
        "文字列をベクタに変換する → ベクタならn番目がざっくり取れる",
        (2, 21),
        "文字番をベクタに変換する → ベクタならn列目がざっくり取れる"
    ),
    (
        "書き換え可能なものにする → 入れ替えてOK",
        (2, 17),
        "書き替え可能なものにする → 入れ換えてOK"
    ),
    (
        "最後にベクタを文字列に変換する",
        (5, 7),
        "最後にベク文をタ字列に変換する"
    ),
];

// m < n を仮定する.
mod swappers {
    use std::cmp::Ordering;

    pub fn swap_chars_0(string: &mut String, m: usize, n: usize) {
        let mut chars = string.chars().collect::<Vec<_>>();
        (chars[m], chars[n]) = (chars[n], chars[m]);
        *string = chars.into_iter().collect();
    }

    pub fn swap_chars_1(string: &mut String, m: usize, n: usize) {
        let (pos_m, char_m) = string.char_indices().nth(m).unwrap();
        let (pos_n, char_n) = string.char_indices().nth(n).unwrap();

        string.replace_range(pos_n..pos_n+char_n.len_utf8(), &char_m.to_string());
        string.replace_range(pos_m..pos_m+char_m.len_utf8(), &char_n.to_string());
    }

    pub fn swap_chars_2(string: &mut String, m: usize, n: usize) {
        let (pos_m, char_m) = string.char_indices().nth(m).unwrap();
        let (pos_n, char_n) = string.char_indices().nth(n).unwrap();

        string.remove(pos_n);
        string.insert(pos_n, char_m);
        string.remove(pos_m);
        string.insert(pos_m, char_n);
    }

    pub fn swap_chars_3(string: &mut String, m: usize, n: usize) {
        let (pos_m, char_m) = string.char_indices().nth(m).unwrap();
        let (pos_n, char_n) = string.char_indices().nth(n).unwrap();
        let (len_m, len_n) = (char_m.len_utf8(), char_n.len_utf8());
        
        let bytes = unsafe {string.as_mut_vec()};

        match len_m.cmp(&len_n) {
            Ordering::Greater => {
                for i in 0..len_n {
                    bytes.swap(pos_m+i, pos_n+i)
                }
                for _ in len_n..len_m {
                    let b = bytes.remove(pos_m+len_n);
                    bytes.insert(pos_n+len_n-1, b);
                }
            }
            Ordering::Equal => {
                for i in 0..len_n {
                    bytes.swap(pos_m+i, pos_n+i)
                }
            }
            Ordering::Less => {
                for i in 0..len_m {
                    bytes.swap(pos_m+i, pos_n+i)
                }
                for i in len_m..len_n {
                    let b = bytes.remove(pos_n+i);
                    bytes.insert(pos_m+i, b);
                }
            }
        }
    }

    pub fn swap_chars_3_v2(string: &mut String, m: usize, n: usize) {
        let (pos_m, char_m) = string.char_indices().nth(m).unwrap();
        let (pos_n, char_n) = string.char_indices().nth(n).unwrap();
        let (len_m, len_n) = (char_m.len_utf8(), char_n.len_utf8());
        
        let bytes = unsafe {string.as_mut_vec()};

        for i in 0..len_m.min(len_n) {
            bytes.swap(pos_m+i, pos_n+i)
        }

        if len_m > len_n {
            for _ in len_n..len_m {
                let b = bytes.remove(pos_m+len_n);
                bytes.insert(pos_n+len_n-1, b);
            }
        } else if len_m < len_n {
            for i in len_m..len_n {
                let b = bytes.remove(pos_n+i);
                bytes.insert(pos_m+i, b);
            }
        }
    }

    pub fn swap_chars_3_v3(string: &mut String, m: usize, n: usize) {
        let (pos_m, char_m) = string.char_indices().nth(m).unwrap();
        let (pos_n, char_n) = string.char_indices().nth(n).unwrap();
        let (len_m, len_n) = (char_m.len_utf8(), char_n.len_utf8());
        
        let bytes = unsafe {string.as_mut_vec()};

        for i in 0..len_m.min(len_n) {
            bytes.swap(pos_m+i, pos_n+i)
        }

        if len_m > len_n {
            bytes[(pos_m+len_n)..(pos_n+len_n)].rotate_left(len_m-len_n)
        } else if len_m < len_n {
            bytes[(pos_m+len_m)..(pos_n+len_n)].rotate_right(len_n-len_m)
        }
    }

    pub fn swap_chars_4(string: &mut String, m: usize, n: usize) {
        let (pos_m, char_m) = string.char_indices().nth(m).unwrap();
        let (pos_n, char_n) = string.char_indices().nth(n).unwrap();
        let (len_m, len_n) = (char_m.len_utf8(), char_n.len_utf8());
        
        let bytes = unsafe {string.as_mut_vec()};
        let (left, bytes_m, mid, bytes_n, right) = (
            &bytes[..pos_m],
            &bytes[pos_m..(pos_m+len_m)],
            &bytes[(pos_m+len_m)..pos_n],
            &bytes[pos_n..(pos_n+len_n)],
            &bytes[(pos_n+len_n)..],
        );
        *bytes = [left, bytes_n, mid, bytes_m, right].concat()
    }

    pub fn swap_chars_5_left(string: &mut String, m: usize, n: usize) {
        let (pos_m, char_m) = string.char_indices().nth(m).unwrap();
        let (pos_n, char_n) = string.char_indices().nth(n).unwrap();
        let (len_m, len_n) = (char_m.len_utf8(), char_n.len_utf8());
        
        let bytes = unsafe {string.as_mut_vec()};
        bytes[pos_m..(pos_n+len_n)].rotate_left(len_m);
        bytes[pos_m..(pos_n+len_n-len_m)].rotate_right(len_n);
    }

    pub fn swap_chars_5_right(string: &mut String, m: usize, n: usize) {
        let (pos_m, char_m) = string.char_indices().nth(m).unwrap();
        let (pos_n, char_n) = string.char_indices().nth(n).unwrap();
        let (len_m, len_n) = (char_m.len_utf8(), char_n.len_utf8());
        
        let bytes = unsafe {string.as_mut_vec()};
        bytes[pos_m..(pos_n+len_n)].rotate_right(len_n);
        bytes[(pos_m+len_n)..(pos_n+len_n)].rotate_left(len_m);
    }

    pub fn swap_chars_7(string: &mut String, m: usize, n: usize) {
        let (pos_m, char_m) = string.char_indices().nth(m).unwrap();
        let (pos_n, char_n) = string.char_indices().nth(n).unwrap();
        let (len_m, len_n) = (char_m.len_utf8(), char_n.len_utf8());
        
        let bytes = unsafe {string.as_mut_vec()};

        let len = len_m.min(len_n);
        let (left, right) = bytes.split_at_mut(pos_n);
        left[pos_m..(pos_m+len)].swap_with_slice(&mut right[..len]);

        if len_m > len_n {
            bytes[(pos_m+len_n)..(pos_n+len_n)].rotate_left(len_m-len_n)
        } else if len_m < len_n {
            bytes[(pos_m+len_m)..(pos_n+len_n)].rotate_right(len_n-len_m)
        }
    }

    pub fn swap_chars_7_v2(string: &mut String, m: usize, n: usize) {
        let (pos_m, char_m) = string.char_indices().nth(m).unwrap();
        let (pos_n, char_n) = string.char_indices().nth(n).unwrap();
        let (len_m, len_n) = (char_m.len_utf8(), char_n.len_utf8());
        
        let mut bytes = std::mem::take(string).into_bytes();

        let len = len_m.min(len_n);
        let (left, right) = bytes.split_at_mut(pos_n);
        left[pos_m..(pos_m+len)].swap_with_slice(&mut right[..len]);

        if len_m > len_n {
            bytes[(pos_m+len_n)..(pos_n+len_n)].rotate_left(len_m-len_n)
        } else if len_m < len_n {
            bytes[(pos_m+len_m)..(pos_n+len_n)].rotate_right(len_n-len_m)
        }

        *string = String::from_utf8(bytes).unwrap()
    }
}

macro_rules! benchmark {
    ($($target:ident)*) => {
        $(
            #[bench]
            fn $target(b: &mut Bencher) {
                b.iter(|| for (case, (m, n), expected) in CASES {
                    let mut case = case.to_string();
                    swappers::$target(&mut case, *m, *n);
                    assert_eq!(&case, expected)
                })
            }
        )*
    };
} benchmark! {
    // swap_chars_0
    // swap_chars_1
    // swap_chars_2
    // swap_chars_3
    // swap_chars_3_v2
    swap_chars_3_v3
    // swap_chars_4
    // swap_chars_5_left
    // swap_chars_5_right
    swap_chars_7
    swap_chars_7_v2
}
