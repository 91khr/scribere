/*!
Read the code blocks in the source.
*/



mod sourcecode;
pub use self::sourcecode::SourceCode;
use crate::codeblock::CodeBlock;



/**
Read the source code into iterator over code blocks.
*/
pub trait Read<'a> {
    /// The result iterator.
    type Output: Iterator<Item = CodeBlock<'a>>;
    /// The errors during read.
    type Error: std::error::Error;
    /// Read the source code and return the result.
    fn read(&self, src: &'a mut SourceCode<'a, '_>) -> Result<Self::Output, Self::Error>;
}



#[cfg(any(feature = "read_cmark", test, doc))]
pub mod cmark;
