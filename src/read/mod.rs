/*!
Reading from sources and return information in them.
*/



mod sourcecode;
pub use self::sourcecode::SourceCode;
use crate::codeblock::CodeBlock;



/**
The result type container for [`Read`].

For detailed document, see [the document of `Read`](Read).
*/
pub trait ReadOut {
    /// The result iterator.
    type Output<'a>: Iterator<Item = CodeBlock<'a>>;
}

/**
Read the source code into iterator over code blocks.
*/
pub trait Read: ReadOut {
    /// The errors during read.
    type Error: std::error::Error;
    /// Read the source code and return the result.
    fn read<'a>(&mut self, src: &'a mut SourceCode<'a, '_>) -> Result<Self::Output<'a>, Self::Error>;
}



#[cfg(feature = "read_cmark")]
#[doc(cfg(feature = "read_cmark"))]
pub mod cmark;
