mod hanoi;

use hanoi::Hanoi;

fn main() {
    let mut hanoi = Hanoi::new(11);
    hanoi.snapshot(20);
    println!("{}", hanoi);

    let mut hanoi2 = Hanoi::new(11);
    hanoi2.snapshot_v2(20);
    println!("{}", hanoi2);
}
