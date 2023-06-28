mod borrow;
mod diagram;

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for inputs not provided");

    let mut borrow = borrow::Borrow::parse(&filepath);
    println!(
        "Problem #1: {:?}",
        borrow::Borrow::optimize(borrow.clone()).unwrap().energy
    );

    borrow.unfold();
    println!(
        "Problem #2: {:?}",
        borrow::Borrow::optimize(borrow).unwrap().energy
    );
}
