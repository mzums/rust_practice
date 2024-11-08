fn another_fn(x: &str) {
    println!("{x} blahaj says hi!");
}

fn xyz(x: i32) -> i32 {
    x + 5
}

fn main() {
    println!("First blahaj says hi!");
    another_fn("Second");

    let x = xyz(5);
    println!("{x}");
}
