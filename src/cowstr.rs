/*!
A copy-on-write string that can be owned or borrowed.
*/

use std::ops::Deref;



/**
A copy-on-write string that can be owned or borrowed.
*/
#[derive(Debug, Hash, Clone)]
pub enum CowStr<'a> {
    /// A borrowed string.
    Borrowed(&'a str),
    /// An owned string.
    Owned(String),
}

impl Deref for CowStr<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(s) => s,
            Self::Owned(s) => s,
        }
    }
}

impl<'a> From<&'a str> for CowStr<'a> {
    fn from(s: &'a str) -> Self {
        Self::Borrowed(s)
    }
}

impl From<String> for CowStr<'_> {
    fn from(s: String) -> Self {
        Self::Owned(s)
    }
}

impl PartialEq for CowStr<'_> {
    fn eq(&self, other: &Self) -> bool {
        self as &str == other as &str
    }
}
impl Eq for CowStr<'_> {}

impl PartialOrd for CowStr<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self as &str).partial_cmp(other as &str)
    }
}
impl Ord for CowStr<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self as &str).cmp(other as &str)
    }
}
