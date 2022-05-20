/*!
Dispatch all blocks into a single file.
*/



use std::path::Path;

use super::Dispatch;
use crate::codeblock::CodeBlock;



/**
Dispatch all received blocks into a single file.
*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MonoFile<'a>(Option<&'a Path>);

impl<'a> MonoFile<'a> {
    /// Create a new dispatcher from a path.
    pub fn from_path(path: &'a Path) -> Self {
        Self(Some(path))
    }
}

impl<'a> From<&'a Path> for MonoFile<'a> {
    fn from(path: &'a Path) -> Self {
        Self::from_path(path)
    }
}

impl<'a> Dispatch<'a> for MonoFile<'a> {
    fn dispatch(&mut self, _block: CodeBlock<'a>) -> Option<&'a Path> {
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
        let mut mono = MonoFile::from_path(Path::new("a"));
        assert_eq!(mono.dispatch(CodeBlock::new()), Some(Path::new("a")));
        assert_eq!(mono.dispatch(CodeBlock::new()), None);
    }
}
