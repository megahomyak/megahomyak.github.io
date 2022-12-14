#[derive(Clone, Copy)]
pub struct CssDeclarationBlock(&'static str);

impl CssDeclarationBlock {
    /// I don't want to validate this because of its complexity, so have a stupid creation method.
    pub fn new_unchecked(name: &'static str) -> Self {
        Self(name)
    }

    pub fn contents(&self) -> &'static str {
        &self.0
    }
}
