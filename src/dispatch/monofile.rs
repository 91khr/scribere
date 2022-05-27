/*!
Dispatch all blocks into a single file.
*/



use std::path::Path;

use super::Dispatch;
use crate::codeblock::CodeBlock;



/**
Dispatch all received blocks into a single file.

The file is specified through the constructor,
and only the first invocation to [`dispatch()`](Dispatch::dispatch) would return `Some(path)`,
latter invocations would return `None` directly.
*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MonoFile<'a>(Option<&'a Path>);

impl<'a> MonoFile<'a> {
    /// Create a new dispatcher from a path.
    pub fn new(path: &'a Path) -> Self {
        Self(Some(path))
    }
}

impl<'a> From<&'a Path> for MonoFile<'a> {
    fn from(path: &'a Path) -> Self {
        Self::new(path)
    }
}

impl Dispatch for MonoFile<'_> {
    fn dispatch<'a>(&'a mut self, _block: &CodeBlock<'a>) -> Option<&'a Path> {
        self.0.take()
    }
}



#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::MonoFile;
    use crate::codeblock::CodeBlock;
    use crate::dispatch::Dispatch;

    #[test]
    fn some() {
        let mut mono = MonoFile::new(Path::new("a"));
        assert_eq!(mono.dispatch(&CodeBlock::default()), Some(Path::new("a")));
        assert_eq!(mono.dispatch(&CodeBlock::default()), None);
    }
}
