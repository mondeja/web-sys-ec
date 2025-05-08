use crate::{by::inner::By, ec::inner::Ec, Condition, Waiter as Wait};
use std::boxed::Box;
use web_sys::wasm_bindgen::JsCast;

pub(crate) async fn until_impl(
    condition: Condition,
    wait: Wait,
    #[cfg(feature = "nightly")] caller_location: &std::panic::Location<'static>,
) {
    Conditioner {
        condition,
        wait,
        negative_until: false,
        #[cfg(feature = "nightly")]
        caller_location: *caller_location,
    }
    .resolve()
    .await;
}

pub(crate) async fn until_not_impl(
    condition: Condition,
    wait: Wait,
    #[cfg(feature = "nightly")] caller_location: &std::panic::Location<'static>,
) {
    Conditioner {
        condition,
        wait,
        negative_until: true,
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
    negative_until: bool,
    #[cfg(feature = "nightly")]
    caller_location: std::panic::Location<'static>,
}

impl Conditioner {
    pub(crate) async fn resolve(&self) {
        match self.condition.ec {
            None => match self.condition.by.as_ref() {
                None => {
                    // TODO: better error message
                    panic!("Expected condition is not set");
                }
                Some(_) => self.wait_for_object::<web_sys::Node>().await,
            },
            Some(Ec::InnerTextContains(_)) => self.wait_for_object::<web_sys::HtmlElement>().await,
            Some(Ec::AttributeValueIs(_, _)) => self.wait_for_object::<web_sys::Element>().await,
            Some(Ec::LocalStorageAttributeValueIs(_, _)) => {
                self.wait_for_object::<web_sys::Storage>().await;
            }
            Some(Ec::LocationSearchIs(_)) => self.wait_for_object::<web_sys::Location>().await,
        }
    }

    pub(crate) async fn wait_for_object<T>(&self)
    where
        T: 'static + web_sys::wasm_bindgen::JsCast,
    {
        let waiter_fn: Box<dyn Fn() -> Option<T>> = match self.condition.by.as_ref() {
            None => Box::new(move || match self.condition.ec {
                Some(Ec::LocalStorageAttributeValueIs(_, _)) => {
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
                Some(Ec::LocationSearchIs(_)) => {
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
            None => Box::new(move |_| true),
            Some(Ec::InnerTextContains(ref text)) => Box::new(move |element: &T| {
                let element = element.unchecked_ref::<web_sys::HtmlElement>();
                let inner_text = element.inner_text();
                inner_text.contains(text)
            }),
            Some(Ec::AttributeValueIs(ref attribute, ref value)) => Box::new(move |element: &T| {
                let element = element.unchecked_ref::<web_sys::Element>();
                let attribute_value = element.get_attribute(attribute);
                if let Some(attribute_value) = attribute_value {
                    attribute_value == *value
                } else {
                    false
                }
            }),
            Some(Ec::LocalStorageAttributeValueIs(ref attribute, ref value)) => {
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
            Some(Ec::LocationSearchIs(ref value)) => Box::new(move |location: &T| {
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
                if (self.negative_until && !expected_condition_match)
                    || (!self.negative_until && expected_condition_match)
                {
                    return;
                }
            } else if self.negative_until {
                return;
            }
            number_of_attempts += 1;
            gloo_timers::future::sleep(poll_frecuency).await;
        }

        // TODO:
        //   - If there is a `By` selector and the until is positive,
        //     try to match the element with the selector and print it
        //     in the panic message either it is found or not.
        //   - If there is a `By` selector and the until is negative,
        //     try to match the element with the selector and print it
        //     in the panic message either it is found or not.
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
                if let Some(ref ec) = self.condition.ec {
                    display.push_str(&format!("  - Condition: {ec}\n"));
                }
                display
            },
            duration,
            poll_frecuency,
            number_of_attempts,
        );
    }
}
