pub struct StringIsEmpty;

pub struct NonEmptyString(String);

impl NonEmptyString {
    pub fn new(origin: String) -> Result<Self, (StringIsEmpty, String)> {
        if origin.len() == 0 {
            Err((StringIsEmpty, origin))
        } else {
            Ok(Self(origin))
        }
    }

    pub fn contents(&self) -> &str {
        &self.0
    }
}
