fn main() {
    let x = 42;

    // FnOnce - consumes x
    let closure_once = move || {
        println!("x: {}", x);
    };

    // FnMut - borrows x mutably
    let mut y = 10;
    let mut closure_mut = || {
        y += 1;
        println!("y: {}", y);
    };

    // Fn - borrows x immutably
    let closure_imm = || {
        println!("x: {}", x);
    };

    // Calling closures
    closure_once(); // This moves x into the closure, can't use x anymore
    closure_mut();  // This borrows y mutably
    closure_imm();  // This borrows x immutably
}
