/*!
Dispatch code in blocks to the path to files they should be put into.

This is done by transforming an iterator over the code blocks into
an iterator of [`Event`]s, which contains the target file of the code block,
and the code block itself.
The `Event` is stateful, just like `Iterator`s themselves;
i.e. its target([`Event::target`]) would be `Some(path)`
only when the code block should be put into a file different from the previous block,
otherwise (when `target` is `None`), the code block should be put to the same file
as the previous code block.

So it's the users' responsibility **not** to use the event iterator seperately,
or the semantics of the event states is likely to be incorrect.
In other words, only process the event iterator from the beginning,
because any attempt of skipping the events without any proper attention to its states
would likely lead to errors.

Due to the statefulness of the event iterator, there is a requirement
that the first event returned by the `Iterator` created by [`Dispatch::dispatch`]
should contain a path to tell where the first code block should be put into,
or there is no way to do so.
However, the dispatchers here wouldn't explicitly confirm the requirement,
for example, a [`MonoFile`] would return the file path at and only at the first code block,
but a [`ByAttr`] would only check the attribute name,
and if the first code block has no such attribute indicating the target,
it simply returns `None` for the target,
and leave the following procedure to discover and report the error.

On the other hand, [`WithDefault`] provides a way to add a default target to an event iterator,
so that when the targets for the initial code blocks are absent,
they would be dispatched to the default target, see [its document](WithDefault) for more.

# Example

```
use scribere::CodeBlock;
use scribere::dispatch::{Event, DispatchErrless, ByAttr};
use std::path::Path;

let ctnt = [
        CodeBlock::new("1", "", vec![
            ("name".into(), "target".into()),
            ("unrelated".into(), "unrel".into())
            ]),
        CodeBlock::new("2", "", vec![("rand".into(), "attr".into())]),
        CodeBlock::new("3", "", vec![("another".into(), "rand attr".into())]),
        CodeBlock::new("4", "", vec![("name".into(), "another target".into())]),
        CodeBlock::new("5", "", vec![("last".into(), "block".into())]),
    ]
    .into_iter();
let disp = ByAttr::new("name");
let res = disp.dispatch(ctnt).collect::<Vec<_>>();
assert_eq!(
    res,
    [
        Event::new_some(Path::new("target"), CodeBlock::new("1", "", vec![
            ("name".into(), "target".into()),
            ("unrelated".into(), "unrel".into())
            ])),
        Event::new_none(CodeBlock::new("2", "", vec![("rand".into(), "attr".into())])),
        Event::new_none(CodeBlock::new("3", "", vec![("another".into(), "rand attr".into())])),
        Event::new_some(
            Path::new("another target"),
            CodeBlock::new("4", "", vec![("name".into(), "another target".into())]),
        ),
        Event::new_none(CodeBlock::new("5", "", vec![("last".into(), "block".into())])),
    ],
);
```
*/



use std::borrow::Cow;
use std::iter::Map;
use std::path::Path;

use crate::codeblock::CodeBlock;



/**
The event items returned by [`Dispatch`].

See the [module document](mod@self) for more details.

If the target is a constant of the form `Some(path)` or `None`,
it's better to construct with [`new_some`](Self::new_some) or [`new_none`](Self::new_none),
because it's impossible to infer a proper type
for the generic parameter of target in `Event::new(None, block)` currently,
since there's no information about it.
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
    ///
    /// If the target is a constant like `Some(path)` or `None`,
    /// use [`new_some`](Self::new_some) or [`new_none`](Self::new_none) instead.
    /// See [the struct document](Self) for more.
    pub fn new<T: Into<Cow<'a, Path>>>(target: Option<T>, block: CodeBlock<'a>) -> Self {
        Self {
            target: target.map(|x| x.into()),
            block,
        }
    }

    /// Construct an event with given dispatch target and code block content.
    ///
    /// This is equivalent to `new(Some(target), block)`,
    /// see [the struct document](Self) for more information and the reason.
    pub fn new_some(target: impl Into<Cow<'a, Path>>, block: CodeBlock<'a>) -> Self {
        Self::new(Some(target), block)
    }

    /// Construct an event with the given code block content.
    ///
    /// This is equivalent to `new::<&Path>(None, block)`,
    /// see [the struct document](Self) for more information and the reason.
    pub fn new_none(block: CodeBlock<'a>) -> Self {
        Self::new::<&Path>(None, block)
    }
}



/**
Dispatch the code block to the files they should be put into.

See the [module document](mod@self) for more.
*/
pub trait Dispatch {
    /// The output type of [`dispatch`](Self::dispatch).
    type Output<'a, It, E: std::error::Error>: Iterator<Item = Result<Event<'a>, E>>
    where
        Self: 'a,
        It: Iterator<Item = Result<CodeBlock<'a>, E>>;
    /// Dispatch the code block iterator into an [`Event`] iterator.
    fn dispatch<'a, It, E: std::error::Error>(&'a self, iter: It) -> Self::Output<'a, It, E>
    where
        It: Iterator<Item = Result<CodeBlock<'a>, E>>;
}

/**
Dispatch the code block in an errorless iterator to the files they should be put into.

This is automatically implemented for types implemented [`Dispatch`],
it's not intended to be implemented manually.

See the [module document](mod@self) for more.
*/
pub trait DispatchErrless {
    /// The output type of [`dispatch`](Self::dispatch).
    type Output<'a, It: Iterator<Item = CodeBlock<'a>>>: Iterator<Item = Event<'a>>
    where
        Self: 'a;
    /// Dispatch the code block iterator into an [`Event`] iterator.
    fn dispatch<'a, It: Iterator<Item = CodeBlock<'a>>>(&'a self, iter: It) -> Self::Output<'a, It>;
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

    fn dispatch<'a, It: Iterator<Item = CodeBlock<'a>>>(&'a self, iter: It) -> Self::Output<'a, It> {
        Dispatch::dispatch::<'a, Map<It, fn(CodeBlock<'a>) -> Result<CodeBlock<'a>, !>>, !>(self, iter.map(Ok))
            .map(|x| x.expect("Never type should not appear"))
    }
}



mod with_default;
pub use with_default::{WithDefault, WithDefaultErrless};

mod monofile;
pub use monofile::MonoFile;
mod by_attr;
pub use by_attr::ByAttr;
