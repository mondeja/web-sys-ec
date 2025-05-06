use crate::{By, Ec, WaitOptions, Waiter};
use gloo_utils::document;
use std::boxed::Box;
use web_sys::wasm_bindgen::JsCast;

#[doc(hidden)]
#[derive(Debug)]
pub struct Condition {
    by: Option<By>,
    ec: Ec,
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

#[doc(hidden)]
pub async fn until_impl(condition: Condition, waiter: Waiter)
{
    let conditioner = Conditioner {
        condition,
        waiter,
        until_negative: false,
    };
    match condition.ec {
        Ec::InnerTextContains(_) => {
            conditioner.wait_for_object::<web_sys::HtmlElement>().await
        }
    }
}

#[derive(Debug)]
pub(crate) struct Conditioner {
    condition: Condition,
    waiter: Waiter,
    until_negative: bool,
}

impl Conditioner {
    pub(crate) async fn wait_for_object<T>(&self)
    where
        T: 'static + web_sys::wasm_bindgen::JsCast,
    {
        let by = self.condition.by.as_ref().unwrap();
        let waiter_fn: Box<dyn Fn() -> Option<T>> = match by {
            By::Id(id) => Box::new(move || {
                let maybe_element =
                    document().get_element_by_id(&id.to_string());
                if let Some(element) = maybe_element {
                    if let Ok(element) = element.dyn_into::<T>() {
                        Some(element)
                    } else {
                        // TODO: here panic, doesn't casts to expected minimum type
                        None
                    }
                } else {
                    None
                }
            }),
            By::Class(class) => Box::new(move || {
                let maybe_element = document()
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
            }),
            By::TagName(tag_name) => Box::new(move || {
                let maybe_element = document()
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
            }),
            By::QuerySelector(selector) => Box::new(move || {
                let maybe_element = document().query_selector(&selector.to_string());
                if let Ok(Some(element)) = maybe_element {
                    if let Ok(element) = element.dyn_into::<T>() {
                        Some(element)
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
        };

        let WaitOptions {
            duration,
            poll_frecuency,
        } = self.waiter.options;

        let start = js_sys::Date::now();
        while js_sys::Date::now() - start < duration.as_millis() as f64 {
            if let Some(ref element) = waiter_fn() {
                if ec_fn(element) {
                    return;
                } else if self.until_negative {
                    return;
                }
            }
            gloo_timers::future::sleep(poll_frecuency).await;
        }

        panic!(
            "Condition not met within the specified duration: {:?}",
            self.waiter.options
        );
    }
}
