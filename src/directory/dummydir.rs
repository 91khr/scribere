/*!
A dummy directory for testing.
*/



use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::Directory;



/**
A dummy directory storing its content in the memory.
*/
#[derive(Debug)]
pub struct DummyDir {
    /// The simulated content of the directory.
    ctnt: HashMap<PathBuf, Vec<u8>>,
}

impl DummyDir {
    /// Construct an empty directory.
    pub fn new() -> Self {
        Self { ctnt: HashMap::new() }
    }

    /// Construct from given content.
    pub fn with_ctnt(elems: impl IntoIterator<Item = (PathBuf, Vec<u8>)>) -> Self {
        let mut ctnt = HashMap::new();
        ctnt.extend(elems);
        Self { ctnt }
    }
}

impl Extend<(PathBuf, Vec<u8>)> for DummyDir {
    fn extend<T: IntoIterator<Item = (PathBuf, Vec<u8>)>>(&mut self, iter: T) {
        self.ctnt.extend(iter)
    }

    fn extend_one(&mut self, item: (PathBuf, Vec<u8>)) {
        self.ctnt.extend_one(item)
    }

    fn extend_reserve(&mut self, additional: usize) {
        self.ctnt.extend_reserve(additional)
    }
}

impl IntoIterator for DummyDir {
    type IntoIter = std::collections::hash_map::IntoIter<PathBuf, Vec<u8>>;
    type Item = (PathBuf, Vec<u8>);

    fn into_iter(self) -> Self::IntoIter {
        self.ctnt.into_iter()
    }
}

impl Default for DummyDir {
    fn default() -> Self {
        Self::new()
    }
}

impl Directory for DummyDir {
    type DirError = !;
    type Writer<'a> = &'a mut Vec<u8>;

    fn open_append<'a>(&'a mut self, path: &Path) -> Result<Self::Writer<'a>, Self::DirError> {
        Ok(self.ctnt.entry(path.to_path_buf()).or_default())
    }
}



#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::path::Path;

    use super::DummyDir;
    use crate::directory::Directory;

    #[test]
    fn some_dummy() {
        let mut dir = DummyDir::new();
        dir.open_append(Path::new("a")).unwrap().write_all(b"hello").unwrap();
        dir.open_append(Path::new("b")).unwrap().write_all(b"hi").unwrap();
        let mut ctnt = dir.into_iter().collect::<Vec<_>>();
        ctnt.sort();
        assert_eq!(ctnt, [("a".into(), b"hello".to_vec()), ("b".into(), b"hi".to_vec())]);
    }
}
