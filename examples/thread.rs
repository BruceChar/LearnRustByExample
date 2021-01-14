use std::rc::Rc;
use std::time::Duration;
use std::thread;
use std::sync::Arc;

fn main1() {
    // let rc = Rc::new("this is not safe");
    // std::thread::spawn(move||{println!("I have an rc with: {}", rc);});

    let arc = Arc::new("this is safe");
    thread::spawn(move|| {
        println!("I have an arc with: {}", arc);
    });

    let mut v = vec![1,3];
    thread::spawn(move||{
        v.push(4);
        println!("{:?}", v);
    });
    // let _ = v.pop();
    thread::sleep(Duration::from_secs(1));
}


fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = numbers;//Arc::new(numbers); // TODO
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = shared_numbers.clone();
        joinhandles.push(thread::spawn(move || {
            let mut i = offset;
            let mut sum = 0;
            while i < child_numbers.len() {
                sum += child_numbers[i];
                i += 8;
            }
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
    thread::sleep(Duration::from_secs(1));
}