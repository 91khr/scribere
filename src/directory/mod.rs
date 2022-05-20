/*!
The directories to store extracted codes in files.
*/



use std::io::Write;
use std::path::Path;



/**
The directory to manipulate files.
*/
pub trait Directory<'a> {
    /// The writer to write to the file in the directory.
    type Writer: Write;
    /// Errors of the directory operation.
    type DirError: std::error::Error;
    /// Open a file at a path relative to the directory for appending.
    fn open_append(&'a mut self, path: &Path) -> Result<Self::Writer, Self::DirError>;
}



pub mod dummydir;
