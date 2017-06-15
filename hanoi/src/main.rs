mod hanoi;

use hanoi::Hanoi;

fn main() {
    let mut hanoi = Hanoi::new(11);
    hanoi.snapshot(20);
    println!("{}", hanoi);
}
