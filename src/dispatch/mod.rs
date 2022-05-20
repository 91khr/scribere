/*!
Dispatch code in blocks to the path to files they should be put into.

The dispatcher is assumed to be stateful --- the first invocation to it should return `Some(path)`,
stating the path to the file the code block should be dispatched into,
and latter invocations returning `None` for keep the block written to the same file,
`Some(path)` for writing the block into another file.
*/



use std::path::Path;

use crate::codeblock::CodeBlock;



/**
Dispatch the code block to the file they should be put to.

See the [module document](mod@self) for more details.
*/
pub trait Dispatch<'a> {
    /// Dispatch the code block.
    fn dispatch(&mut self, block: CodeBlock<'a>) -> Option<&'a Path>;
}

impl<'a, T: FnMut(CodeBlock<'a>) -> Option<&'a Path>> Dispatch<'a> for T {
    fn dispatch(&mut self, block: CodeBlock<'a>) -> Option<&'a Path> {
        (self)(block)
    }
}



mod monofile;
pub use monofile::MonoFile;