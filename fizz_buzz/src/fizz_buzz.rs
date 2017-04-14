use std::fmt;

pub enum FizzBuzzValue {
    Fizz,
    Buzz,
    FizzBuzz,
    Value(i32),
}

impl fmt::Display for FizzBuzzValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FizzBuzzValue::Fizz => write!(f, "Fizz"),
            FizzBuzzValue::Buzz => write!(f, "Buzz"),
            FizzBuzzValue::FizzBuzz => write!(f, "FizzBuzz"),
            FizzBuzzValue::Value(i) => write!(f, "{}", i),
        }
    }
}
