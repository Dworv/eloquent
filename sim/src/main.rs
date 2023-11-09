use sim::{Speeds, Slot};

fn main() {
    let speeds = Speeds::init();
    println!("speed: {:?}", speeds.time(
        Slot::new(1, 1),
        Slot::new(2, 1)
    ));
}
