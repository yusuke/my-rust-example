use std::fmt::{Debug, Display, Formatter};

pub struct Value {
    pub original: i32,
    pub text: String,
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Value { original:").unwrap();
        f.write_str(self.original.to_string().as_str()).unwrap();
        f.write_str(",text:'").unwrap();
        f.write_str(self.text.as_str()).unwrap();
        return f.write_str("'}");
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value[original={}, text='{}']", self.original, self.text)
    }
}

impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        return self.text.eq(&other.text) && self.original == other.original;
    }
}

impl Eq for Value {}

#[test]
fn test_debug_value_failure() {
    let value = Value { original: 0, text: String::from("") };
    let string = value.to_string();
    assert_eq!(string, "foo-bar");
}

