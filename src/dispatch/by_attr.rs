/*!
Dispatch the code blocks to files according to their attributes.
*/



use std::borrow::Cow;
use std::path::PathBuf;
use std::str::FromStr;

use super::{Dispatch, Event};
use crate::codeblock::CodeBlock;



/**
Dispatch the code blocks to files according to their attributes.

The name of the attribute must be given, and when the attribute is absent,
the code block would be dispatched to the same file as the last code block.
The first code block should have the attribute, or the writer would raise an error.
*/
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByAttr<'a> {
    /// The name of the attribute indicating the destination file.
    name: &'a str,
}

impl<'a> ByAttr<'a> {
    /// Create a new dispatcher with the attribute name.
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

/**
The iterator returned by [`ByAttr::dispatch`].

See [the document of `Dispatch`](Dispatch) for more.
*/
#[derive(Debug, Clone)]
pub struct Iter<'a, It, E: std::error::Error>
where
    It: Iterator<Item = Result<CodeBlock<'a>, E>>,
{
    /// The name of the attribute indicating the destination file.
    name: &'a str,
    /// The underlying code block iterator.
    iter: It,
}

impl<'a, It, E: std::error::Error> Iterator for Iter<'a, It, E>
where
    It: Iterator<Item = Result<CodeBlock<'a>, E>>,
{
    type Item = Result<Event<'a>, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(blk)) => Some(Ok(Event::new(
                blk.attrs
                    .iter()
                    .find(|x| x.0 == self.name)
                    .map(|x| PathBuf::from_str(Cow::as_ref(&x.1)).unwrap()),
                blk,
            ))),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }
}

impl Dispatch for ByAttr<'_> {
    type Output<'a, It, E: std::error::Error> = Iter<'a, It, E>
    where
        Self: 'a,
        It: Iterator<Item = Result<CodeBlock<'a>, E>>;

    fn dispatch<'a, E: std::error::Error, It: Iterator<Item = Result<CodeBlock<'a>, E>>>(
        &'a mut self,
        iter: It,
    ) -> Self::Output<'a, It, E> {
        Iter { name: self.name, iter }
    }
}



#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::path::Path;

    use crate::codeblock::CodeBlock;
    use crate::dispatch::{ByAttr, DispatchErrless, Event};

    #[test]
    fn some() {
        let mut disp = ByAttr::new("a");
        let iter = [
            CodeBlock::new("", "1", vec![("a".into(), "qwq".into())]),
            CodeBlock::new("", "2", vec![]),
        ]
        .into_iter();
        assert_eq!(
            disp.dispatch_errless(iter).collect::<Vec<_>>(),
            vec![
                Event::new(
                    Some(Cow::from(Path::new("qwq"))),
                    CodeBlock::new("", "1", vec![("a".into(), "qwq".into())])
                ),
                Event::new::<&Path>(None, CodeBlock::new("", "2", vec![]))
            ]
        );
    }
}
