mod fibonacci;

use fibonacci::fib;

fn main() {
    for n in fib().take(45) {
        println!("{}", n);
    }
}
