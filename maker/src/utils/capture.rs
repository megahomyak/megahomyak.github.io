pub struct Capture<Input> {
    input: Input,
}

impl<Input> Capture<Input> {
    pub fn new(input: Input) -> Capture<Input> {
        Capture { input }
    }

    pub fn error<Ok, Error>(self, error: Error) -> Result<Ok, (Error, Input)> {
        Err((error, self.input))
    }
}
