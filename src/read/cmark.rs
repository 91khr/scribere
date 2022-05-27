/*!
Read markdown with the [pulldown-cmark] library.

[pulldown-cmark]: https://docs.rs/pulldown-cmark/latest/pulldown_cmark/
*/



use pulldown_cmark::{Event, Parser};

use super::{Read, ReadOut};
use crate::codeblock::CodeBlock;



/**
Read the source code and filter out the code blocks in it.
*/
#[derive(Debug, Clone)]
pub struct Reader<F: Clone + for<'a> FnMut(Event<'a>) -> Option<CodeBlock<'a>>> {
    /// The filter to pick out the blocks.
    filter: F,
}

impl<F: Clone + for<'a> FnMut(Event<'a>) -> Option<CodeBlock<'a>>> Reader<F> {
    /// Construct a new reader with the filter.
    pub fn new(filter: F) -> Self {
        Self { filter }
    }
}

/**
The output type of [`Reader::read`].
*/
#[allow(missing_debug_implementations)] // `Parser` dont impl `Debug` >_<
pub struct ReaderOut<'a, F: Clone + FnMut(Event<'a>) -> Option<CodeBlock<'a>>> {
    /// The filter to pick out the blocks.
    filter: F,
    /// The parser generating the blocks.
    it: Parser<'a, 'a>,
}

impl<'a, F: Clone + FnMut(Event<'a>) -> Option<CodeBlock<'a>>> Iterator for ReaderOut<'a, F> {
    type Item = CodeBlock<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        for ev in self.it.by_ref() {
            if let Some(blk) = (self.filter)(ev) {
                return Some(blk);
            }
        }
        None
    }
}

impl<F: Clone + for<'a> FnMut(Event<'a>) -> Option<CodeBlock<'a>>> ReadOut for Reader<F> {
    type Output<'a> = ReaderOut<'a, F>;
}

impl<F: Clone + for<'a> FnMut(Event<'a>) -> Option<CodeBlock<'a>>> Read for Reader<F> {
    type Error = std::io::Error;

    fn read<'a>(&mut self, src: &'a mut super::SourceCode<'a, '_>) -> Result<Self::Output<'a>, Self::Error> {
        src.to_code()?;
        Ok(ReaderOut {
            filter: self.filter.clone(),
            it: Parser::new(src.as_code().unwrap()),
        })
    }
}



#[cfg(test)]
mod tests {
    use super::Reader;
    use crate::codeblock::CodeBlock;
    use crate::read::Read;

    #[test]
    fn some() {
        let mut rd = Reader::new(|e| match e {
            pulldown_cmark::Event::Code(x) => Some(CodeBlock {
                lang: "".into(),
                content: x.to_string().into(),
                attrs: vec![],
            }),
            _ => None,
        });
        let mut c1 = "hello `print(world)` hi".into();
        let mut c2 = "`CodeBlocks`".into();
        let a = rd.read(&mut c1).unwrap();
        let b = rd.read(&mut c2).unwrap();
        assert_eq!(
            a.collect::<Vec<_>>(),
            vec![CodeBlock {
                lang: "".into(),
                content: "print(world)".into(),
                attrs: vec![],
            }]
        );
        assert_eq!(
            b.collect::<Vec<_>>(),
            vec![CodeBlock {
                lang: "".into(),
                content: "CodeBlocks".into(),
                attrs: vec![],
            }]
        );
    }
}
