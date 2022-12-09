use crate::value::Value;

pub trait Rule {
    fn available(&self, value: Value) -> bool;
    fn apply(&self, value: Value) -> Value;
}

pub fn traditional_rule(divisor: i32, text: String) -> Box<dyn Rule> {
    return Box::new(AppendingFixedTextOnMultipleOfN { divisor, text });
}

struct AppendingFixedTextOnMultipleOfN {
    divisor: i32,
    text: String,
}

impl Rule for AppendingFixedTextOnMultipleOfN {
    fn available(&self, value: Value) -> bool {
        return value.original % self.divisor == 0;
    }

    fn apply(&self, value: Value) -> Value {
        let mut text: String = String::from(value.text);
        text.push_str(self.text.as_str());
        return Value {
            original: value.original,
            text: text.to_string(),
        };
    }
}

pub struct DefaultRule {}

impl DefaultRule {
    pub fn apply(value: Value) -> Value {
        Value {
            original: value.original,
            text: value.original.to_string(),
        }
    }
    pub const INSTANCE: DefaultRule = DefaultRule {};
}

#[test]
fn test_fixed_rule_available_match_case() {
    let rule = AppendingFixedTextOnMultipleOfN { divisor: 3, text: String::from("foo") };
    let value = Value {
        original: 9,
        text: String::from(""),
    };
    let result = rule.available(value);
    assert_eq!(result, true);
}

#[test]
fn test_fixed_rule_apply_match_case() {
    let rule = traditional_rule(3, String::from("foo"));
    let value = Value {
        original: 9,
        text: String::from(""),
    };
    let result = rule.apply(value);
    let expected = Value { original: 9, text: "foo".to_string() };
    assert_eq!(result, expected);
}

#[test]
fn default_rule_returns_original_and_its_string() {
    let value = Value::original(20);
    let result = DefaultRule::apply(value);
    let expected = Value { original: 20, text: "".to_string() };
    assert_eq!(result, expected);
}
