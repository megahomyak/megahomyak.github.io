use std::collections::HashMap;

use super::{html_attribute_name::HtmlAttributeName, html_escaped_string::HtmlEscapedString};

pub struct HtmlAttributes(HashMap<HtmlAttributeName, Option<HtmlEscapedString>>);

#[derive(Debug)]
pub enum InsertionError {
    AttributeNameIsDisallowed,
}

impl HtmlAttributes {
    pub fn new() -> HtmlAttributes {
        Self(HashMap::new())
    }

    pub fn insert(
        &mut self,
        name: HtmlAttributeName,
        value: Option<HtmlEscapedString>,
    ) -> Result<
        (),
        (
            InsertionError,
            (HtmlAttributeName, Option<HtmlEscapedString>),
        ),
    > {
        if name.contents().to_lowercase() == "class" {
            Err((InsertionError::AttributeNameIsDisallowed, (name, value)))
        } else {
            self.0.insert(name, value);
            Ok(())
        }
    }

    pub fn contents(&self) -> &HashMap<HtmlAttributeName, Option<HtmlEscapedString>> {
        &self.0
    }

    pub fn deconstruct(self) -> HashMap<HtmlAttributeName, Option<HtmlEscapedString>> {
        self.0
    }
}
