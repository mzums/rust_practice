fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s.clone()); // without clone s wouldn't be valid

    println!("{s}");

    let x = 5;

    makes_copy(x);  // x would move into the function, but i32 is Copy, so it's okay to still use x afterward
    println!("{x}");

    let s1 = gives_ownership(); 
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{s1}, {s3}");

}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {    // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("yours");

    some_string // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    a_string  // a_string is returned and moves out to the calling function
}