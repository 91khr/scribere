/*!
The source code to be read.
*/

use std::path::Path;

/**
The source to be read, can be a owned or borrowed string, or a path.
*/
#[derive(Debug, Clone, Hash, PartialEq, PartialOrd)]
pub enum SourceCode<'str, 'path> {
    /// A borrowed source code string.
    Borrowed(&'str str),
    /// An owned source code string.
    Owned(String),
    /// Path to the source file.
    File(&'path Path),
}

impl<'a, 'b> SourceCode<'a, 'b> {
    /// Create a `SourceCode` from a string.
    pub const fn from_str(s: &'a str) -> Self {
        Self::Borrowed(s)
    }

    /// Convert self into an owned string, either by copying or reading the file.
    pub fn to_owned_string(&mut self) -> Result<(), std::io::Error> {
        match self {
            Self::Borrowed(s) => {
                *self = s.to_string().into();
                Ok(())
            }
            Self::Owned(_) => Ok(()),
            Self::File(p) => {
                *self = std::fs::read_to_string(p)?.into();
                Ok(())
            }
        }
    }

    /// Get the source string, if self is a path, return `None`.
    pub fn as_str(&'a self) -> Option<&'a str> {
        match self {
            Self::Borrowed(s) => Some(s),
            Self::Owned(s) => Some(s),
            Self::File(_) => None,
        }
    }
}

impl<'str, 'path> From<&'path Path> for SourceCode<'str, 'path> {
    fn from(v: &'path Path) -> Self {
        Self::File(v)
    }
}

impl<'str, 'path> From<String> for SourceCode<'str, 'path> {
    fn from(v: String) -> Self {
        Self::Owned(v)
    }
}

impl<'str, 'path> From<&'str str> for SourceCode<'str, 'path> {
    fn from(v: &'str str) -> Self {
        Self::Borrowed(v)
    }
}
