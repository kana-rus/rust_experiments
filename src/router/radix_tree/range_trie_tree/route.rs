use std::{ops::Range, collections::vec_deque::VecDeque};
use crate::router::radix_tree::range_trie_tree::pattern::Pattern;

pub struct Route {
    route_str:      &'static str,
    section_ranges: Ranges,
}
struct Ranges(
    <VecDeque<Range<usize>> as IntoIterator>::IntoIter
);

impl Route {
    pub fn new(route_str: &'static str) -> Self {
        Self {
            route_str,
            section_ranges: Ranges::new(route_str)
        }
    }
    // fn overlaps_with(&self, another: &Self) -> bool {
    //     let this = self.clone().collect::<Vec<_>>();
    //     let another = another.clone().collect::<Vec<_>>();
// 
    //     let n_sections = this.len();
    //     if another.len() != n_sections {return false}
    //     for i in 0..n_sections {
    //         if this[i].is_str()
    //         && another[i].is_str()
    //         && this[i] != another[i] {
    //             return false
    //         }
    //     }
// 
    //     true
    // }
}
impl Ranges {
    fn new(route_str: &'static str) -> Self {
        Self(match route_str {
            "/" => VecDeque::new().into_iter(),
             _  => {
                let (mut queue, mut read_pos) = (VecDeque::new(), 0);
                let split = route_str
                    .trim_start_matches('/')
                    .trim_end_matches('/')
                    .split('/');
                for section in split {
                    let len = section.len();
                    queue.push_front(Range {
                        start: read_pos + 1/* '/' */,
                        end:   read_pos + len/* section */,
                    });
                    read_pos += 1/* '/' */ + len/* section */
                }
                queue.into_iter()
            },
        })
    }
}

const _: (/* Route impls */) = {
    impl Iterator for Route {
        type Item = Pattern;
        fn next(&mut self) -> Option<Self::Item> {
            self.section_ranges.next().map(|range| Pattern::new(self.route_str, range))
        }
    }
    impl Clone for Route {
        fn clone(&self) -> Self {
            Self {
                route_str: self.route_str,
                section_ranges:  self.section_ranges.clone(),
            }
        }
    }
    impl std::fmt::Display for Route {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.route_str.fmt(f)
        }
    }
};
const _: (/* Ranges impls */) = {
    impl Iterator for Ranges {
        type Item = Range<usize>;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }
    impl Clone for Ranges {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
};
