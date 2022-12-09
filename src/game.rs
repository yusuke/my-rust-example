use vararg::vararg;

use crate::rule::Rule;
use crate::value::Value;

#[warn(unused_macros)]
#[vararg]
pub fn traditional_game<const L: usize>(rules: &[Box<dyn Rule>; L]) -> Box<dyn Game> {
    todo!()
}

trait Game {
    fn apply(&self, values: &Vec<i32>) -> Vec<Value>;
}

struct DefaultGame {
    rules: [Box<dyn Rule>],
}

impl Game for DefaultGame {
    fn apply(&self, values: &Vec<i32>) -> Vec<Value> {
        let mut result = Vec::with_capacity(values.capacity());
        for (index, value) in values.into_iter().enumerate() {
            result[index] = Value::original(*value)
        }
        result
    }
}

#[test]
fn test_traditional_game() {
    let rules: [Box<dyn Rule>; 0] = [];
    let _game = traditional_game(&rules);
}
