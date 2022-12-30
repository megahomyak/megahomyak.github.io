use std::{collections::HashMap, path::PathBuf};

use crate::{
    context::Context,
    html::{ElementKind, Node, Style, StylesCollectionError},
    special_types::{
        css_declaration_block::CssDeclarationBlock, html_attributes::HtmlAttributes,
        html_class_name::HtmlClassName, html_escaped_string::HtmlEscapedString,
        html_tag_name::HtmlTagName, non_empty::NonEmpty,
    },
};

use url::Url;

use crate::special_types::{identifier::Identifier, slogan::Slogan};

// Note: an article should not be written to be false. If something is needed to be negated, it is
// convenient to add negation to the article itself.
pub enum Truthfulness {
    AssumedToBeFalse,
    AssumedToBeTrue,
    Unknown,
    DetermineByContent,
}

pub enum ProgrammingLanguage {
    Rust,
    Python,
    /// A programming language that was made up
    Pseudo,
    Html,
}

pub struct Image {
    id: Identifier,
    contextual_description: Option<Text>,
}

pub struct OuterReference {
    url: Url,
}

pub enum InnerReferenceKind {
    /// A section of the current article
    LocalSection {
        section_id: Identifier,
    },
    ForeignArticle {
        article_id: Identifier,
    },
    ForeignSection {
        article_id: Identifier,
        section_id: Identifier,
    },
}

pub struct InnerReference {
    kind: InnerReferenceKind,
    text: Option<Text>,
}

pub enum TextPart {
    Plain(NonEmpty<String>),
    Emphasized(NonEmpty<Vec<TextPart>>),
    Important(NonEmpty<Vec<TextPart>>),
    InnerReference(InnerReference),
    OuterReference(OuterReference),
}

pub struct Text {
    contents: NonEmpty<Vec<TextPart>>,
}

pub struct Code {
    programming_language: ProgrammingLanguage,
    contents: String,
}

pub struct Section<ContentType> {
    id: Identifier,
    title: Option<Text>,
    contents: NonEmpty<Vec<ContentType>>,
    truthfullness: Truthfulness,
}

pub enum InlineElement {
    Text(Text),
    Code(Code),
    Section(Section<InlineElement>),
    ContactMe,
}

pub struct Paragraph {
    contents: NonEmpty<Vec<InlineElement>>,
}

pub struct List {
    contents: Vec<Paragraph>,
}

pub struct Set {
    contents: Vec<Paragraph>,
}

pub struct Association {
    information: Paragraph,
    visualization: NonEmpty<Vec<Image>>,
}

pub struct Warning {
    text: Text,
}

pub enum BlockElement {
    Image(Image),
    Paragraph(Paragraph),
    Section(Section<BlockElement>),
    List(List),
    Set(Set),
    Association(Association),
    Warning(Warning),
    WarningForNewcomers,
}

impl BlockElement {
    pub fn to_html(self) -> ! {
        todo!()
    }
}

// A catalog should *not* have an ID because it will not be linked internally and only the links to
// the compiled representation should be used for outer linking.
pub struct Catalog {
    input_directory: PathBuf,
    title: Text,
    slogan: Option<Slogan>,
    description: NonEmpty<Vec<BlockElement>>,
}

// A very important detail: any webpage generated must not contain any interactive elements, so it
// can be printed on paper. There should also be "print versions" linked on every page (or only
// every article?) containing the same information as their web counterparts, but black and white
// and with less fancy things such as links (even those going to the same webpage (in-place anchors
// included)) and big outer margins.
pub struct Article {
    id: Identifier,
    title: Text,
    slogan: Option<Slogan>,
    contents: NonEmpty<Vec<BlockElement>>,
    truthfulness: Truthfulness,
}

/// An error that occurs when converting an Article to HTML.
pub enum HtmlConversionError {
    Collision { class_name: HtmlClassName },
}

impl Article {
    pub fn to_html(self, context: Context) -> Result<String, HtmlConversionError> {
        let body = Node::Element {
            style: Some(Style {
                class_name: HtmlClassName::new("body").unwrap(),
                declaration_block: CssDeclarationBlock::new_unchecked(
                    "margin: 0; padding: 10%; background-color: #{background color in hex}; \
                    color: #{text color in hex}; word-wrap: anywhere; font-family: sans-serif;",
                ),
            }),
            attributes: HtmlAttributes::new(),
            name: HtmlTagName::new("body".to_string()).unwrap(),
            kind: ElementKind::Filled {
                contents: self
                    .contents
                    .deconstruct()
                    .into_iter()
                    .map(|block_element| block_element.to_html())
                    .collect(),
            },
        };
        let styles = match body.get_styles_recursively() {
            Ok(styles) => styles,
            Err(error) => match error {
                StylesCollectionError::Collision { class_name } => {
                    return Err(HtmlConversionError::Collision { class_name })
                }
            },
        };
        let head = Node::Element {
            style: None,
            attributes: HtmlAttributes::new(),
            name: HtmlTagName::new("head".to_string()).unwrap(),
            kind: ElementKind::Filled {
                contents: vec![Node::Element {
                    style: None,
                    attributes: HtmlAttributes::new(),
                    name: HtmlTagName::new("style".to_string()).unwrap(),
                    kind: ElementKind::Filled {
                        contents: vec![Node::Text(HtmlEscapedString::convert(
                            styles
                                .iter()
                                .map(|(class_name, declaration_block)| {
                                    format!(
                                        ".{} {{{}}}",
                                        class_name.contents(),
                                        declaration_block.contents()
                                    )
                                })
                                .collect::<Vec<String>>()
                                .join("\n"),
                        ))],
                    },
                }],
            },
        };
        let html = Node::Element {
            style: None,
            attributes: HtmlAttributes::new(),
            name: HtmlTagName::new("html".to_string()).unwrap(),
            kind: ElementKind::Filled {
                contents: vec![head, body],
            },
        };
        return Ok("<!DOCTYPE html>\n".to_owned() + &html.compile());
    }
}
