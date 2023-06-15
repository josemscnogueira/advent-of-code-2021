mod scanner;
mod utils;

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for inputs not provided");

    let mut scanners = scanner::Scanner::parse(&filepath);
    scanner::generate_links(&mut scanners);
    scanner::normalize_links(&mut scanners);

    let beacons = scanner::join_beacons(&scanners);
    println!("Problem #1: {}", beacons.len());
    println!(
        "Problem #2: {}",
        scanner::max_manhattan_distance(&scanners).unwrap()
    );
}
