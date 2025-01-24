#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen;
extern crate wasm_bindgen_test;
extern crate alloc;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use js_sys::{Promise};
use alloc::rc::Rc;
use core::future::Future;
use core::pin::Pin;
use core::task::{self, Poll};
use core::cell::{Cell};


pub struct State {
    /// Include ignored tests.
    include_ignored: Cell<bool>,

}

fn example() -> Promise {
    let state = Rc::new(State {
        include_ignored: Default::default(),
    });
    return future_to_promise(async {
        let passed = ExecuteTests(state).await;
        Ok(JsValue::from(passed))
    })
}

struct ExecuteTests(Rc<State>);

impl Future for ExecuteTests {
    type Output = bool;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<bool> {
        Poll::Ready(true)
    }
}

#[wasm_bindgen]
pub fn works() {
    let res = example();
}
