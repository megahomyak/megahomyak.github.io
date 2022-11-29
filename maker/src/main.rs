mod filled_string;
mod capture;
mod dependency_link;
mod section_id;
mod existing_path;
mod elements;

use std::path::PathBuf;

use dependency_link::ExistingDependency;
use filled_string::FilledString;
use url::Url;
use walkdir::WalkDir;

enum ProofValue {
    Unproven,
    Proven,
    OnlyEvidence,
}

enum Text {
    Plain(FilledString),
    Emphasized(FilledString),
    Strong(FilledString),
    EmphasizedAndStrong(FilledString),
    Code(FilledString),
}

struct Section {
    id: String,
    title: Option<Text>,
    proof_value: ProofValue,
    contents: Vec<Node>,
    inner_dependency_links: Vec<ExistingDependency>,
}

enum Image {
    Outer { url: Url },
    Inner { path: PathBuf },
}

type List = Vec<ListItem>;

enum ListItem {
    Text(Text),
    Image(Image),
    Sublist(List),
}

enum Node {
    Section(Section),
    List(List),
    Image(Image),
}

type Article = xmltree::Element;

fn process(article: Article) -> Section {}

fn main() {
    for entry in WalkDir::new(".").into_iter().map(|entry| entry.unwrap()) {
        let root = xmltree::Element::parse(std::fs::File::open(entry.path()).unwrap());
    }
}
