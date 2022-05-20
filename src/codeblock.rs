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

impl<'a> CodeBlock<'a> {
    /// Create an empty codeblock.
    pub fn new() -> Self {
        Self {
            lang: "",
            content: "",
            attrs: vec![],
        }
    }

    /// Set lang and return self.
    pub fn with_lang(&mut self, lang: &'a str) -> &mut Self {
        self.lang = lang;
        self
    }

    /// Set content and return self.
    pub fn with_content(&mut self, ctnt: &'a str) -> &mut Self {
        self.content = ctnt;
        self
    }

    /// Set attributes and return self.
    pub fn with_attrs(&mut self, attrs: Vec<(&'a str, &'a str)>) -> &mut Self {
        self.attrs = attrs;
        self
    }
}

impl Default for CodeBlock<'_> {
    fn default() -> Self {
        Self::new()
    }
}
