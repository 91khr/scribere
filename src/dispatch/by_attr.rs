/*!
Dispatch the code blocks to files according to their attributes.
*/



use std::borrow::Cow;
use std::path::Path;

use super::Dispatch;
use crate::codeblock::CodeBlock;



/**
Dispatch the code blocks to files according to their attributes.

The name of the attribute must be given, and when the attribute is absent,
the code block would be dispatched to the same file as the last code block.
The first code block should have the attribute, or the writer would raise an error.
*/
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByAttr<'a>(&'a str);

impl<'a> ByAttr<'a> {
    /// Create a new dispatcher with the attribute name.
    pub fn new(name: &'a str) -> Self {
        Self(name)
    }
}

impl Dispatch for ByAttr<'_> {
    fn dispatch<'a>(&mut self, block: &'a CodeBlock<'a>) -> Option<&'a std::path::Path> {
        block
            .attrs
            .iter()
            .find(|x| x.0 == self.0)
            .map(|x| Path::new(Cow::as_ref(&x.1)))
    }
}



#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::codeblock::CodeBlock;
    use crate::dispatch::{ByAttr, Dispatch};

    #[test]
    fn some() {
        let mut disp = ByAttr::new("a");
        assert_eq!(
            disp.dispatch(
                CodeBlock::default().with_attrs(vec![("b".into(), "qaq".into()), ("a".into(), "qwq".into())])
            ),
            Some(Path::new("qwq"))
        );
        assert_eq!(
            disp.dispatch(CodeBlock::default().with_attrs(vec![("b".into(), "qwq".into())])),
            None
        );
        assert_eq!(
            disp.dispatch(CodeBlock::default().with_attrs(vec![("a".into(), "qeq".into())])),
            Some(Path::new("qeq"))
        );
    }
}
