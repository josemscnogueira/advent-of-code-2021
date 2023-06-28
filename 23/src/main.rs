mod borrow;
mod diagram;

fn main() {
    // Parse map filepath from first argument
    let filepath = std::env::args()
        .nth(1)
        .expect("Filepath for inputs not provided");

    let borrow = borrow::Borrow::parse(&filepath);
    println!(
        "Problem #1: {:?}",
        if let Some(b) = borrow::Borrow::optimize(borrow) {
            Some((b.energy, b.moves))
        } else {
            None
        }
    );
}
