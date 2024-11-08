fn main() {
    let num = 5;

    if num == 2 {
        println!("num is 2");
    }
    else if num == 3 {
        println!("num is 3");
    }
    else {
        println!("num is sth else");
    }

    let condition = true;
    let xyz = if condition { "yes" } else { "no" };
    println!("{xyz}");

    let mut cnt = 0;

    let res = loop {
        cnt += 1;

        if cnt == 10 {
            break cnt * 2;
        }
    };
    println!("The result is {res}\n");

    cnt = 0;

    'counting_up: loop {
        println!("count = {cnt}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if cnt == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        cnt += 1;
    }
    println!("End count = {cnt}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("!!!");

    let arr = [10, 20, 30, 40, 50];

    for el in arr {
        println!("{el}");
    }
    for el in (1..4).rev() {
        println!("{el}");
    }
}
