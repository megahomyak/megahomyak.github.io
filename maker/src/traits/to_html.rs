pub trait ToHtml {
    type Context;
    type Error;

    fn to_html(&self, context: Self::Context) -> Result<build_html::Container, Self::Error>;
}
