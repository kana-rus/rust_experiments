use std::ops::Range;

pub enum Pattern {
    Str(Section),
    Param,
    Nil,
}
pub struct Section {
    pub route_str: &'static str,
    pub range:     Range<usize>,
}

impl Pattern {
    pub fn new(route_str: &'static str, section_range: Range<usize>) -> Self {
        let section = &route_str[section_range.clone()];
        if section.starts_with(':') {
            Self::Param
        } else {
            Self::Str(Section { route_str, range: section_range })
        }
    }

    pub fn is_str(&self) -> bool {
        match self {
            Self::Str(_) => true,
            _ => false,
        }
    }
    pub fn is_param(&self) -> bool {
        match self {
            Self::Param => true,
            _ => false,
        }
    }
    pub fn is_nil(&self) -> bool {
        match self {
            Self::Nil => true,
            _ => false,
        }
    }

    fn get_str(&self) -> Option<&str> {
        match self {
            Self::Param | Self::Nil => None,
            Self::Str(Section { route_str, range }) => Some(&route_str[range.clone()]),
        }
    }
}

const _: (/* Pattern impls */) = {
    impl PartialEq for Pattern {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Self::Nil => other.is_nil(),
                Self::Param => other.is_param(),
                _ => self.get_str() == other.get_str()
            }
        }
    }
};
