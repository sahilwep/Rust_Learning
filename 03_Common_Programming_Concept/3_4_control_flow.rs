fn main(){
    let x = 21;
    let y = 32;
    compare(x, y);
    println!("");
    loop_type_1(10);
    println!("");
    loop_type_2(11);
    println!("");
    loop_type_3(12);
}


// iterations : 
fn loop_type_1 (i : i32) {
    let mut x = 1;
    loop {
        if x == 11 {
            break;
        }
        println!("{} * {} = {}", i, x, x*i );
        x += 1;
    }
}

fn loop_type_2 (x : i32) {
    let mut i = 1;
    while i <= 10 {
        println!("{} * {} = {}", i, x, x*i );
        i += 1;
    }
}

fn loop_type_3 (x : i32) {
    for i in 1..11 {
        println!("{} * {} = {}", x, i, x*i);
    }
}

// Conditionals : 
fn compare(x: i32, y: i32) {
    if x > y {
        println!("x is Grater");
    } else if x < y {
        println!("x is Lesser");
    } else {
        println!("x is equal");
    }
}

