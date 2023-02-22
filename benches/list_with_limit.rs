#![feature(test)]
extern crate test;
use test::Bencher;

use setup::{Element, LIMIT, SEARCH_KEYS};
use targets::{ListFromVec, ListFromArray};

pub trait List {
    fn new() -> Self;
    fn push(&mut self, new: Element) -> Result<(), String>;
    fn search(&self, target_key: &str) -> Option<&Element>;
}

mod setup {
    pub struct Element {
        pub key:   Pattern,
        pub value: fn(String) -> String,
    } impl Element {
        pub fn new(i: usize) -> Self {
            Self {
                key:   Pattern::new(&format!("element{}{}{}", i, i+1, i+2)),
                value: sample,
            }
        }
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
    pub enum Pattern {
        Param,
        Str(String),
    } impl Pattern {
        pub fn new(string: &str) -> Self {
            if string.starts_with(':') {
                Self::Param
            } else {
                Self::Str(string.to_owned())
            }
        }
        pub fn matches(&self, ref_str: &str) -> bool {
            match self {
                Self::Param  => ref_str.starts_with(':'),
                Self::Str(s) => s == ref_str
            }
        }
    }

    pub const LIMIT: usize = 8; //16; //32;
    pub const fn sample(s: String) -> String {s}

    pub const SEARCH_KEYS: &'static [&'static str] = &[
        "dummy",
        ":param",
        "element0120",
        "element012",
        "element000",
        "element345",
        ":param",
        "element789",
        "element101112",
        ":param",
        "element?",
        ":param",
        "element202122",
        "element262728",
        "element101112?",
        ":param",
    ];
}

mod targets {
    use crate::{List, setup::{Element, LIMIT, Pattern}};

    pub struct ListFromVec(
        Vec<Element>
    ); impl List for ListFromVec {
        fn new() -> Self {
            Self(Vec::with_capacity(LIMIT))
        }
        fn push(&mut self, new: Element) -> Result<(), String> {
            self.0.push(new);
            self.0.sort_unstable_by_key(|e| e.key.clone());
            Ok(())
        }
        fn search(&self, target_key: &str) -> Option<&Element> {
            let i = self.0.binary_search_by_key(
                &&Pattern::new(target_key), |e| &e.key)
                .ok()?;
            Some(&self.0[i])
        }
    }

    pub struct ListFromArray {
        list:     [Option<Element>; LIMIT],
        next_pos: usize,
    } impl List for ListFromArray {
        fn new() -> Self {
            Self {
                next_pos: 0,
                list: [
                    None, None, None, None, None, None, None, None,
                    // None, None, None, None, None, None, None, None,
                    // None, None, None, None, None, None, None, None,
                    // None, None, None, None, None, None, None, None,
                ],
            }
        }
        fn push(&mut self, new: Element) -> Result<(), String> {
            if self.next_pos == LIMIT {return Err(format!("no space for the new"))}

            self.list[self.next_pos].replace(new);
            self.next_pos += 1;

            Ok(())
        }
        fn search(&self, target_key: &str) -> Option<&Element> {
            for option in &self.list[..self.next_pos] {
                match option {
                    Some(element) => if element.key.matches(target_key) {return option.as_ref()},
                    _ => (),
                }
            }
            None
        }
    }
}

#[bench]
fn list_from_vec(b: &mut Bencher) {
    let mut list = ListFromVec::new();
    for i in 0..LIMIT {
        match list.push(Element::new(i)) {
            Ok(_) => (),
            Err(msg) => panic!("{msg}"),
        }
    }

    b.iter(|| for _ in 0..100 {
        for case in SEARCH_KEYS {
            if let Some(e) = list.search(case) {
                println!("found: {:?}", e.key)
            }
        }
    })
}
#[bench]
fn list_from_array(b: &mut Bencher) {
    let mut list = ListFromArray::new();
    for i in 0..LIMIT {
        match list.push(Element::new(i)) {
            Ok(_) => (),
            Err(msg) => panic!("{msg}"),
        }
    }

    b.iter(|| for _ in 0..100 {
        for case in SEARCH_KEYS {
            if let Some(e) = list.search(case) {
                println!("found: {:?}", e.key)
            }
        }
    })
}
