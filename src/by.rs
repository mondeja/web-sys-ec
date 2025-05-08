pub(crate) mod inner {
    pub enum By {
        Id(String),
        Class(String),
        TagName(String),
        QuerySelector(String),
    }

    impl core::fmt::Display for By {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                By::Id(id) => write!(f, "HTML element with id '{}' (`{:?}`)", id, &self),
                By::Class(class) => {
                    write!(f, "HTML element with class '{}' (`{:?}`)", class, &self)
                }
                By::TagName(tag_name) => write!(
                    f,
                    "HTML element with tag name '{}' (`{:?}`)",
                    tag_name, &self
                ),
                By::QuerySelector(selector) => write!(
                    f,
                    "HTML element queried with selector '{}' (`{:?}`)",
                    selector, &self
                ),
            }
        }
    }

    impl core::fmt::Debug for By {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                By::Id(id) => write!(f, "By::Id({id:?})"),
                By::Class(class) => write!(f, "By::Class({class:?})"),
                By::TagName(tag_name) => write!(f, "By::TagName({tag_name:?})"),
                By::QuerySelector(selector) => write!(f, "By::QuerySelector({selector:?})"),
            }
        }
    }
}

#[allow(non_snake_case)]
pub mod By {
    use super::inner;

    #[inline]
    pub fn Id(id: impl Into<String>) -> inner::By {
        inner::By::Id(id.into())
    }

    #[inline]
    pub fn Class(class: impl Into<String>) -> inner::By {
        inner::By::Class(class.into())
    }

    #[inline]
    pub fn TagName(tag_name: impl Into<String>) -> inner::By {
        inner::By::TagName(tag_name.into())
    }

    #[inline]
    pub fn QuerySelector(selector: impl Into<String>) -> inner::By {
        inner::By::QuerySelector(selector.into())
    }
}
