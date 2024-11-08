fn main() {
    println!("wrapping");
    let a: u8 = 255;
    let b: u8 = 2;
    println!("{}", a.wrapping_add(b));
    println!("{:?}", a.checked_add(b));
    let (result, overflowed) = a.overflowing_add(b);
    println!("{:?}", (result, overflowed));
    println!("{}", a.saturating_mul(b));

    println!("5 + 10 = {}", 5+10);
    println!("5.5-2.3 = {}", 5.5-2.3);
    println!("3.3 * 2.5 = {}", 3.3 * 2.5);
    println!("32.7 / 21.3 = {}", 32.7 / 21.3);
    println!("-5 / 3 = {}", -5 / 3);
    println!("43 % 5 = {}", 43 % 5);

    let tup = (200, 2.4, true);
    let (_, _, z) = tup;
    println!("{}", z);
    println!("{}", tup.1);

    let _arr = [1, 2, 3, 4];

    let _arr: [i32; 4] = [1, 2, 3, 4];

    let _arr = [3; 5];
}
