pub struct Fibonacci {
    curr: usize,
    next: usize,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci{curr: 1, next: 1}
    }
}

impl Iterator for Fibonacci {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let new_value = self.curr + self.next;
        self.curr = self.next;
        self.next = new_value;

        Some(self.curr)
    }
}
