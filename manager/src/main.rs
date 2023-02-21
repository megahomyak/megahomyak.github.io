static OUTPUT_DIR_NAME: &'static str = "generated";
static INPUT_DIR_NAME: &'static str = "input";
static IMAGES_DIR_NAME: &'static str = "images";

use std::{collections::HashMap, path::Path};

use nxml::Node;

#[derive(PartialEq, Eq, Hash)]
struct Id(String);

struct HTMLDocument(String);

struct Manager {
    documents: HashMap<Id, HTMLDocument>,
}

struct Index(usize);

impl Manager {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
        }
    }

    /// Both validate the document and convert it into HTML
    pub fn add_document(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let nodes = nxml::parse_sequential_nodes(&std::fs::read_to_string(path)?)?;
        let root = {
            let nodes = nxml::fields(nodes.iter());
            nxml::fields(
                nodes
                    .get("article")
                    .or_else(|| nodes.get("catalog"))
                    .ok_or("this is not an article or a catalog")?
                    .iter(),
            )
        };
        let id = root
            .get("id")
            .and_then(|id| id.iter().next())
            .and_then(|id| id.text())
            .ok_or("id is not text")?;
        let html_document = {
            let mut styles = Vec::<&'static str>::new();
            let mut body = String::new();
            format!(
                "<html><head><style>{}</style></head><body>{}</body></html>",
                styles.join("\n"),
                body
            )
        };
        self.documents
            .insert(Id(id.clone()), HTMLDocument(html_document));
        Ok(())
    }
}

fn main() -> Result<(), &'static str> {
    println!("Cleaning the output directory...");
    std::fs::remove_dir_all(OUTPUT_DIR_NAME).unwrap();

    let mut manager = Manager::new();

    println!("Building the articles...");
    walkdir::WalkDir::new(INPUT_DIR_NAME)
        .into_iter()
        .map(|f| f.unwrap())
        .fold(Ok(()), |result, file| {
            match manager.add_document(file.path()) {
                Ok(()) => result,
                Err(e) => {
                    println!(
                        "An error encountered when processing {:?}: {:?}",
                        file.path(),
                        e
                    );
                    Err("Compilation wasn't successful")
                }
            }
        })
}
