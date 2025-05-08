use gloo_timers::callback::Timeout;
use gloo_utils::document;
use wasm_bindgen_test::*;
use web_sys::{wasm_bindgen::JsCast, HtmlElement};
use web_sys_ec::{Ec, Wait};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub async fn inner_text_contains() {
    let _tm = Timeout::new(5, move || {
        let el = document()
            .create_element("inner_text_contains")
            .unwrap()
            .unchecked_into::<HtmlElement>();
        el.set_inner_text("Select a language:");
        document().body().unwrap().append_child(&el).unwrap();
    });

    Wait(0.2)
        .until((
            "inner_text_contains",
            Ec::InnerTextContains("Select a language:"),
        ))
        .await;

    let _tm = Timeout::new(10, move || {
        let el = document()
            .get_elements_by_tag_name("inner_text_contains")
            .item(0)
            .unwrap()
            .unchecked_into::<HtmlElement>();
        el.set_inner_text("Selecciona un idioma:");
        document().body().unwrap().remove_child(&el).unwrap();
    });

    Wait(0.3)
        .until_not((
            "inner_text_contains",
            Ec::InnerTextContains("Select a language:"),
        ))
        .await;
}

#[wasm_bindgen_test]
pub async fn attribute_value_is() {
    let _tm = Timeout::new(5, move || {
        let el = document().create_element("attribute_value_is").unwrap();
        el.set_attribute("lang", "es").unwrap();
        document().body().unwrap().append_child(&el).unwrap();
    });

    Wait(0.2)
        .until(("attribute_value_is", Ec::AttributeValueIs("lang", "es")))
        .await;

    let _tm = Timeout::new(10, move || {
        let el = document()
            .query_selector("attribute_value_is[lang='es']")
            .unwrap()
            .unwrap();
        el.set_attribute("lang", "es1").unwrap();
        document().body().unwrap().remove_child(&el).unwrap();
    });

    Wait(0.3)
        .until_not(("attribute_value_is", Ec::AttributeValueIs("lang", "es")))
        .await;
}
