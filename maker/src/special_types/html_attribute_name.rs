#[derive(Debug)]
pub enum CreationError {
    AttributeNameContainsADisallowedCharacter { index: usize },
}

#[derive(PartialEq, Eq, Hash)]
pub struct HtmlAttributeName(String);

impl HtmlAttributeName {
    pub fn new(name: String) -> Result<Self, (CreationError, String)> {
        for (index, character) in name.chars().enumerate() {
            if !(character.is_ascii_alphanumeric() || character == '-') {
                return Err((
                    CreationError::AttributeNameContainsADisallowedCharacter { index },
                    name,
                ));
            }
        }
        Ok(Self(name))
    }

    pub fn contents(&self) -> &str {
        &self.0
    }
}
