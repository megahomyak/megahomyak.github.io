mod context;
mod elements;
mod html;
mod special_types;
mod utils;

use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new(".").into_iter().map(Result::unwrap) {
        let root = xmltree::Element::parse(std::fs::File::open(entry.path()).unwrap()).unwrap();
    }
}
