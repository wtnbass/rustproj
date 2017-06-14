mod fibonacci;

use fibonacci::Fibonacci;

fn main() {
    let fib = Fibonacci::new();
    for n in fib.take(40) {
        println!("{}", n);
    }
}
