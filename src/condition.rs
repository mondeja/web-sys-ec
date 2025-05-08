use crate::{by::inner::By, ec::inner::Ec};
use std::borrow::Cow;

#[derive(Debug)]
pub(crate) struct Condition {
    pub(crate) by: Option<By>,
    pub(crate) ec: Option<Ec>,
}

impl From<(By, Ec)> for Condition {
    fn from((by, ec): (By, Ec)) -> Self {
        Condition {
            by: Some(by),
            ec: Some(ec),
        }
    }
}

impl From<Ec> for Condition {
    fn from(ec: Ec) -> Self {
        Condition {
            by: None,
            ec: Some(ec),
        }
    }
}

impl From<By> for Condition {
    fn from(by: By) -> Self {
        Condition {
            by: Some(by),
            ec: None,
        }
    }
}

impl From<&str> for Condition {
    fn from(selector: &str) -> Self {
        Condition {
            by: Some(By::QuerySelector(selector.to_string())),
            ec: None,
        }
    }
}

impl From<(&str, Ec)> for Condition {
    fn from((selector, ec): (&str, Ec)) -> Self {
        Condition {
            by: Some(By::QuerySelector(selector.to_string())),
            ec: Some(ec),
        }
    }
}

impl From<String> for Condition {
    fn from(selector: String) -> Self {
        Condition {
            by: Some(By::QuerySelector(selector)),
            ec: None,
        }
    }
}

impl From<(String, Ec)> for Condition {
    fn from((selector, ec): (String, Ec)) -> Self {
        Condition {
            by: Some(By::QuerySelector(selector)),
            ec: Some(ec),
        }
    }
}

impl From<Cow<'_, str>> for Condition {
    fn from(selector: Cow<'_, str>) -> Self {
        Condition {
            by: Some(By::QuerySelector(selector.to_string())),
            ec: None,
        }
    }
}

impl From<(Cow<'_, str>, Ec)> for Condition {
    fn from((selector, ec): (Cow<'_, str>, Ec)) -> Self {
        Condition {
            by: Some(By::QuerySelector(selector.to_string())),
            ec: Some(ec),
        }
    }
}
