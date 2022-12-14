#[derive(Debug)]
pub enum CreationError {
    TagNameContainsADisallowedCharacter { index: usize },
    TagNameIsEmpty,
}

pub struct HtmlTagName(String);

impl HtmlTagName {
    pub fn new(name: String) -> Result<Self, (CreationError, String)> {
        if name.is_empty() {
            return Err((CreationError::TagNameIsEmpty, name));
        }
        for (index, character) in name.chars().enumerate() {
            if !character.is_ascii_alphanumeric() {
                return Err((
                    CreationError::TagNameContainsADisallowedCharacter { index },
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
