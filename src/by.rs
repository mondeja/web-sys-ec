#[derive(Debug)]
pub enum By {
    Id(String),
    Class(String),
    TagName(String),
    QuerySelector(String),
}