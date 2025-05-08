use gloo_timers::callback::Timeout;
use gloo_utils::document;
use wasm_bindgen_test::*;
use web_sys_ec::{By, Wait};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub async fn id() {
    let _tm = Timeout::new(5, move || {
        let el = document().create_element("p").unwrap();
        el.set_attribute("id", "test1").unwrap();
        document().body().unwrap().append_child(&el).unwrap();
    });

    Wait(0.2).until(By::Id("test1")).await;

    let _tm = Timeout::new(10, move || {
        let el = document().query_selector("#test1").unwrap().unwrap();
        document().body().unwrap().remove_child(&el).unwrap();
    });

    Wait(0.3).until_not("#test1").await;
}

#[wasm_bindgen_test]
pub async fn class() {
    let _tm = Timeout::new(5, move || {
        let el = document().create_element("p").unwrap();
        el.set_attribute("class", "test2").unwrap();
        document().body().unwrap().append_child(&el).unwrap();
    });

    Wait(0.2).until(By::Class("test2")).await;

    let _tm = Timeout::new(10, move || {
        let el = document().query_selector(".test2").unwrap().unwrap();
        document().body().unwrap().remove_child(&el).unwrap();
    });

    Wait(0.3).until_not(".test2").await;
}

#[wasm_bindgen_test]
pub async fn tag_name() {
    let _tm = Timeout::new(5, move || {
        let el = document().create_element("test3").unwrap();
        document().body().unwrap().append_child(&el).unwrap();
    });

    Wait(0.2).until(By::TagName("test3")).await;

    let _tm = Timeout::new(10, move || {
        let el = document().query_selector("test3").unwrap().unwrap();
        document().body().unwrap().remove_child(&el).unwrap();
    });

    Wait(0.3).until_not("test3").await;
}

#[wasm_bindgen_test]
pub async fn query_selector() {
    let _tm = Timeout::new(5, move || {
        let el = document().create_element("test4").unwrap();
        el.set_attribute("class", "test4").unwrap();
        el.set_attribute("id", "test4").unwrap();
        document().body().unwrap().append_child(&el).unwrap();
    });

    Wait(0.2)
        .until(By::QuerySelector("test4.test4#test4"))
        .await;

    let _tm = Timeout::new(10, move || {
        let el = document()
            .query_selector("test4.test4#test4")
            .unwrap()
            .unwrap();
        document().body().unwrap().remove_child(&el).unwrap();
    });

    Wait(0.3).until_not("test4").await;
}
