use std::{collections::HashMap, path::PathBuf};

use crate::special_types::{read_only::ReadOnly, identifier::Identifier, non_empty::NonEmpty};

pub struct ImageInfo {
    path: PathBuf,
    absolute_description: NonEmpty<String>,
}

pub struct Context {
    images: ReadOnly<HashMap<Identifier, ImageInfo>>,
}

impl Context {
    pub fn images(&self) -> &ReadOnly<HashMap<Identifier, ImageInfo>> {
        &self.images
    }
}
