use std::fmt::{Debug, Formatter};

pub struct Panic {
    pub(crate) message: String,
}

impl Debug for Panic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Panic {}", self.message)
    }
}

#[test]
fn test_panic() {
    panic!("test");
}

#[test]
fn test_foo() {
    assert_ne!("foo", "foo");
}

#[test]
fn test_bar() {
    let string = String::from("bar");
    assert!(string.len() < "bar".to_string().len());
}
