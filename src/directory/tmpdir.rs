/*!
Implement [`Directory`](super::Directory) for [`tempfile::TempDir`].
*/



use std::fs::{create_dir_all, File};

use tempfile::TempDir;

use super::dir_iter::DirIter;
use super::Directory;



impl Directory for TempDir {
    type DirIter<'a> = DirIter<'a>;
    type OpenError = std::io::Error;
    type WalkError = std::io::Error;
    type Writer<'a> = File;

    fn open_append<'a>(&'a mut self, path: &std::path::Path) -> Result<Self::Writer<'a>, Self::OpenError> {
        let path = self.path().join(path);
        create_dir_all(path.parent().expect("The TempDir and its file should have a path"))?;
        File::create(path)
    }

    fn walk(&self) -> Result<Self::DirIter<'_>, Self::OpenError> {
        DirIter::new(self.path())
    }
}



#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use std::io::Write;
    use std::path::Path;

    use tempfile::TempDir;

    use crate::directory::Directory;

    #[test]
    fn new_file() {
        let mut dir = TempDir::new().unwrap();
        let base = dir.path().to_path_buf();
        let mut f = dir.open_append(Path::new("file")).unwrap();
        f.write_all(b"Some text").unwrap();
        drop(f);
        assert_eq!(read_to_string(base.join("file")).unwrap(), "Some text");
        let mut f = dir.open_append(Path::new("a/b")).unwrap();
        f.write_all(b"File b").unwrap();
        drop(f);
        assert_eq!(read_to_string(base.join("a/b")).unwrap(), "File b");
    }

    #[test]
    fn walk_dir() {
        let mut dir = TempDir::new().unwrap();
        let base = dir.path().to_str().unwrap().to_string();
        let fnames = [
            "a/b/c/d/e",
            "a/b/c/d/f",
            "a/b/c/e",
            "a/b/f",
            "a/sdf",
            "b/d/g",
            "b/dg",
            "b/g/d",
            "c",
            "f",
        ];
        for s in fnames {
            dir.open_append(Path::new(s)).unwrap();
        }
        let mut ctnt = dir
            .walk()
            .unwrap()
            .map(|x| {
                let mut path = x.unwrap().as_owned_file().unwrap().to_str().unwrap().to_string();
                assert!(path.starts_with(&base));
                path.split_off(base.len() + 1)
            })
            .collect::<Vec<_>>();
        ctnt.sort();
        assert_eq!(ctnt, fnames);
    }
}
