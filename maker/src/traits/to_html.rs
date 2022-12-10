use crate::context::Context;

pub trait ToHtml {
    fn to_html(&self, buffer: html_builder::Buffer, context: Context);
}
