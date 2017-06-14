mod hanoi;

use hanoi::Hanoi;

fn main() {
    let mut hanoi = Hanoi::new(11);
    hanoi.run();
    println!("{}\nmove {} times.", hanoi, hanoi.count);
}
