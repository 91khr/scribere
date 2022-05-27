/*!
The source code to be read.
*/



use std::borrow::Cow;
use std::path::{Path, PathBuf};



/**
The source to be read, can be a string or a path to the source file, and can be owned or borrowed.

This is a pointer type to the actual data.

`File`s can be converted to `Code`s with [`to_code`], by reading them into the memory.
*/
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SourceCode<'str, 'path> {
    /// Code string.
    Code(Cow<'str, str>),
    /// Path to the source file.
    File(Cow<'path, Path>),
}

impl<'a, 'b> SourceCode<'a, 'b> {
    /// Create a `SourceCode` from a string.
    pub const fn from_code(s: &'a str) -> Self {
        Self::Code(Cow::Borrowed(s))
    }

    /// Convert self into an owned string, either by copying or reading the file.
    pub fn to_code(&mut self) -> Result<(), std::io::Error> {
        if let Self::File(p) = self {
            *self = std::fs::read_to_string(p)?.into();
        }
        Ok(())
    }

    /// Convert self into owned, and extend lifetime.
    pub fn into_owned<'str, 'file>(self) -> SourceCode<'str, 'file> {
        match self {
            Self::Code(x) => SourceCode::Code(Cow::Owned(x.into_owned())),
            Self::File(x) => SourceCode::File(Cow::Owned(x.into_owned())),
        }
    }

    /// Returns the code string if self is a code string, otherwise `None`.
    pub fn as_code(&self) -> Option<&Cow<'a, str>> {
        if let Self::Code(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns the path to source file if self is a source file, otherwise `None`.
    pub fn as_file(&self) -> Option<&Cow<'b, Path>> {
        if let Self::File(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the source code is [`Code`].
    ///
    /// [`Code`]: SourceCode::Code
    #[must_use]
    pub fn is_code(&self) -> bool {
        matches!(self, Self::Code(..))
    }

    /// Returns `true` if the source code is [`File`].
    ///
    /// [`File`]: SourceCode::File
    #[must_use]
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(..))
    }
}

impl<'str, 'path> From<String> for SourceCode<'str, 'path> {
    fn from(v: String) -> Self {
        Self::Code(Cow::from(v))
    }
}

impl<'str, 'path> From<&'str str> for SourceCode<'str, 'path> {
    fn from(v: &'str str) -> Self {
        Self::Code(Cow::from(v))
    }
}

impl<'str, 'path> From<&'path Path> for SourceCode<'str, 'path> {
    fn from(v: &'path Path) -> Self {
        Self::File(Cow::from(v))
    }
}

impl<'str, 'path> From<PathBuf> for SourceCode<'str, 'path> {
    fn from(v: PathBuf) -> Self {
        Self::File(Cow::from(v))
    }
}
