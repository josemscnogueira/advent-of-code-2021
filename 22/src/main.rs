mod reactor;
mod reboot;

use reboot::Reboot;

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for inputs not provided");

    let mut reboot = Reboot::parse(&filepath);
    reboot.retain_by_limit(50);
    reboot.process();
    println!("Problem #1: {}", reboot.n_elems());

    let mut reboot = Reboot::parse(&filepath);
    reboot.process();
    println!("Problem #2: {}", reboot.n_elems());
}
