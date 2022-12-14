/// Made for less repetition in functions that return their input on error
pub struct Capture<T>(pub T);

impl<T> Capture<T> {
    pub fn err<E, R>(self, err: E) -> Result<R, (E, T)> {
        Err((err, self.0))
    }

    pub fn deconstruct(self) -> T {
        self.0
    }

    pub fn peek(&self) -> &T {
        &self.0
    }
}
