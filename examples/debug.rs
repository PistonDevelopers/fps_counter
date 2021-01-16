extern crate fps_counter;

use fps_counter::*;

fn main() {
    let mut a = FPSCounter::default();
    a.tick();
    println!("{:?}", a);
}
