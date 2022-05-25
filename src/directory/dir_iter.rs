/*!
An iterator over all files in a directory in the filesystem.
*/



use std::fs::{read_dir, ReadDir};
use std::io::Error;
use std::marker::PhantomData;
use std::path::Path;

use crate::read::SourceCode;



/**
An iterator over all files in a directory in the filesystem.
*/
#[derive(Debug)]
pub struct DirIter<'a> {
    /// All iterating directories, in a stack.
    state: Vec<ReadDir>,
    /// We need to return a type of such lifetime bound, make the rustc happy.
    phantom: PhantomData<&'a Path>,
}

impl DirIter<'_> {
    /// Create an iterator in the directory at `path`.
    pub fn new(path: &Path) -> Result<Self, Error> {
        Ok(Self {
            state: vec![read_dir(path)?],
            phantom: PhantomData,
        })
    }
}

impl<'a> Iterator for DirIter<'a> {
    // The result item is owned
    type Item = Result<SourceCode<'a, 'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.state.last_mut().unwrap().next() {
            None => {
                self.state.pop();
                return if self.state.is_empty() { None } else { self.next() };
            }
            Some(Ok(x)) => x,
            Some(Err(e)) => return Some(Err(e)),
        };
        if item.path().is_dir() {
            self.state.push(match read_dir(item.path()) {
                Ok(x) => x,
                Err(e) => return Some(Err(e)),
            });
            self.next()
        } else {
            Some(Ok(item.path().into()))
        }
    }
}



// Tested in `tmpdir`
