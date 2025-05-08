use crate::{by::inner::By, ec::inner::Ec, Waiter as Wait};
use std::boxed::Box;
use web_sys::wasm_bindgen::JsCast;

#[derive(Debug)]
pub(crate) struct Condition {
    pub(crate) by: Option<By>,
    pub(crate) ec: Ec,
}

impl From<(By, Ec)> for Condition {
    fn from((by, ec): (By, Ec)) -> Self {
        Condition { by: Some(by), ec }
    }
}

impl From<Ec> for Condition {
    fn from(ec: Ec) -> Self {
        Condition { by: None, ec }
    }
}

pub(crate) async fn until_impl(
    condition: Condition,
    wait: Wait,
    #[cfg(feature = "nightly")] caller_location: &std::panic::Location<'static>,
) {
    Conditioner {
        condition,
        wait,
        until_negative: false,
        #[cfg(feature = "nightly")]
        caller_location: *caller_location,
    }
    .resolve()
    .await;
}

#[derive(Debug)]
pub(crate) struct Conditioner {
    condition: Condition,
    wait: Wait,
    until_negative: bool,
    #[cfg(feature = "nightly")]
    caller_location: std::panic::Location<'static>,
}

impl Conditioner {
    pub(crate) async fn resolve(&self) {
        match self.condition.ec {
            Ec::InnerTextContains(_) => self.wait_for_object::<web_sys::HtmlElement>().await,
            Ec::AttributeValueIs(_, _) => self.wait_for_object::<web_sys::Element>().await,
            Ec::LocalStorageAttributeValueIs(_, _) => {
                self.wait_for_object::<web_sys::Storage>().await;
            }
            Ec::LocationSearchIs(_) => self.wait_for_object::<web_sys::Location>().await,
        }
    }

    pub(crate) async fn wait_for_object<T>(&self)
    where
        T: 'static + web_sys::wasm_bindgen::JsCast,
    {
        let waiter_fn: Box<dyn Fn() -> Option<T>> = match self.condition.by.as_ref() {
            None => Box::new(move || match self.condition.ec {
                Ec::LocalStorageAttributeValueIs(_, _) => {
                    if let Some(window) = web_sys::window() {
                        if let Ok(Some(local_storage)) = window.local_storage() {
                            if let Ok(storage) = local_storage.dyn_into::<T>() {
                                Some(storage)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Ec::LocationSearchIs(_) => {
                    if let Some(window) = web_sys::window() {
                        if let Ok(location) = window.location().dyn_into::<T>() {
                            Some(location)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                _ => unreachable!(),
            }),
            Some(By::Id(id)) => Box::new(move || {
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(element) = document.get_element_by_id(&id.to_string()) {
                            if let Ok(element) = element.dyn_into::<T>() {
                                Some(element)
                            } else {
                                // TODO: here panic, doesn't casts to expected minimum type
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }),
            Some(By::Class(class)) => Box::new(move || {
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        let maybe_element = document
                            .get_elements_by_class_name(&class.to_string())
                            .item(0);
                        if let Some(element) = maybe_element {
                            if let Ok(element) = element.dyn_into::<T>() {
                                Some(element)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }),
            Some(By::TagName(tag_name)) => Box::new(move || {
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        let maybe_element = document
                            .get_elements_by_tag_name(&tag_name.to_string())
                            .item(0);
                        if let Some(element) = maybe_element {
                            if let Ok(element) = element.dyn_into::<T>() {
                                Some(element)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }),
            Some(By::QuerySelector(selector)) => Box::new(move || {
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        let maybe_element = document.query_selector(&selector.to_string());
                        if let Ok(Some(element)) = maybe_element {
                            if let Ok(element) = element.dyn_into::<T>() {
                                Some(element)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }),
        };

        let ec_fn: Box<dyn Fn(&T) -> bool> = match self.condition.ec {
            Ec::InnerTextContains(ref text) => Box::new(move |element: &T| {
                let element = element.unchecked_ref::<web_sys::HtmlElement>();
                let inner_text = element.inner_text();
                inner_text.contains(text)
            }),
            Ec::AttributeValueIs(ref attribute, ref value) => Box::new(move |element: &T| {
                let element = element.unchecked_ref::<web_sys::Element>();
                let attribute_value = element.get_attribute(attribute);
                if let Some(attribute_value) = attribute_value {
                    attribute_value == *value
                } else {
                    false
                }
            }),
            Ec::LocalStorageAttributeValueIs(ref attribute, ref value) => {
                Box::new(move |storage: &T| {
                    let storage = storage.unchecked_ref::<web_sys::Storage>();
                    let attribute_value = storage.get_item(attribute);
                    if let Ok(Some(attribute_value)) = attribute_value {
                        attribute_value == *value
                    } else {
                        false
                    }
                })
            }
            Ec::LocationSearchIs(ref value) => Box::new(move |location: &T| {
                let location = location.unchecked_ref::<web_sys::Location>();
                if let Ok(search) = location.search() {
                    search == *value
                } else {
                    false
                }
            }),
        };

        let wait_options = &self.wait.options;
        let duration = wait_options.duration();
        let poll_frecuency = wait_options.poll_frecuency();

        let mut number_of_attempts = 1;
        let start = js_sys::Date::now();
        while js_sys::Date::now() - start < duration.as_millis() as f64 {
            if let Some(ref element) = waiter_fn() {
                let expected_condition_match = ec_fn(element);
                if (self.until_negative && !expected_condition_match)
                    || (!self.until_negative && expected_condition_match)
                {
                    return;
                }
            }
            number_of_attempts += 1;
            gloo_timers::future::sleep(poll_frecuency).await;
        }

        panic!(
            concat!(
                "\n",
                "Expected condition has not been met in the given time:\n",
                "{}",
                "{}",
                "  - Duration: {:?}\n",
                "  - Poll frecuency: {:?}\n",
                "  - Number of attempts: {}\n",
            ),
            {
                #[cfg(feature = "nightly")]
                let caller = format!("  - Caller: {}\n", self.caller_location);
                #[cfg(not(feature = "nightly"))]
                let caller = String::new();
                caller
            },
            {
                let mut display = String::new();
                if let Some(ref by) = self.condition.by {
                    display.push_str(&format!("  - Selector: {by}\n"));
                }
                display.push_str(&format!("  - Condition: {}\n", self.condition.ec));
                display
            },
            duration,
            poll_frecuency,
            number_of_attempts,
        );
    }
}
