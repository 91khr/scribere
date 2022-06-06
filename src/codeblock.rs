/*!
A code block in the source.
*/

use std::borrow::Cow;

/**
A code block in the source.
*/
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CodeBlock<'a> {
    /// The content of the code block.
    pub content: Cow<'a, str>,
    /// The language of the code block.
    pub lang: Cow<'a, str>,
    /// The attributes of the code block.
    pub attrs: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> CodeBlock<'a> {
    /// Create an empty codeblock.
    pub fn new(
        content: impl Into<Cow<'a, str>>,
        lang: impl Into<Cow<'a, str>>,
        attrs: Vec<(Cow<'a, str>, Cow<'a, str>)>,
    ) -> Self {
        Self {
            content: content.into(),
            lang: lang.into(),
            attrs,
        }
    }

    /// Set lang and return self.
    pub fn with_lang(&mut self, lang: Cow<'a, str>) -> &mut Self {
        self.lang = lang;
        self
    }

    /// Set content and return self.
    pub fn with_content(&mut self, ctnt: Cow<'a, str>) -> &mut Self {
        self.content = ctnt;
        self
    }

    /// Set attributes and return self.
    pub fn with_attrs(&mut self, attrs: Vec<(Cow<'a, str>, Cow<'a, str>)>) -> &mut Self {
        self.attrs = attrs;
        self
    }
}

impl Default for CodeBlock<'_> {
    fn default() -> Self {
        Self::new("", "", vec![])
    }
}
