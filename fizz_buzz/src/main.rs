mod fizz_buzz;

use fizz_buzz::FizzBuzzValue;

fn main() {
    for n in (0..101).map(fizz_buzz) {
        println!("{}", n);
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
