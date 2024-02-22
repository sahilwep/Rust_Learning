use std::thread;
use std::sync::mpsc;

fn main(){
    // main thread start here
    // create a new channel
    let (sender, receiver) = mpsc::channel();

    // spawn a new thread
    let handel = thread::spawn( move || {
        // receive message from channel
        let message = receiver.recv().unwrap();

        println!("Received message : {}", message);
    });

    let message = String::from("Sahilwep");
    // send message to channel
    sender.send(message).unwrap();

    // wait for spawned thread to finish
    handel.join().unwrap();
}