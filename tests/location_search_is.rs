use gloo_timers::callback::Timeout;
use gloo_utils::window;
use wasm_bindgen_test::*;
use web_sys_ec::{Ec, Wait};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub async fn location_search_is() {
    let _tm = Timeout::new(5, move || {
        let url = web_sys::Url::new(
            &window()
                .location()
                .href()
                .expect("Failed to get location.href from the browser"),
        )
        .expect("Failed to parse location.href from the browser");
        url.search_params().set("key", "value");
        window()
            .history()
            .expect("Failed to get the history from the browser")
            .replace_state_with_url(&web_sys::wasm_bindgen::JsValue::NULL, "", Some(&url.href()))
            .expect("Failed to replace the history state");
    });

    Wait(0.2).until(Ec::LocationSearchIs("?key=value")).await;

    let _tm = Timeout::new(10, move || {
        let url = web_sys::Url::new(
            &window()
                .location()
                .href()
                .expect("Failed to get location.href from the browser"),
        )
        .expect("Failed to parse location.href from the browser");
        url.search_params().delete("key");
        window()
            .history()
            .expect("Failed to get the history from the browser")
            .replace_state_with_url(&web_sys::wasm_bindgen::JsValue::NULL, "", Some(&url.href()))
            .expect("Failed to replace the history state");
    });

    Wait(0.3)
        .until_not(Ec::LocationSearchIs("?key=value"))
        .await;
}
