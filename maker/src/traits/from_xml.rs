pub trait FromXml where Self: Sized {
    type Error;

    fn from_xml(xml: xmltree::Element) -> Result<Self, (Self::Error, xmltree::Element)>;
}
