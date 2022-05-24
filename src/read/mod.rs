/*!
Read the code blocks in the source.
*/



mod sourcecode;
pub use self::sourcecode::SourceCode;
use crate::codeblock::CodeBlock;



/**
Read the source code into iterator over code blocks.
*/
pub trait Read {
    /// The result iterator.
    type Output<'a>: Iterator<Item = CodeBlock<'a>>
    where
        Self: 'a;
    /// The errors during read.
    type Error: std::error::Error;
    /// Read the source code and return the result.
    fn read<'a>(&'a mut self, src: &'a mut SourceCode<'a, '_>) -> Result<Self::Output<'a>, Self::Error>;
}



#[cfg(feature = "read_cmark")]
#[doc(cfg(feature = "read_cmark"))]
pub mod cmark;
