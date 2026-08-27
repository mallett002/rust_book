// Integration tests go at root of project in "tests" dir.
// To run all tests in this file:
// cargo test --test <file_name>
//
// we have tests/common/mod.rs for setup
// This "common" dir could be named anything, but file name must be mod.rs
use adder::add_two;
mod common;

#[test]
fn it_adds_two() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}
