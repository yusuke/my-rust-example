use std::fmt::{Debug, Formatter};
use std::process::{ExitCode, Termination};

use crate::example::deep::Panic;

#[test]
fn test_panic() {
    one();
}

fn one() {
    two();
}

fn two() {
    three();
}

fn three() {
    let t = Exit {};
    panic!("fail at three");
}


struct Exit {}

impl Termination for Exit {
    fn report(self) -> ExitCode {
        println!("exit impl termination!!!!!");
        panic!("exit impl termination!!!!!");
        ExitCode::FAILURE
    }
}

#[test]
fn test_termination() {
    let _ = create_exit();
    println!("test")
}

fn create_exit() -> Exit {
    Exit {}
}

fn make_no_panic(msg: String) -> Result<String, Panic> {
    if msg.is_empty() {
        Result::Err(Panic { message: String::from("no message") })
    } else {
        Result::Ok(msg)
    }
}

#[test]
fn test_make_no_panic() {
    let result = make_no_panic(String::from(""));
    let string = result.unwrap();
    assert_eq!(string.is_empty(), false);
}
