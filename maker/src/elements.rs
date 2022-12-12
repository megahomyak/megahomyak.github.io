use std::path::PathBuf;

use crate::{context::Context, special_types::non_empty::NonEmpty};

use html_builder::Html5;
use url::Url;

use crate::{
    special_types::{identifier::Identifier, slogan::Slogan},
    traits::to_html::ToHtml,
};

// Note: an article should not be written to be false. If something is needed to be negated, it is
// convenient to add negation to the article itself.
pub enum Truthfulness {
    False,
    True,
    Unknown,
}

// Truthfulness of a section should be derived from the references and subsections located inside
// the section under discussion, and if nothing to derive from was found, a default value should be
// used.
impl Default for Truthfulness {
    fn default() -> Self {
        Self::Unknown
    }
}

pub enum ProgrammingLanguage {
    Rust,
    Python,
    /// A programming language that was made up
    Pseudo,
    HTML,
}

pub struct Image {
    id: Identifier,
    contextual_description: Option<Text>,
}

impl ToHtml for Image {
    fn to_html(&self, buffer: html_builder::Buffer, context: Context) {
        buffer.img().attr("src=''")
    }
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
    StronglyEmphasized(NonEmpty<Vec<TextPart>>),
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
    truthfullness: Option<Truthfulness>,
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
}
