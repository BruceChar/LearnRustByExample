use std::fmt::Debug;
use std::mem::size_of;
use std::ops::{Deref, DerefMut};
use std::alloc::handle_alloc_error;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

trait Size {
    fn size(&self) -> usize;
}

trait Show: Debug {
    fn show(&self) {
        println!("{:?}", self);
    }
}


#[derive(Debug, Copy, Clone)]
struct Test {
    x: i32,
    y: f32
}

impl Size for Test {
    fn size(&self) -> usize {
        std::mem::size_of::<Test>()
    }
}

impl Show for Test {}

impl Test {
    fn new() -> Self {
        Self {
            x: 42,
            y:1.5
        }
    }
}

fn demo_ref<T>(t:&T) where T:Debug {
    show_type(t);
}

fn show_type<T:Debug>(_v: &T) {
    let name = std::any::type_name::<T>();
    println!("TypeID: {:?}, size: {:?}", name, size_of::<T>())
}

struct Counter<T> {
    v: T,
}

impl <T> Iterator for Counter<T>
where T: Copy
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.v)
    }
}
struct Mybox<T> (T);

impl<T> Mybox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }

    fn deref(&self) -> &T {
        &self.0
    }
}

impl <T> Drop for Mybox<T>
// where T: Copy
{
    fn drop(&mut self) {
        // todo!();
    }
}
impl <T> Deref for Mybox<T>{
    type Target = T;
    fn deref(&self) -> &T {
         &self.0
    }
}

// impl <T>DerefMut for Mybox<T> {
//     // type Target = T;
//
//     fn deref_mut(&mut self) -> &mut T {
//         &mut self.0
//     }
// }

fn hello(str: &str) {
    println!("Hello {}", str);
}

#[derive(Debug, Eq,)]
struct MyStruct {
    val: u32
}

impl PartialEq for MyStruct {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl PartialOrd for MyStruct {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Hash for MyStruct {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.val.hash(hasher);
    }
}

impl PartialEq<u32> for MyStruct {
    fn eq(&self, other: &u32) -> bool {
        self.val == *other
    }
}

impl PartialOrd<u32> for MyStruct {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        self.val.partial_cmp(other)
    }
}



fn main() {
    // let t = Test::new();
    // let tp = t;
    // let t = tp;
    // tp.show();
    // t.show();
    // println!("{}", t.size());
    //
    // demo_ref(&t);
    // show_type(&t);
    //
    // let mut c = Counter {v: 3};
    // println!("{:?}", c.next());

    // let mut v = Mybox::new(5);
    // println!("{:p}", &mut(*v));

    let str = Mybox::new(String::from("Rust"));
    hello(&str.deref());

    let o = Some(3);

    let s1 = MyStruct {val: 3};
    let s2 = MyStruct { val: 4};
    let mut h= HashMap::new();
    h.insert(s1, 3);
    for k in h.keys() {
        println!("{:?}", *k);
    }
    print!("{:?}", h.keys());

    println!("{}", s2 > 4);
}