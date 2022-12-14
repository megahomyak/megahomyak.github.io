#[derive(Debug)]
pub enum CreationError {
    ClassNameContainsADisallowedCharacter { index: usize },
    ClassNameIsEmpty,
    ClassNameStartsWithAHyphen,
    ClassNameEndsWithAHyphen,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct HtmlClassName(&'static str);

impl HtmlClassName {
    pub fn new(name: &'static str) -> Result<Self, (CreationError, &'static str)> {
        if let Some(c) = name.chars().next() {
            if c == '-' {
                return Err((CreationError::ClassNameStartsWithAHyphen, name));
            }
        } else {
            return Err((CreationError::ClassNameIsEmpty, name));
        }
        if name.chars().rev().next().expect("It is proven above that the name is not empty") == '-' {
            return Err((CreationError::ClassNameEndsWithAHyphen, name));
        }
        for (index, character) in name.chars().enumerate() {
            if !(character.is_ascii_alphabetic() || character == '-') {
                return Err((
                    CreationError::ClassNameContainsADisallowedCharacter { index },
                    name,
                ));
            }
        }
        Ok(Self(name))
    }

    pub fn contents(&self) -> &'static str {
        &self.0
    }
}
