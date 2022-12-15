#[derive(Debug)]
pub enum CreationError {
    AttributeNameContainsADisallowedCharacter { index: usize },
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct HtmlAttributeName(&'static str);

impl HtmlAttributeName {
    pub fn new(name: &'static str) -> Result<Self, (CreationError, &'static str)> {
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
