/*!
The source code to be read.
*/

use std::path::{Path, PathBuf};

/**
The source to be read, can be a owned or borrowed string, or a path.
*/
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SourceCode<'str, 'path> {
    /// A borrowed source code string.
    Borrowed(&'str str),
    /// An owned source code string.
    Owned(String),
    /// Path to the source file.
    File(&'path Path),
    /// Owned path to the source file.
    OwnedFile(PathBuf),
}

impl<'a, 'b> SourceCode<'a, 'b> {
    /// Create a `SourceCode` from a string.
    pub const fn from_str(s: &'a str) -> Self {
        Self::Borrowed(s)
    }

    /// Convert self into an owned string, either by copying or reading the file.
    pub fn try_into_owned(&mut self) -> Result<(), std::io::Error> {
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
            Self::OwnedFile(p) => {
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
            Self::File(_) | Self::OwnedFile(_) => None,
        }
    }

    /// Returns the owned path if the source code is, otherwise `None`.
    pub fn as_owned_file(&self) -> Option<&PathBuf> {
        if let Self::OwnedFile(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns the borrowed path if the source code is, otherwise `None`.
    pub fn as_file(&self) -> Option<&&'b Path> {
        if let Self::File(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns the owned string if the source code is, otherwise `None`.
    pub fn as_owned(&self) -> Option<&String> {
        if let Self::Owned(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns the borrowed string if the source code is, otherwise `None`.
    pub fn as_borrowed(&self) -> Option<&&'a str> {
        if let Self::Borrowed(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the source code is [`Borrowed`].
    ///
    /// [`Borrowed`]: SourceCode::Borrowed
    #[must_use]
    pub fn is_borrowed(&self) -> bool {
        matches!(self, Self::Borrowed(..))
    }

    /// Returns `true` if the source code is [`Owned`].
    ///
    /// [`Owned`]: SourceCode::Owned
    #[must_use]
    pub fn is_owned(&self) -> bool {
        matches!(self, Self::Owned(..))
    }

    /// Returns `true` if the source code is [`File`].
    ///
    /// [`File`]: SourceCode::File
    #[must_use]
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(..))
    }

    /// Returns `true` if the source code is [`OwnedFile`].
    ///
    /// [`OwnedFile`]: SourceCode::OwnedFile
    #[must_use]
    pub fn is_owned_file(&self) -> bool {
        matches!(self, Self::OwnedFile(..))
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

impl<'str, 'path> From<PathBuf> for SourceCode<'str, 'path> {
    fn from(v: PathBuf) -> Self {
        Self::OwnedFile(v)
    }
}
