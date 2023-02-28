use std::ops::Range;

#[derive(PartialEq, Eq)]
pub enum Pattern {
    Section(Section),
    Param,
    Nil,
}
#[derive(PartialEq, Eq)]
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
            Self::Section(Section { route_str, range: section_range })
        }
    }

    pub fn matches(&self, another: &Self) -> bool {
        match self {
            Self::Nil => unreachable!(),
            Self::Param => true,
            Self::Section(_) => self.get_str() == another.get_str()
        }
    }
    pub fn matches_str(&self, section_str: &str) -> bool {
        match self {
            Self::Nil => unreachable!(),
            Self::Param => true,
            Self::Section(s) => s.read_str() == section_str
        }
    }

    pub fn is_section(&self) -> bool {
        match self {
            Self::Section(_) => true,
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

    fn get_section(&self) -> Option<&Section> {
        match self {
            Self::Nil | Self::Param => None,
            Self::Section(section) => Some(section),
        }
    }
    fn get_section_mut(&mut self) -> Option<&mut Section> {
        match self {
            Self::Nil | Self::Param => None,
            Self::Section(section) => Some(section),
        }
    }
    fn get_str(&self) -> Option<&str> {
        self.get_section().map(|s| s.read_str())
    }

    pub fn merge_sections(&mut self, child_pattern: Self) {
        let Some(s) = self.get_section_mut() else {return};
        let Some(ref c) = child_pattern.get_section() else {return};

        if s.route_str == c.route_str
        && s.range.end == c.range.start {
            s.range.end = c.range.end
        }
    }
}
impl Section {
    pub fn read_str(&self) -> &'static str {
        &self.route_str[self.range.clone()]
    }
}

const _: (/* Pattern impls */) = {
    impl Clone for Pattern {
        fn clone(&self) -> Self {
            match self {
                Self::Nil => Self::Nil,
                Self::Param => Self::Param,
                Self::Section(s) => Self::Section(s.clone()),
            }
        }
    }

    impl Ord for Pattern {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match self {
                Self::Nil => unreachable!(),
                Self::Param => std::cmp::Ordering::Greater,
                Self::Section(s) => s.read_str().cmp(other.get_str().unwrap())
            }
        }
    }
    impl PartialOrd for Pattern {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match self {
                Self::Nil => unreachable!(),
                Self::Param => Some(std::cmp::Ordering::Greater),
                Self::Section(s) => s.read_str().partial_cmp(other.get_str()?),
            }
        }
    }
};
const _: (/* Section impls */) = {
    impl Clone for Section {
        fn clone(&self) -> Self {
            Section { route_str: self.route_str, range: self.range.clone() }
        }
    }
};
