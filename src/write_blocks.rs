/*!
Write all code blocks in the iterator to the directory with the dispatcher.
*/



use std::io::Write;

use thiserror::Error;

use crate::codeblock::CodeBlock;
use crate::directory::Directory;
use crate::dispatch::Dispatch;



/**
The result of writing operations.
*/
#[derive(Debug, Error)]
pub enum WriteError<E: std::error::Error> {
    /// The path returned by dispatcher for the first code block is `None`.
    #[error("dispatcher returned `None` for the first code block")]
    NullPath,
    /// The error returned from the directory.
    #[error("directory error: {0}")]
    DirError(E),
    /// The error during writing.
    #[error("IO error: {0}")]
    IOError(std::io::Error),
}



/**
Write all code blocks in the iterator to the directory with the dispatcher.
*/
pub fn write_blocks<'a, Dir: Directory>(
    it: &mut impl Iterator<Item = CodeBlock<'a>>,
    dir: &mut Dir,
    disp: &mut impl Dispatch<'a>,
) -> Result<(), WriteError<Dir::DirError>> {
    let mut blk = match it.next() {
        Some(b) => b,
        None => return Ok(()),
    };
    let mut path = disp.dispatch(&blk).ok_or(WriteError::NullPath)?;
    loop {
        let mut writer = dir.open_append(path).map_err(WriteError::DirError)?;
        loop {
            writer.write_all(blk.content.as_bytes()).map_err(WriteError::IOError)?;
            blk = match it.next() {
                Some(b) => b,
                None => return Ok(()),
            };
            if let Some(p) = disp.dispatch(&blk) {
                path = p;
                break;
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::codeblock::CodeBlock;
    use crate::directory::dummydir::DummyDir;
    use crate::dispatch::ByAttr;
    use crate::write_blocks;

    #[test]
    fn some() {
        let mut dir = DummyDir::new();
        write_blocks(
            &mut [
                CodeBlock {
                    lang: "",
                    content: "block 1\n",
                    attrs: vec![("a", "hello")],
                },
                CodeBlock {
                    lang: "",
                    content: "block 2\n",
                    attrs: vec![("b", "hello")],
                },
                CodeBlock {
                    lang: "",
                    content: "block 3\n",
                    attrs: vec![("a", "hi")],
                },
                CodeBlock {
                    lang: "",
                    content: "block 4\n",
                    attrs: vec![("a", "hello")],
                },
                CodeBlock {
                    lang: "",
                    content: "block 5\n",
                    attrs: vec![("b", "hello")],
                },
            ]
            .into_iter(),
            &mut dir,
            &mut ByAttr::new("a"),
        )
        .unwrap();
        let mut dir = dir.dump().into_iter().collect::<Vec<_>>();
        dir.sort();
        assert_eq!(
            dir,
            [
                (PathBuf::from("hello"), b"block 1\nblock 2\nblock 4\nblock 5\n".to_vec()),
                (PathBuf::from("hi"), b"block 3\n".to_vec())
            ]
        );
    }

    #[test]
    fn empty_blocks() {
        write_blocks(&mut [].into_iter(), &mut DummyDir::new(), &mut ByAttr::new("a")).unwrap();
    }
}
