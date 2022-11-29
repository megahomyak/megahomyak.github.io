pub enum CreationError {
    StringIsEmpty,
}

pub struct FilledString(String);

impl FilledString {
    pub fn new(origin: String) -> Result<Self, (CreationError, String)> {
        if origin.len() == 0 {
            Err(Creat)
        }
        let chars = origin.chars();
        let capture = Capture::new(origin);
        if let Some(c) = chars.next() {
            if c.is_whitespace() {
                capture.error(CreationError::StringBeginsWithWhitespace)
            } else {
                if chars.last().map_or(false, |c| c.is_whitespace()) {
                    capture.error(CreationError::StringEndsWithWhitespace)
                } else {
                    Ok(Self(origin))
                }
            }
        } else {
            capture.error(CreationError::StringIsEmpty)
        }
    }
}
