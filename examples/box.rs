#![allow(clippy::print_literal)]

use core::fmt::{ Debug };

/*-- same as print!("\n  sp = {:?}", sp) --*/
fn show<T:Debug>(name:&str, t:&T) {
    print!("\n  {} = {:?}", name, t);
}

#[derive(Debug)]
#[derive(Default)]
struct Test {
    i: u32
}


impl Test {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn value(&self) -> u32 {
        self.i
    }
}

trait Draw {
    fn draw(&self) {
        print!("Draw");
    }
    fn newer();
}

impl Draw for Test {
    fn newer() {
        print!("new draw");
    }
}

struct Node<'a> {
    v: i32,
    next: &'a Node<'a>,
}

impl <'a> Node<'a> {
    fn new() -> Node<'a> {
        Self {
            v: 0,
            next
        }
    }
}

fn main() {

    let t = Test::new();
    print!("{:?} {}", t, t.value());
    t.draw();
    Test::newer();

    let putline = || print!("\n");
    let bv = Box::new("Bruce");
    print!("{}", bv);

    let n = Node::new();

}