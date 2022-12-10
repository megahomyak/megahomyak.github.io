use std::collections::HashMap;

use crate::{
    elements,
    special_types::{
        self, existing_path::ExistingPath, identifier::Identifier, read_only::ReadOnly,
    },
};

pub enum Article {
    Invalid(elements::Article),
    Valid(special_types::valid_article::ValidArticle),
}

pub struct Context {
    image_paths: ReadOnly<HashMap<Identifier, ExistingPath>>,
    articles: HashMap<Identifier, Article>,
}
