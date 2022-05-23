/*!
The directories to store extracted codes in files.
*/



use std::io::Write;
use std::path::Path;



/**
The directory to manipulate files.

Only one file can be opened for appending at a time, the lifetime is to guarantee this.
*/
pub trait Directory {
    /// The writer to write to the file in the directory.
    type Writer<'a>: Write where Self: 'a;
    /// Errors of the directory operation.
    type DirError: std::error::Error;
    /// Open a file at a path relative to the directory for appending.
    fn open_append<'a, 'b: 'a>(&'b mut self, path: &Path) -> Result<Self::Writer<'a>, Self::DirError>;
}



pub mod dummydir;
