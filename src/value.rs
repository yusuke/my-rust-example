use std::fmt::{Debug, Formatter};

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

impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        return self.text.eq(&other.text) && self.original == other.original;
    }
}

impl Eq for Value {}
