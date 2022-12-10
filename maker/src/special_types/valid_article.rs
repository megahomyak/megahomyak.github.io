use crate::elements;

/// An article containing the valid inner references
pub struct ValidArticle(elements::Article);

pub enum CreationError {
    ImageDoesNotExist,
}

impl ValidArticle {
    pub fn new(article: elements::Article) -> Result<Self, ()> {}
}
