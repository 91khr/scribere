/*!
Implement [`Directory`](super::Directory) for [`tempfile::TempDir`].
*/



use std::fs::{create_dir_all, File};

use tempfile::TempDir;

use super::Directory;



impl Directory for TempDir {
    type DirError = std::io::Error;
    type Writer<'a> = File;

    fn open_append<'a>(&'a mut self, path: &std::path::Path) -> Result<Self::Writer<'a>, Self::DirError> {
        let path = self.path().join(path);
        create_dir_all(path.parent().expect("The TempDir and its file should have a path"))?;
        File::create(path)
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
}
