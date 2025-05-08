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
