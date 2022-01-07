#[derive(Debug, PartialEq)]
pub struct Purl(Uri);

#[derive(Debug, PartialEq)]
pub struct Uri(url::Url);
