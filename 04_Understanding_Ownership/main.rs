fn main() {
    let reference_to_noting = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    s  // ownership moved from this scope.
}