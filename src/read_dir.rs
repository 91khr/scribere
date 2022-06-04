/*!
Utilities to read all files in a directory and dispatch them into files.
*/



use std::fmt::Debug;
use std::path::Path;
use std::pin::Pin;

use thiserror::Error;

use crate::directory::Directory;
use crate::dispatch::Event;
use crate::read::{Read, SourceCode};



/**
The error when iterating over the directory.

See [the document of `read_dir`] for more.
*/
#[derive(Debug, Error)]
pub enum IterError<W: std::error::Error, R: std::error::Error> {
    /// Error during walking the directory.
    #[error("walk file error: {0}")]
    WalkError(W),
    /// Error during reading file content.
    #[error("read file error: {0}")]
    ReadError(R),
}



/**
The event iterator over all code blocks in all sources in the directory.

The iterator is returned by [`read_dir`], see its document for more.
*/
#[derive(Debug, Clone)]
pub struct DirIter<'a, D: Directory + 'a, R: Read> {
    /// The iterator over the sources in the directory.
    files: D::DirIter<'a>,
    /// The reader that reads code blocks in a source.
    reader: R,
    /// Source codes yielded by `files` so far,
    /// pinned to make sure it's avaliable during
    src: Vec<Pin<Box<SourceCode<'a, 'a>>>>,
    /// Iterators over the code blocks in sources yielded by `files` so far.
    blocks: Vec<R::Output<'a>>,
    /// Current dispatch target. Will be taken in the next iteration.
    target: Option<&'a Path>,
}

impl<'a, D: Directory + 'a, R: Read + 'a> Iterator for DirIter<'a, D, R>
where
    Self: 'a,
{
    type Item = Result<Event<'a>, IterError<D::WalkError, R::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut iter) = self.blocks.last_mut() {
            if let Some(blk) = iter.next() {
                return Some(Ok(Event::new(self.target.take(), blk)));
            }
        }
        self.src.push(match self.files.next()? {
            Ok(s) => Box::pin(s),
            Err(e) => return Some(Err(IterError::WalkError(e))),
        });
        let src = self.src.last_mut().expect("It's just pushed").as_mut().get_mut();
        // Safety: There are two requirements for the safety of this transmution:
        // 1. src outlives `'a`: Since `Self: 'a`, and it's borrowed from self, this is guaranteed.
        // 2. src is only borrowed once during `'a`: It's only borrowed here,
        //    and each time we borrow from self.src, only the newly-pushed source is borrowed.
        let src = unsafe { std::mem::transmute::<&mut SourceCode<'a, 'a>, &'a mut SourceCode<'a, 'a>>(src) };
        self.blocks.push(match self.reader.read(src) {
            Ok(b) => b,
            Err(e) => return Some(Err(IterError::ReadError(e))),
        });
        self.next()
    }
}

/**
Read the content in the directory with a reader,
returning an event iterator dispatching them to the same relative path.

Usually the targets yielded by the result event iterator need to be processed
to have prettier names, or, for example, for a file in the source directory named `src/a.md`,
code blocks in it would be dispatched to `src/a.md` too (both pathes are relative path),
which is usually not what's expected.
*/
pub fn read_dir<D: Directory, R: Read>(dir: &mut D, reader: R) -> Result<DirIter<D, R>, D::OpenError> {
    Ok(DirIter {
        files: dir.walk()?,
        reader,
        blocks: vec![],
        target: None,
        src: vec![],
    })
}



#[cfg(test)]
mod tests {}
