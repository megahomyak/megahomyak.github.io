mod special_types;
mod elements;
mod utils;
mod traits;
mod context;

use std::path::PathBuf;

use walkdir::WalkDir;

pub struct ArticleData {
    path: PathBuf,
    contents: Article,
}

fn main() {
    for entry in WalkDir::new(".").into_iter().map(|entry| entry.unwrap()) {
        let root = xmltree::Element::parse(std::fs::File::open(entry.path()).unwrap());
    }
}
