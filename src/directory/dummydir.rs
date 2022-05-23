/*!
A dummy directory for testing.
*/



use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::Directory;



/**
A dummy directory for testing.
*/
#[derive(Debug)]
pub struct DummyDir {
    /// The simulated content of the directory.
    ctnt: HashMap<PathBuf, Vec<u8>>,
}

impl DummyDir {
    /// Create an empty DummyDir.
    pub fn new() -> Self {
        Self { ctnt: HashMap::new() }
    }

    /// Dump the content in self.
    pub fn dump(self) -> HashMap<PathBuf, Vec<u8>> {
        self.ctnt
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

    fn open_append<'a, 'b: 'a>(&'b mut self, path: &Path) -> Result<Self::Writer<'a>, Self::DirError> {
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
        let mut ctnt = dir.dump().into_iter().collect::<Vec<_>>();
        ctnt.sort();
        assert_eq!(ctnt, [("a".into(), b"hello".to_vec()), ("b".into(), b"hi".to_vec())]);
    }
}
