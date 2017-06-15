use std::fmt;

pub struct Hanoi {
    n: usize,
    count: usize,
    max: usize,
    piles: [Pile; 3],
}

type Pile = Vec<usize>;

fn mersenne(n: usize) -> usize {
    let mrsn = 2i32.pow(n as u32) - 1;
    mrsn as usize
}

impl Hanoi {
    pub fn new(n: usize) -> Self {
        let max = mersenne(n);
        let mut piles = [ Vec::new(), Vec::new(), Vec::new() ];

        // Add disks to the first pile.
        for i in (0..n).rev() {
            piles[0].push(i);
        }

        Hanoi { n: n, count: 0, max: max, piles: piles }
    }

    fn move_to(&mut self, from: usize, to: usize) {
        if let Some(value) = self.piles[from].pop() {
            self.piles[to].push(value);
            self.count += 1;
        }
    }

    fn hanoi(&mut self, n: usize, from: usize, to: usize, work: usize) {
        if n > 0 {
            self.hanoi(n - 1, from, work, to);

            if self.max <= self.count {
                return
            }
            self.move_to(from, to);
            println!("{}", self);

            self.hanoi(n - 1, work, to, from);
        }
    }

    pub fn snapshot(&mut self, stop_count: usize) {
        let n = self.n;
        self.max = stop_count;
        self.hanoi(n, 0, 1, 2);
    }
}

impl fmt::Display for Hanoi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..3 {
            let p = &self.piles[i];
            write!(f, "{}:", i).unwrap();
            for j in 0..p.len() {
                write!(f, " {}", p[j]).unwrap();
            }
            writeln!(f, "").unwrap();
        }
        writeln!(f, "move: {} times", self.count)
    }
}
