use std::rc::Rc;
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {

    fn next(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => {
                Some(item)
            },
            Nil => None
        }
    }
}

use List::{
    Cons,
    Nil
};
use std::borrow::{BorrowMut, Borrow};
use std::ops::{Deref, DerefMut};
use std::cell::{RefMut, RefCell};

fn main() {

    let a = Rc::new(Cons(Rc::new(RefCell::new(1)),
                         Rc::new(Nil)));
    let mut b = Cons(Rc::new(RefCell::new(2)), Rc::clone(&a));
    println!("Count after b: {}", Rc::strong_count(&a));
    let  c = Cons(Rc::new(RefCell::new(3)), a.clone()); // 只增加引用计数
    println!("Count after c: {}", Rc::strong_count(&a));
    //

    // for i in  b {
    //     println!("{}", i);
    // }

    let tmp = match &c {
        List::Cons(a, b) => {
            println!("Match: {:?} {:?}", a, b);
        },
        m @ Nil => {print!("Nil, {:?}", m);}

        _ => {print!("WTF")}
    };
    println!("{:?}", std::any::type_name::<List>());

    let bo = Box::new(String::from("sdf"));
    let by = bo.clone();        // 克隆
    // println!("{:?} {:?}", b, c);

    let r = b.next();
    println!("{:?}", r);


}