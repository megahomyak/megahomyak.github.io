use crate::utils::capture::Capture;

pub struct Slogan(String);

pub enum CreationError {
    StringIsEmpty,
    StringBeginsWithWhitespace,
    StringEndsWithWhitespace,
}

impl Slogan {
    pub fn new(origin: String) -> Result<Self, (CreationError, String)> {
        let capture = Capture(origin);
        if capture.peek().is_empty() {
            capture.err(CreationError::StringIsEmpty) } else if capture.peek()
            .chars()
            .next()
            .expect("String shouldn't be empty at this point")
            .is_whitespace()
        {
            capture.err(CreationError::StringBeginsWithWhitespace)
        } else if capture.peek()
            .chars()
            .rev()
            .next()
            .expect("String shouldn't be empty at this point")
            .is_whitespace()
        {
            capture.err(CreationError::StringEndsWithWhitespace)
        } else {
            Ok(Self(capture.deconstruct()))
        }
    }

    pub fn contents(&self) -> &str {
        &self.0
    }
}
