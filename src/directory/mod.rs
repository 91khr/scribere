/*!
The directories to store extracted codes in files.
*/



use std::io::Write;
use std::path::Path;

use crate::read::SourceCode;



/**
The directory to manipulate files.

Only one file can be opened for appending at a time, the lifetime is to guarantee this.
*/
pub trait Directory {
    /// The writer to write to the file in the directory.
    type Writer<'a>: Write
    where
        Self: 'a;
    /// Errors when opening file for appending or opening the directory for walking.
    type OpenError: std::error::Error;
    /// Open a file at a path relative to the directory for appending.
    fn open_append<'a>(&'a mut self, path: &Path) -> Result<Self::Writer<'a>, Self::OpenError>;
    /// Errors during walking the directory.
    type WalkError: std::error::Error;
    /// The iterator over the files in the directory.
    type DirIter<'a>: Iterator<Item = Result<SourceCode<'a, 'a>, Self::WalkError>>
    where
        Self: 'a;
    /// Get the iterator over the directory content.
    fn walk(&self) -> Result<Self::DirIter<'_>, Self::OpenError>;
}



pub mod dir_iter;

pub mod dummydir;
#[cfg(feature = "dir_tmpdir")]
#[doc(cfg(feature = "dir_tmpdir"))]
pub mod tmpdir;
