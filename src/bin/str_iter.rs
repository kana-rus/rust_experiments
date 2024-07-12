struct StrIter<'a> {
    str:   &'a String,
    index: usize,
}

impl<'a> StrIter<'a> {
    fn of(str: &'a String) -> Self {
        Self { str, index: 0 }
    }
}

struct CharHandle<'a> {
    str:   &'a String,
    index: usize,
}

impl<'a> CharHandle<'a> {
    fn new(str: &'a String, index: usize) -> Self {
        Self { str, index }
    }
}

impl<'a> std::fmt::Display for CharHandle<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.str.chars().nth(self.index).unwrap().fmt(f)
    }
}

impl<'a> Iterator for StrIter<'a> {
    type Item = CharHandle<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.str.len() {
            return None
        }

        let handle: CharHandle<'a> = CharHandle::new(self.str, self.index);

        self.index += 1;

        Some(handle)
    }
}


fn main() {
    let s = String::from("Hello, world!");

    for ch in StrIter::of(&s) {
        println!("{ch}")
    }
}
