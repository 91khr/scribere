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
pub enum WriteError<D: std::error::Error, B: std::error::Error> {
    /// The path returned by dispatcher for the first code block is `None`.
    #[error("dispatcher returned `None` for the first code block")]
    NullPath,
    /// The error returned from the directory.
    #[error("directory error: {0}")]
    DirError(D),
    /// The error during writing.
    #[error("IO error: {0}")]
    IOError(std::io::Error),
    /// The error while iterating the code blocks.
    #[error("iterating blocks error: {0}")]
    BlockError(B),
}



/**
Write all code blocks in the iterator to the directory with the dispatcher,
where the iterator may yield an error.
*/
pub fn write_blocks_mayerr<'a, Dir: Directory, E: std::error::Error>(
    mut it: impl Iterator<Item = Result<CodeBlock<'a>, E>>,
    disp: &mut impl Dispatch,
    dir: &mut Dir,
) -> Result<(), WriteError<Dir::OpenError, E>> {
    let mut blk = match it.next() {
        Some(Ok(b)) => b,
        Some(Err(e)) => return Err(WriteError::BlockError(e)),
        None => return Ok(()),
    };
    let mut path = disp.dispatch(&blk).ok_or(WriteError::NullPath)?;
    loop {
        let mut writer = dir.open_append(path).map_err(WriteError::DirError)?;
        loop {
            writer.write_all(blk.content.as_bytes()).map_err(WriteError::IOError)?;
            blk = match it.next() {
                Some(Ok(b)) => b,
                Some(Err(e)) => return Err(WriteError::BlockError(e)),
                None => return Ok(()),
            };
            if let Some(p) = disp.dispatch(&blk) {
                path = p;
                break;
            }
        }
    }
}

/**
Write all code blocks in the iterator to the directory with the dispatcher,
where the iterator won't yield errors.

The function forwards to [`write_blocks_mayerr`],
for detailed document, see its document.
*/
pub fn write_blocks<'a, Dir: Directory>(
    it: impl Iterator<Item = CodeBlock<'a>>,
    disp: &mut impl Dispatch,
    dir: &mut Dir,
) -> Result<(), WriteError<Dir::OpenError, !>> {
    write_blocks_mayerr(it.map(Ok), disp, dir)
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
            [
                CodeBlock {
                    lang: "".into(),
                    content: "block 1\n".into(),
                    attrs: vec![("a".into(), "hello".into())],
                },
                CodeBlock {
                    lang: "".into(),
                    content: "block 2\n".into(),
                    attrs: vec![("b".into(), "hello".into())],
                },
                CodeBlock {
                    lang: "".into(),
                    content: "block 3\n".into(),
                    attrs: vec![("a".into(), "hi".into())],
                },
                CodeBlock {
                    lang: "".into(),
                    content: "block 4\n".into(),
                    attrs: vec![("a".into(), "hello".into())],
                },
                CodeBlock {
                    lang: "".into(),
                    content: "block 5\n".into(),
                    attrs: vec![("b".into(), "hello".into())],
                },
            ]
            .into_iter(),
            &mut ByAttr::new("a"),
            &mut dir,
        )
        .unwrap();
        let mut dir = dir.into_iter().collect::<Vec<_>>();
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
        write_blocks([].into_iter(), &mut ByAttr::new("a"), &mut DummyDir::new()).unwrap();
    }
}
