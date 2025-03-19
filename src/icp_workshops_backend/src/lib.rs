
use candid::types::number::Nat;

use std::cell::RefCell;
use std::borrow::Borrow;

thread_local! {
    static MSG: RefCell<String> = RefCell::new(String::new());
}
#[ic_cdk::update]
fn set_name(name: String) {
    MSG.with(|msg| *msg.borrow_mut()=name);
}

#[ic_cdk::query]
fn get_name() -> String {
    MSG.with(|msg| (*msg.borrow()).clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
