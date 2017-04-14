use std::fmt;

enum FizzBuzzValue {
    Value(i32),
    Fizz,
    Buzz,
    FizzBuzz,
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
fn fizz_buzz(i: i32) -> FizzBuzzValue {
    match i {
        i if i % 15 == 0 => FizzBuzzValue::FizzBuzz,
        i if i % 5 == 0 => FizzBuzzValue::Buzz,
        i if i % 3 == 0 => FizzBuzzValue::Fizz,
        _ => FizzBuzzValue::Value(i),
    }
}

fn main() {
    for n in (0..101).map(fizz_buzz) {
        println!("{}", n);
    }
}
