use std::sync::mpsc::{
    self,
    Sender,
    Receiver
};
use std::thread;

fn main() {
    let (sx,rx) = mpsc::channel::<i32>();
    thread::spawn(move||{
        sx.send(12).unwrap();
    });
    io::spawn();
    println!("Receive: {}", rx.recv().unwrap());
}