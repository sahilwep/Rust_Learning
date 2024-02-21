use std::thread;

fn main(){
    let message = String::from("Sahilwep");

    // using the message variable without a move
    let handel = thread::spawn( move || {
        println!("{}", message);
    });

    handel.join().unwrap();
}