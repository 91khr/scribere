/*!
A dummy directory for testing.
*/



use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str::Utf8Error;

use super::Directory;
use crate::read::SourceCode;



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

/// Convert the pair to its wrapping source code string,
/// so that we can write out the type of `DummyDir::DirIter`.
fn conv_elem_to_str<'a, T>(x: (&T, &'a Vec<u8>)) -> Result<SourceCode<'a, 'a>, Utf8Error> {
    Ok(std::str::from_utf8(x.1)?.into())
}

impl Directory for DummyDir {
    type DirIter<'a> = std::iter::Map<
        std::collections::hash_map::Iter<'a, PathBuf, Vec<u8>>,
        fn((&PathBuf, &'a Vec<u8>)) -> Result<SourceCode<'a, 'a>, Utf8Error>,
    >;
    type OpenError = !;
    type WalkError = Utf8Error;
    type Writer<'a> = &'a mut Vec<u8>;

    fn open_append<'a>(&'a mut self, path: &Path) -> Result<Self::Writer<'a>, Self::OpenError> {
        Ok(self.ctnt.entry(path.to_path_buf()).or_default())
    }

    fn walk(&self) -> Result<Self::DirIter<'_>, Self::OpenError> {
        Ok(self.ctnt.iter().map(conv_elem_to_str))
    }
}



#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::path::{Path, PathBuf};

    use super::DummyDir;
    use crate::directory::Directory;

    #[test]
    fn fill_ctnt() {
        let mut dir = DummyDir::new();
        dir.open_append(Path::new("a")).unwrap().write_all(b"hello").unwrap();
        dir.open_append(Path::new("b")).unwrap().write_all(b"hi").unwrap();
        let mut ctnt = dir.into_iter().collect::<Vec<_>>();
        ctnt.sort();
        assert_eq!(ctnt, [("a".into(), b"hello".to_vec()), ("b".into(), b"hi".to_vec())]);
    }

    #[test]
    fn walk() {
        let dir = DummyDir::with_ctnt([
            (PathBuf::from("a"), b"hello".to_vec()),
            (PathBuf::from("b"), b"hi".to_vec()),
        ]);
        let mut ctnt = dir.walk().unwrap().map(|x| x.unwrap()).collect::<Vec<_>>();
        ctnt.sort();
        assert_eq!(ctnt, ["hello".into(), "hi".into()]);
    }
}
