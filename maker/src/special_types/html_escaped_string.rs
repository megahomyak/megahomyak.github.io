use std::borrow::Cow;

pub struct HtmlEscapedString(String);

impl HtmlEscapedString {
    pub fn convert(origin: String) -> Self {
        Self(match html_escape::encode_safe(&origin) {
            Cow::Borrowed(_same_string) => origin,
            Cow::Owned(result) => result,
        })
    }

    pub fn contents(&self) -> &str {
        &self.0
    }

    pub fn deconstruct(self) -> String {
        self.0
    }
}
