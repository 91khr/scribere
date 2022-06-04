/*!
Dispatch all blocks into a single file.
*/



use std::path::Path;

use super::{Dispatch, Event};
use crate::codeblock::CodeBlock;



/**
Dispatch all received blocks into a single file.

The file is specified through the constructor,
and only the first invocation to [`dispatch()`](Dispatch::dispatch) would return `Some(path)`,
latter invocations would return `None` directly.
*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MonoFile<'a>(Option<&'a Path>);

impl<'a> MonoFile<'a> {
    /// Create a new dispatcher from a path.
    pub fn new(path: &'a Path) -> Self {
        Self(Some(path))
    }
}

impl<'a> From<&'a Path> for MonoFile<'a> {
    fn from(path: &'a Path) -> Self {
        Self::new(path)
    }
}

/**
The iterator returned by [`MonoFile::dispatch`].

See [the document of `Dispatch`](Dispatch) for more.
*/
#[derive(Debug, Clone)]
pub struct Iter<'a, E: std::error::Error, It: Iterator<Item = Result<CodeBlock<'a>, E>>> {
    /// The path to be dispatched to.
    path: Option<&'a Path>,
    /// The underlying code block iterator.
    iter: It,
}

impl<'a, E: std::error::Error, It: Iterator<Item = Result<CodeBlock<'a>, E>>> Iterator for Iter<'a, E, It> {
    type Item = Result<Event<'a>, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(blk)) => Some(Ok(Event::new(self.path.take(), blk))),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }
}

impl Dispatch for MonoFile<'_> {
    type Output<'a, It, E: std::error::Error> = Iter<'a, E, It> where Self: 'a, It: Iterator<Item = Result<CodeBlock<'a>, E>>;

    fn dispatch<'a, E: std::error::Error, It: Iterator<Item = Result<CodeBlock<'a>, E>>>(
        &'a mut self,
        iter: It,
    ) -> Self::Output<'a, It, E> {
        Iter { path: self.0, iter }
    }
}



#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::MonoFile;
    use crate::codeblock::CodeBlock;
    use crate::dispatch::{DispatchErrless, Event};

    #[test]
    fn some() {
        let mut mono = MonoFile::new(Path::new("a"));
        let iter = [CodeBlock::new("", "1", vec![]), CodeBlock::new("", "2", vec![])].into_iter();
        assert_eq!(
            mono.dispatch_errless(iter).collect::<Vec<_>>(),
            vec![
                Event::new(Some(Path::new("a")), CodeBlock::new("", "1", vec![])),
                Event::new::<&Path>(None, CodeBlock::new("", "2", vec![]))
            ]
        );
    }
}
