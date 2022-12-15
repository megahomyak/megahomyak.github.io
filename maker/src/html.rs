use std::collections::HashMap;

use crate::special_types::{
    css_declaration_block::CssDeclarationBlock, html_attributes::HtmlAttributes,
    html_class_name::HtmlClassName, html_escaped_string::HtmlEscapedString,
    html_tag_name::HtmlTagName,
};

pub struct Style {
    pub class_name: HtmlClassName,
    pub declaration_block: CssDeclarationBlock,
}

pub enum ElementKind {
    Filled { contents: Vec<Node> },
    Empty,
}

pub enum Node {
    Element {
        style: Option<Style>,
        attributes: HtmlAttributes,
        name: HtmlTagName,
        kind: ElementKind,
    },
    Text(HtmlEscapedString),
}

pub enum StylesCollectionError {
    Collision { class_name: HtmlClassName },
}

impl Node {
    pub fn compile(self) -> String {
        match self {
            Self::Text(text) => text.deconstruct(),
            Self::Element {
                name,
                kind,
                style,
                attributes,
            } => {
                let attributes: String = attributes
                    .deconstruct()
                    .into_iter()
                    .map(|(k, v)| {
                        if let Some(v) = v {
                            format!(" {}=\"{}\"", k.contents(), v.deconstruct())
                        } else {
                            k.contents().to_string()
                        }
                    })
                    .collect();

                let textual_representation = {
                    let mut parts = Vec::from([{
                        match style {
                            Some(style) => format!(
                                "<{}{} class=\"{}\">",
                                name.contents(),
                                attributes,
                                style.class_name.contents()
                            ),
                            None => format!("<{}{}>", name.contents(), attributes),
                        }
                    }]);

                    match kind {
                        ElementKind::Empty => (),
                        ElementKind::Filled { contents } => {
                            parts.extend(contents.into_iter().map(|node| {
                                let lines: Vec<String> = node
                                    .compile()
                                    .split("\n")
                                    .map(|line| "    ".to_owned() + line)
                                    .collect();
                                lines.join("\n")
                            }));
                            parts.push(format!("</{}>", name.contents()));
                        }
                    }

                    parts.join("\n")
                };
                textual_representation
            }
        }
    }

    pub fn get_styles_recursively(
        &self,
    ) -> Result<HashMap<HtmlClassName, CssDeclarationBlock>, StylesCollectionError> {
        match self {
            Self::Text(_) => Ok(HashMap::new()),
            Self::Element { .. } => {
                let mut styles = HashMap::new();
                let mut remaining_nodes = vec![self];
                while let Some(node) = remaining_nodes.pop() {
                    if let Node::Element { style, kind, .. } = node {
                        if let Some(style) = style {
                            if styles.contains_key(&style.class_name) {
                                return Err(StylesCollectionError::Collision {
                                    class_name: style.class_name,
                                });
                            }
                            styles.insert(style.class_name, style.declaration_block);
                        }
                        if let ElementKind::Filled { contents } = kind {
                            remaining_nodes.extend(contents.into_iter());
                        }
                    }
                }
                Ok(styles)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::special_types::html_attribute_name::HtmlAttributeName;

    use super::*;

    use indoc::indoc;

    #[test]
    fn test_complex_tree_generation() {
        let compiled_complex_tree = Node::Element {
            name: HtmlTagName::new("div".to_string()).unwrap(),
            attributes: HtmlAttributes::new(),
            style: Some(Style {
                class_name: HtmlClassName::new("classname").unwrap(),
                declaration_block: CssDeclarationBlock::new_unchecked("blahblah"),
            }),
            kind: ElementKind::Filled {
                contents: vec![
                    Node::Element {
                        style: None,
                        attributes: HtmlAttributes::new(),
                        name: HtmlTagName::new("p".to_string()).unwrap(),
                        kind: ElementKind::Filled {
                            contents: vec![
                                Node::Text(HtmlEscapedString::convert("Hello".to_string())),
                                Node::Element {
                                    style: None,
                                    attributes: HtmlAttributes::new(),
                                    name: HtmlTagName::new("br".to_string()).unwrap(),
                                    kind: ElementKind::Empty,
                                },
                                Node::Text(HtmlEscapedString::convert("world".to_string())),
                            ],
                        },
                    },
                    Node::Element {
                        style: None,
                        attributes: {
                            let mut attributes = HtmlAttributes::new();
                            attributes
                                .insert(
                                    HtmlAttributeName::new("src").unwrap(),
                                    Some(HtmlEscapedString::convert("image.png".to_string())),
                                )
                                .unwrap();
                            attributes
                                .insert(
                                    HtmlAttributeName::new("alt").unwrap(),
                                    Some(HtmlEscapedString::convert(
                                        "Image description".to_string(),
                                    )),
                                )
                                .unwrap();
                            attributes
                        },
                        name: HtmlTagName::new("img".to_string()).unwrap(),
                        kind: ElementKind::Empty,
                    },
                ],
            },
        }
        .compile();
        assert!(
            compiled_complex_tree
                == indoc! {r#"
                <div class="classname">
                    <p>
                        Hello
                        <br>
                        world
                    </p>
                    <img src="image.png" alt="Image description">
                </div>"#}
                .to_string()
                || compiled_complex_tree
                    == indoc! {r#"
                <div class="classname">
                    <p>
                        Hello
                        <br>
                        world
                    </p>
                    <img alt="Image description" src="image.png">
                </div>"#},
            "compiled_complex_tree = {}",
            compiled_complex_tree
        );
    }
}
