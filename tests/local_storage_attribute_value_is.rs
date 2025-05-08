use gloo_timers::callback::Timeout;
use gloo_utils::window;
use wasm_bindgen_test::*;
use web_sys_ec::{Ec, Wait};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub async fn local_storage_attribute_value_is() {
    let _tm = Timeout::new(5, move || {
        window()
            .local_storage()
            .unwrap()
            .unwrap()
            .set_item("language", "es")
            .unwrap();
    });

    Wait(0.2)
        .until(Ec::LocalStorageAttributeValueIs("language", "es"))
        .await;

    let _tm = Timeout::new(10, move || {
        window()
            .local_storage()
            .unwrap()
            .unwrap()
            .remove_item("language")
            .unwrap();
    });

    Wait(0.3)
        .until_not(Ec::LocalStorageAttributeValueIs("language", "es"))
        .await;
}
