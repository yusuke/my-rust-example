extern crate core;

mod rule;
mod value;
mod game;
mod panic;
mod example;

fn main() {
    let number: i32 = 1 + 1;
    println!("Hello, world!");
    println!("{}", number);
    for i in 0..32 {
        let value = fizz_buzz(i);
        println!("value: {}", value)
    }
}

fn fizz_buzz(int: i32) -> String {
    let mut result = String::new();
    if int % 3 == 0 {
        result.push_str("Fizz");
    }
    if int % 5 == 0 {
        result.push_str("Buzz");
    }
    if result.is_empty() {
        result.push_str(int.to_string().as_str());
    }
    return result.to_string();
}

#[test]
fn test_fizz_buzz_2() {
    assert_eq!(fizz_buzz(2), "2");
}

#[test]
fn test_fizz_buzz_3() {
    assert_eq!(fizz_buzz(3), "Fizz");
    assert_eq!(fizz_buzz(6), "Fizz");
}

#[test]
fn test_fizz_buzz_5() {
    assert_eq!(fizz_buzz(5), "Buzz");
    assert_eq!(fizz_buzz(10), "Buzz");
}

#[test]
fn test_fizz_buzz_15() {
    assert_eq!(fizz_buzz(15), "FizzBuzz");
}
