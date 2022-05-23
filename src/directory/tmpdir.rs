/*!
Implement [`Directory`](super::Directory) for [`tempfile::Tempdir`].
*/



use std::fs::File;

use tempfile::TempDir;

use super::Directory;



impl Directory for TempDir {
    type DirError = std::io::Error;
    type Writer<'a> = File;

    fn open_append<'a, 'b: 'a>(&'b mut self, path: &std::path::Path) -> Result<Self::Writer<'a>, Self::DirError> {
        File::create(self.path().join(path))
    }
}
