
use candid::types::number::Nat;

use std::cell::RefCell;
use std::borrow::Borrow;

thread_local! {
    static MSG: RefCell<Vec<String>> = RefCell::new(Vec::new());
}
#[ic_cdk::update]
fn send_msg(in_msg: String) {
    MSG.with(|msg| msg.borrow_mut().push(in_msg));
}

#[ic_cdk::query]
fn get_chat() -> Vec<String> {
    MSG.with(|msg| (*msg.borrow()).clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
