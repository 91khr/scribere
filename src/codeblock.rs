/*!
A code block in the source.
*/

/**
A code block in the source.
*/
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CodeBlock<'a> {
    /// The language of the code block.
    pub lang: &'a str,
    /// The content of the code block.
    pub content: &'a str,
    /// The attributes of the code block.
    pub attrs: Vec<(&'a str, &'a str)>,
}
