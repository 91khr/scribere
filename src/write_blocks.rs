/*!
Write all code blocks in the iterator to the directory with the dispatcher.
*/



use std::io::Write;

use thiserror::Error;

use crate::directory::Directory;
use crate::dispatch::Event;



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
Write all code blocks in the iterator according to its dispatching targets.
*/
pub fn write_blocks<'a, Dir: Directory, E: std::error::Error>(
    mut it: impl Iterator<Item = Result<Event<'a>, E>>,
    dir: &mut Dir,
) -> Result<(), WriteError<Dir::OpenError, E>> {
    let mut event = match it.next() {
        Some(Ok(e)) => e,
        Some(Err(e)) => return Err(WriteError::BlockError(e)),
        None => return Ok(()),
    };
    loop {
        let mut writer = dir
            .open_append(&event.target.ok_or(WriteError::NullPath)?)
            .map_err(WriteError::DirError)?;
        loop {
            writer
                .write_all(event.block.content.as_bytes())
                .map_err(WriteError::IOError)?;
            match it.next() {
                Some(Ok(e)) => {
                    event = e;
                    if event.target.is_some() {
                        break;
                    }
                }
                Some(Err(e)) => return Err(WriteError::BlockError(e)),
                None => return Ok(()),
            }
        }
    }
}

/**
Write all code blocks in the errorless iterator according to its dispatching targets.
*/
pub fn write_blocks_errless<'a, Dir: Directory>(
    it: impl Iterator<Item = Event<'a>>,
    dir: &mut Dir,
) -> Result<(), WriteError<Dir::OpenError, !>> {
    write_blocks(it.map(Ok), dir)
}



#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::codeblock::CodeBlock;
    use crate::directory::dummydir::DummyDir;
    use crate::dispatch::{ByAttr, DispatchErrless};
    use crate::write_blocks::write_blocks_errless;

    #[test]
    fn some() {
        let ctnt = [
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
        .into_iter();
        let res = [
            (PathBuf::from("hello"), b"block 1\nblock 2\nblock 4\nblock 5\n".to_vec()),
            (PathBuf::from("hi"), b"block 3\n".to_vec()),
        ];
        let mut dir = DummyDir::new();
        write_blocks_errless(&mut ByAttr::new("a").dispatch(ctnt), &mut dir).unwrap();
        let mut dir = dir.into_iter().collect::<Vec<_>>();
        dir.sort();
        assert_eq!(dir, res);
    }

    #[test]
    fn empty_blocks() {
        write_blocks_errless(&mut ByAttr::new("a").dispatch([].into_iter()), &mut DummyDir::new()).unwrap();
    }
}
