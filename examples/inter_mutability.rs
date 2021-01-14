use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(RefCell<i32>, Rc<List>),
    Nil
}

use List::{
    Cons,
    Nil
};

fn main() {
    let val = RefCell::new(5);
    let a = Cons(RefCell::clone(&val), Rc::new(Nil));
    let b = Cons(RefCell::new(4), Rc::clone(&a));
    let c = Cons(RefCell::new(3), Rc::clone(&a));

    *val.borrow_mut() += 5;
    match a {
        Cons(v, l) => {
            *v.borrow_mut += 5;
        }
        Nil => {}
    }

    println!("{:?} {:?} {:?} {:?}", a, b, c, val);
}