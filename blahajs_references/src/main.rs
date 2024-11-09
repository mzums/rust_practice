fn main() {
    let mut s1 = String::from("blahaj");
    let len = calc_len(&mut s1);
    println!("Blahaj has {len} letters!");
}

fn calc_len(s: &mut String) -> usize {
    s.push_str("!");
    s.len()
}