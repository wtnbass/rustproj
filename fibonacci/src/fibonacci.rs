pub struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new_value = self.curr + self.next;
        self.curr = self.next;
        self.next = new_value;

        Some(self.curr)
    }
}

pub fn fib() -> Fibonacci {
    Fibonacci{curr: 1, next: 1}
}
