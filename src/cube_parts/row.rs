use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Row {
    pub l: char,
    pub c: char,
    pub r: char,
}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "|{}|{}|{}|", self.l, self.c, self.r)
    }
}
