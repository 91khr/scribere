/*!
Dispatch code in blocks to the path to files they should be put into.

The dispatcher is assumed to be stateful --- the first invocation to it should return `Some(path)`,
stating the path to the file the code block should be dispatched into,
and latter invocations returning `None` for keep the block written to the same file,
`Some(path)` for writing the block into another file.
However, no dispatcher here,
if it's up to the user to decide which file the block should be dispatched to,
would check if the requirement is satisfied.
*/



use std::borrow::Cow;
use std::iter::Map;
use std::path::Path;

use crate::codeblock::CodeBlock;



/**
The event items returned by [`Dispatch`].

See the [module document](mod@self) for more details.
*/
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event<'a> {
    /// Target file path for dispatching, `None` for the same as the previous block.
    pub target: Option<Cow<'a, Path>>,
    /// Code block in the source.
    pub block: CodeBlock<'a>,
}

impl<'a> Event<'a> {
    /// Construct an event with given dispatch target and code block content.
    pub fn new<T: Into<Cow<'a, Path>>>(target: Option<T>, block: CodeBlock<'a>) -> Self {
        Self {
            target: target.map(|x| x.into()),
            block,
        }
    }
}

/**
Dispatch the code block to the files they should be put into.

See the [module document](mod@self) for more details.
*/
pub trait Dispatch {
    /// The output type of [`dispatch`].
    type Output<'a, It, E: std::error::Error>: Iterator<Item = Result<Event<'a>, E>>
    where
        Self: 'a,
        It: Iterator<Item = Result<CodeBlock<'a>, E>>;
    /// Dispatch the code block iterator into an [`Event`] iterator.
    fn dispatch<'a, E: std::error::Error, It: Iterator<Item = Result<CodeBlock<'a>, E>>>(
        &'a mut self,
        iter: It,
    ) -> Self::Output<'a, It, E>;
}

/**
Dispatch the code block in an errorless iterator to the files they should be put into.

This is automatically implemented for types implemented [`Dispatch`],
it's not supposed intended to be implemented manually.
*/
pub trait DispatchErrless {
    /// The output type of [`dispatch_errless`].
    type Output<'a, It: Iterator<Item = CodeBlock<'a>>>: Iterator<Item = Event<'a>>
    where
        Self: 'a;
    /// Dispatch the code block iterator into an [`Event`] iterator.
    fn dispatch_errless<'a, It: Iterator<Item = CodeBlock<'a>>>(&'a mut self, iter: It) -> Self::Output<'a, It>;
}

impl<T: Dispatch> DispatchErrless for T {
    type Output<'a, It: Iterator<Item = CodeBlock<'a>>> =
        Map<
            <Self as Dispatch>::Output<
                'a,
                Map<It, fn(CodeBlock<'a>) -> Result<CodeBlock<'a>, !>>,
                !,
            >,
            fn(Result<Event<'a>, !>) -> Event<'a>,
        > where Self: 'a;

    fn dispatch_errless<'a, It: Iterator<Item = CodeBlock<'a>>>(&'a mut self, iter: It) -> Self::Output<'a, It> {
        Dispatch::dispatch::<'a, !, Map<It, fn(CodeBlock<'a>) -> Result<CodeBlock<'a>, !>>>(self, iter.map(Ok))
            .map(Result::unwrap)
    }
}



mod monofile;
pub use monofile::MonoFile;
mod by_attr;
pub use by_attr::ByAttr;
