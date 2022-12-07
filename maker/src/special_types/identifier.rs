use crate::utils::capture::Capture;

pub struct Identifier(String);

pub enum CreationError {
    StringIsEmpty,
    StringBeginsWithWhitespace,
    StringEndsWithWhitespace,
}

impl Identifier {
    pub fn new(origin: String) -> Result<Self, (CreationError, String)> {
        let capture = Capture(origin);
        if origin.len() == 0 {
            capture.err(CreationError::StringIsEmpty) } else if origin
            .chars()
            .next()
            .expect("String shouldn't be empty at this point")
            .is_whitespace()
        {
            capture.err(CreationError::StringBeginsWithWhitespace)
        } else if origin
            .chars()
            .rev()
            .next()
            .expect("String shouldn't be empty at this point")
            .is_whitespace()
        {
            capture.err(CreationError::StringEndsWithWhitespace)
        } else {
            Ok(Self(origin))
        }
    }

    pub fn contents(&self) -> &str {
        &self.0
    }
}
