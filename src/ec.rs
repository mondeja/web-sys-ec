pub(crate) mod inner {
    pub enum Ec {
        InnerTextContains(String),
        AttributeValueIs(String, String),
        LocalStorageAttributeValueIs(String, String),
        LocationSearchIs(String),
    }

    impl core::fmt::Display for Ec {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Ec::InnerTextContains(text) => write!(
                    f,
                    "HTML element innerText contains the text {:?} (`{:?}`)",
                    text, &self
                ),
                Ec::AttributeValueIs(attr, value) => {
                    write!(
                        f,
                        "HTML element attribute {:?} value is equal to {:?} (`{:?}`)",
                        attr, value, &self
                    )
                }
                Ec::LocalStorageAttributeValueIs(attr, value) => {
                    write!(
                        f,
                        "localStorage attribute {:?} value is equal to {:?} (`{:?}`)",
                        attr, value, &self
                    )
                }
                Ec::LocationSearchIs(value) => {
                    write!(
                        f,
                        "window.location.search is equal to {:?} (`{:?}`)",
                        value, &self
                    )
                }
            }
        }
    }

    impl core::fmt::Debug for Ec {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Ec::InnerTextContains(text) => write!(f, "Ec::InnerTextContains({text:?})"),
                Ec::AttributeValueIs(attr, value) => {
                    write!(f, "Ec::AttributeValueIs({attr:?}, {value:?})")
                }
                Ec::LocalStorageAttributeValueIs(attr, value) => {
                    write!(f, "Ec::LocalStorageAttributeValueIs({attr:?}, {value:?})",)
                }
                Ec::LocationSearchIs(value) => write!(f, "Ec::LocationSearchIs({value:?})"),
            }
        }
    }
}

/// Conditions to be expected while waiting.
#[allow(non_snake_case)]
pub mod Ec {
    use super::inner;

    /// The property `innerText` of an element contains the given text.
    ///
    /// ```rust,ignore
    /// use web_sys_ec::{By, Ec, Wait};
    ///
    /// Wait(1).until((By::TagName("p"), Ec::InnerTextContains("text")));
    /// ```
    #[inline]
    pub fn InnerTextContains(text: impl Into<String>) -> inner::Ec {
        inner::Ec::InnerTextContains(text.into())
    }

    /// The attribute value of an element is equal to the given value.
    ///
    /// ```rust,ignore
    /// use web_sys_ec::{By, Ec, Wait};
    ///
    /// Wait(1).until((By::TagName("p"), Ec::AttributeValueIs("attr", "value")));
    /// ```
    #[inline]
    pub fn AttributeValueIs(attr: impl Into<String>, value: impl Into<String>) -> inner::Ec {
        inner::Ec::AttributeValueIs(attr.into(), value.into())
    }

    /// The localStorage attribute value is equal to the given value.
    ///
    /// ```rust,ignore
    /// use web_sys_ec::{Ec, Wait};
    ///
    /// Wait(1).until(Ec::LocalStorageAttributeValueIs("key", "value"));
    /// ```
    #[inline]
    pub fn LocalStorageAttributeValueIs(
        attr: impl Into<String>,
        value: impl Into<String>,
    ) -> inner::Ec {
        inner::Ec::LocalStorageAttributeValueIs(attr.into(), value.into())
    }

    /// The `window.location.search` is equal to the given value.
    ///
    /// ```rust,ignore
    /// use web_sys_ec::{Ec, Wait};
    ///
    /// Wait(1).until(Ec::LocationSearchIs("?key=value"));
    /// ```
    #[inline]
    pub fn LocationSearchIs(value: impl Into<String>) -> inner::Ec {
        inner::Ec::LocationSearchIs(value.into())
    }
}
