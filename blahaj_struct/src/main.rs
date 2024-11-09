fn main() {
    let mut blahaj1 = Blahaj {
        active: true,
        email: String::from("blahaj@uwu.com"),
        age: 2,
    };
    blahaj1.email = String::from("blahaj1@uwu.com");

    let blahaj2 = make_blahaj(String::from("blahaj2@uwu.com"), 12);
    let blahaj3 = Blahaj {
        ..blahaj2
    };
    println!("{blahaj3:?}");
    dbg!(&blahaj3);

    let _black = Color(0, 0, 0);

    let res = check_condition(true);
    match res {
        Ok(Success) => println!("Okay"),
        Err(Error) => println!("Not okay"),
    }
}

#[derive(Debug)]
struct Blahaj {
    active: bool,
    email: String,
    age: i32,
}

fn make_blahaj(email: String, age: i32) -> Blahaj {
    Blahaj {
        active: true,
        email,
        age,
    }
}

struct Color(i32, i32, i32); // tuple struct


struct Success; // unit like structs
struct Error;

fn check_condition(condition: bool) -> Result<Success, Error> {
    if condition {
        Ok(Success)
    } else {
        Err(Error)
    }
}