use std::{
    fmt::{Debug, Display},
    ops::{Deref, Range},
    rc::Rc,
};

#[derive(Eq, Hash, PartialEq)]
pub struct SlicableRcString {
    string: Rc<String>,
    span: Range<usize>,
}

impl SlicableRcString {
    pub fn new(string: Rc<String>) -> Self {
        let span = 0..string.len();
        Self { string, span }
    }

    pub fn substr(&self, range: Range<usize>) -> Self {
        Self {
            string: Rc::clone(&self.string),
            span: (self.span.start + range.start)..(self.span.start + range.end),
        }
    }

    pub fn start_index(&self) -> usize {
        self.span.start
    }

    pub fn end_index(&self) -> usize {
        self.span.end
    }

    pub fn len(&self) -> usize {
        self.span.end - self.span.start
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn value(&self) -> String {
        self.string[self.span.clone()].to_string()
    }

    /// ## Get a slice of the full rerefenced string
    pub fn super_slice(&self, span: Range<usize>) -> Self {
        Self {
            string: Rc::clone(&self.string),
            span,
        }
    }
}

impl Deref for SlicableRcString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.string[self.span.clone()]
    }
}

impl Clone for SlicableRcString {
    fn clone(&self) -> Self {
        Self {
            string: Rc::clone(&self.string),
            span: self.span.clone(),
        }
    }
}

impl Display for SlicableRcString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl Debug for SlicableRcString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SRS {{ slice(): \"{0}\", span: {1:?} }}",
            self.value(),
            self.span
        )
    }
}

impl PartialEq<&str> for SlicableRcString {
    fn eq(&self, other: &&str) -> bool {
        self.value() == *other
    }
}
