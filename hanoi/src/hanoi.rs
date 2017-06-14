use std::fmt;

pub struct Hanoi {
    pub n: usize,
    pub count: usize,
    piles: [Pile; 3],
}

type Pile = Vec<usize>;

impl Hanoi {
    pub fn new(n: usize) -> Self {
        let mut v = Vec::new();
        for i in (0..n).rev() {
            v.push(i);
        }
        Hanoi { n: n, count: 0, piles: [v, Vec::new(), Vec::new()] }
    }

    fn move_to(&mut self, from: usize, to: usize) {
        if let Some(value) = self.piles[from].pop() {
            self.piles[to].push(value);
            println!("{}", self);
            self.count += 1;
        }
    }

    fn hanoi(&mut self, n: usize, from: usize, to: usize, work: usize) {
        if n > 0 {
            self.hanoi(n - 1, from, work, to);
            self.move_to(from, to);
            self.hanoi(n - 1, work, to, from);
        }
    }

    pub fn run(&mut self) {
        let n = self.n;
        self.hanoi(n, 0, 1, 2);
    }
}

impl fmt::Display for Hanoi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in (0..self.n).rev() {
            for j in 0..3 {
                if i < self.piles[j].len() {
                    write!(f, "{}", self.piles[j][i]).unwrap();
                } else {
                    write!(f, " ").unwrap();
                }
                write!(f, "   ").unwrap();
            }
            write!(f, "\n").unwrap();
        }
        writeln!(f, "a   b   c")
    }
}
