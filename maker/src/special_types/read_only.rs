pub struct ReadOnly<T>(T);

impl<T> ReadOnly<T> {
    pub fn new(thing: T) -> Self {
        Self(thing)
    }

    pub fn peek(&self) -> &T {
        &self.0
    }
}
