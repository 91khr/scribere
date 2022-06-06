/*!
Provide default targets for event iterators.
*/



use std::borrow::Cow;
use std::iter::Map;
use std::path::Path;

use super::Event;



/**
The iterator type returned by [`WithDefault::with_default`],
see [the document of the struct](WithDefault) for more.
*/
#[derive(Debug, Clone)]
pub struct WithDefaultIter<'a, It, E: std::error::Error>
where
    It: Iterator<Item = Result<Event<'a>, E>>,
{
    /// The underlying iterator.
    iter: It,
    /// The default path.
    default: Option<Cow<'a, Path>>,
}

impl<'a, It, E: std::error::Error> Iterator for WithDefaultIter<'a, It, E>
where
    It: Iterator<Item = Result<Event<'a>, E>>,
{
    type Item = Result<Event<'a>, E>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = self.iter.next();
        if let Some(Ok(ref mut x @ Event { target: None, block: _ })) = res {
            x.target = self.default.take();
        }
        self.default.take(); // Only replace the first block, so drop the default value here
        res
    }
}

/**
Add a default dispatch target for the event iterator.

It's explained in [the module document](mod@super)
that the first event should provide a target for the code blocks to be dispatched to,
and sometimes the target may be absent in the first event;
and this is what the trait (more precisely, the function) is for:
when the first code block lacks a target,
the target would be replaced with the path provided in the argument.

Another use of the function is that when it's necessary to split the event iterator,
the latter one can be provided the last target of the prior one as its default target,
so that the states of the iterators are correct.

# Examples

Examples here actually uses [`WithDefaultErrless::with_default`],
but the only difference between them is whether there may be errors during iteration ---
and there would be no errors in this example.

When the target of the first event is absent:

```
use scribere::dispatch::{Event, WithDefaultErrless};
use scribere::CodeBlock;
use std::path::Path;

let ctnt = [
        Event::new_none(CodeBlock::new("1", "", vec![])),
        Event::new_none(CodeBlock::new("2", "", vec![])),
        Event::new_some(Path::new("change"), CodeBlock::new("3", "", vec![])),
        Event::new_none(CodeBlock::new("4", "", vec![])),
    ]
    .into_iter();
let after = ctnt.clone().with_default(Path::new("default")).collect::<Vec<_>>();
assert_eq!(
    vec![
        Event::new_some(Path::new("default"), CodeBlock::new("1", "", vec![])),
        Event::new_none(CodeBlock::new("2", "", vec![])),
        Event::new_some(Path::new("change"), CodeBlock::new("3", "", vec![])),
        Event::new_none(CodeBlock::new("4", "", vec![])),
    ],
    after
);
```

When it exists (almost the same as the previous one):

```
use scribere::dispatch::{Event, WithDefaultErrless};
use scribere::CodeBlock;
use std::path::Path;

let ctnt = [
        Event::new_some(Path::new("given"), CodeBlock::new("1", "", vec![])),
        Event::new_none(CodeBlock::new("2", "", vec![])),
    ]
    .into_iter();
let after = ctnt.clone().with_default(Path::new("default")).collect::<Vec<_>>();
assert_eq!(
    vec![
        Event::new_some(Path::new("given"), CodeBlock::new("1", "", vec![])),
        Event::new_none(CodeBlock::new("2", "", vec![])),
    ],
    after
);
```

*/
pub trait WithDefault<'a, E: std::error::Error>: Iterator<Item = Result<Event<'a>, E>> + Sized {
    /// Add a default dispatch target for the event iterator.
    ///
    /// See [the struct document](Self) for more.
    fn with_default(self, default: impl Into<Cow<'a, Path>>) -> WithDefaultIter<'a, Self, E>;
}

impl<'a, E: std::error::Error, It: Iterator<Item = Result<Event<'a>, E>>> WithDefault<'a, E> for It {
    fn with_default(self, default: impl Into<Cow<'a, Path>>) -> WithDefaultIter<'a, Self, E> {
        WithDefaultIter {
            iter: self,
            default: Some(default.into()),
        }
    }
}



/**
The iterator type returned by [`WithDefaultErrless::with_default`],
see [the document of `WithDefault`](WithDefault) for more.
*/
#[derive(Debug, Clone)]
//# The type is not intended for human to use its content ><
#[allow(clippy::type_complexity)]
pub struct WithDefaultErrlessIter<'a, It: Iterator<Item = Event<'a>>>(
    WithDefaultIter<'a, Map<It, fn(Event<'a>) -> Result<Event<'a>, !>>, !>,
);

impl<'a, It: Iterator<Item = Event<'a>>> Iterator for WithDefaultErrlessIter<'a, It> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| x.expect("Never type should never occur"))
    }
}

/**
Add a default dispatch target for the errorless event iterator.

See [the document of `WithDefault`](WithDefault) for more.
*/
pub trait WithDefaultErrless<'a>: Iterator<Item = Event<'a>> + Sized {
    /// Add a default dispatch target for the event iterator.
    ///
    /// See [the document of `WithDefault`](WithDefault) for more.
    fn with_default(self, default: impl Into<Cow<'a, Path>>) -> WithDefaultErrlessIter<'a, Self>;
}

impl<'a, It: Iterator<Item = Event<'a>>> WithDefaultErrless<'a> for It {
    fn with_default(self, default: impl Into<Cow<'a, Path>>) -> WithDefaultErrlessIter<'a, Self> {
        WithDefaultErrlessIter(WithDefaultIter {
            iter: self.map(Ok),
            default: Some(default.into()),
        })
    }
}



#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::WithDefaultErrless;
    use crate::codeblock::CodeBlock;
    use crate::dispatch::Event;

    #[test]
    fn followings() {
        let ctnt = [
            Event::new_none(CodeBlock::new("1", "", vec![])),
            Event::new_none(CodeBlock::new("2", "", vec![])),
            Event::new_some(Path::new("c"), CodeBlock::new("3", "", vec![])),
        ]
        .into_iter();
        let after = ctnt.clone().with_default(Path::new("b")).collect::<Vec<_>>();
        assert_eq!(
            vec![
                Event::new_some(Path::new("b"), CodeBlock::new("1", "", vec![])),
                Event::new_none(CodeBlock::new("2", "", vec![])),
                Event::new_some(Path::new("c"), CodeBlock::new("3", "", vec![])),
            ],
            after
        );
    }
}
