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
    Association(Association),
    Warning(Warning),
    WarningForNewcomers,
}

pub struct Article {
    id: Identifier,
    title: Text,
    slogan: Option<Slogan>,
    contents: NonEmpty<Vec<BlockElement>>,
}
