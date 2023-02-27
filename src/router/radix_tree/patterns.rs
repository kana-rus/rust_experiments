pub struct Patterns(
    Vec<Pattern>
);
pub enum Pattern {
    Str(String),
    Param,
    Nil,
}

impl Patterns {
    pub fn root() -> Self {
        Self(vec![Pattern::Nil])
    }

    // fn append(&mut self, child: Self) {
    //     if self.tail().is_str()
    //     && child.head().is_str() {
    //         self.tail_mut()
    //             .
    //                 /*
    //                     tail_str += head_str;
    //                 */
    //                 /*
    //                     for p in rem_child {
    //                         self.push(p)
    //                     }
    //                 */
    //             )
    //     }
    // }

    fn head(&self) -> &Pattern {
        self.0.first().unwrap()
    }
    fn tail(&self) -> &Pattern {
        self.0.last().unwrap()
    }
    fn tail_mut(&mut self) -> &mut Pattern {
        self.0.last_mut().unwrap()
    }
    fn pop(&mut self) -> Pattern {
        self.0.remove(0)
    }
}
impl IntoIterator for Patterns {
    type Item = <Vec<Pattern> as IntoIterator>::Item;
    type IntoIter = <Vec<Pattern> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {self.0.into_iter()}
}
impl Pattern {
    fn is_str(&self) -> bool {
        match self {
            Self::Str(_) => true,
            _            => false,
        }
    }
    fn is_param(&self) -> bool {
        match self {
            Self::Param  => true,
            _            => false,
        }
    }
}

