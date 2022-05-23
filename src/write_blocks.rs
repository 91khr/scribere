/*!
Write all code blocks in the iterator to the directory with the dispatcher.
*/



use std::io::Write;

use crate::codeblock::CodeBlock;
use crate::directory::Directory;
use crate::dispatch::Dispatch;



/**
Write all code blocks in the iterator to the directory with the dispatcher.
*/
pub fn write_blocks<'a>(
    mut it: impl Iterator<Item = CodeBlock<'a>>,
    dir: &mut impl Directory,
    mut disp: impl Dispatch<'a>,
) {
    let mut blk = match it.next() {
        Some(b) => b,
        None => return,
    };
    let mut path = disp.dispatch(&blk).unwrap();
    loop {
        let mut writer = dir.open_append(path).unwrap();
        loop {
            writer.write_all(blk.content.as_bytes()).unwrap();
            blk = match it.next() {
                Some(b) => b,
                None => return,
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
    #[test]
    fn some() {
    }
}
