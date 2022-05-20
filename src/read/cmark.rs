/*!
Read markdown with the [pulldown-cmark] library.

[pulldown-cmark]: https://docs.rs/pulldown-cmark/latest/pulldown_cmark/
*/



use pulldown_cmark::{Event, Parser};

use super::Read;
use crate::codeblock::CodeBlock;



/**
Read the source code and filter out the code blocks in it.
*/
#[derive(Debug, Clone)]
pub struct Reader<F: Clone + for<'a> FnMut(Event<'a>) -> Option<CodeBlock<'a>>> {
    /// The filter to pick out the blocks.
    filter: F,
}

/**
The output type of [`Reader::read`]
*/
#[allow(missing_debug_implementations)]
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

impl<'a, F: Clone + for<'b> FnMut(Event<'b>) -> Option<CodeBlock<'b>>> Read<'a> for Reader<F> {
    type Error = std::io::Error;
    type Output = ReaderOut<'a, F>;

    fn read(&self, src: &'a mut super::SourceCode<'a, '_>) -> Result<Self::Output, Self::Error> {
        src.to_owned_string()?;
        Ok(ReaderOut {
            filter: self.filter.clone(),
            it: Parser::new(src.as_str().unwrap()),
        })
    }
}



#[cfg(test)]
mod tests {}
