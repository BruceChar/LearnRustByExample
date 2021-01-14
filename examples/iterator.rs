//! ##Iterator迭代器
//! 数组类型[T; n]没有实现Iterator，但实现了IntoIterator.所以无法直接通过for遍历，但可以使用
//! .iter() 或 .iter_mut() 或 into_iter() 进行转换为迭代器类型。但IntoIterator只实现了引用和
//! 可变引用迭代器，未实现值迭代器，即into_iter() 根据上下文只相当于iter()或iter_mut().
//! 数组切片&[T; N]实现了Iterator,所以自动实现了IntoIterator
//!
//! ### Iterator & IntoIterator
//! 如果实现了Iterator,自动实现IntoIterator--into_iter(), 以及FromIterator--collect(),转为对应的collections；
//! 如果实现了Iterator,可以直接调用next()方法，也可以直接用for遍历；
//! 如果没实现Iterator，实现了IntoIterator,可以通过for直接遍历，也可以通过into_iter()转成Iterator，调用next();
//! iter()和iter_mut()是collections中为了引用迭代器和可变引用迭代器特设的语法，自动实现的IntoIterator只实现
//! into_iter()方法。

use std::iter::FromIterator;

fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

#[derive(Debug)]
struct Fibonacci {
    val: u32,
    next: u32
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new = self.val + self.next;
        self.val = self.next;
        self.next = new;
        Some(self.val)
    }
}

impl Fibonacci {
    fn new() -> Self {
        Self {
            val: 0,
            next: 1
        }
    }
}

/// 自定义结构体，不适合实现Iterator，但可以实现IntoIterator
#[derive(Debug, PartialEq)]
struct Words<'a> {
    words: Vec<&'a str>
}

impl<'a> Words<'a> {
    fn new(init: &'a str) -> Self {
        Self { words: init.split(" ").collect() }
    }
}

/// 值类型迭代器
struct IntoIteratorHelper<'a> {
    iter: std::vec::IntoIter<&'a str>
}

impl<'a> IntoIterator for Words<'a> {
    type Item = &'a str;
    type IntoIter = IntoIteratorHelper<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIteratorHelper {
            iter: self.words.into_iter()
        }
    }
}

impl<'a> Iterator for IntoIteratorHelper<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// 引用类型迭代器
struct IterHelper<'a> {
    iter: std::slice::Iter<'a, &'a str>
}

impl<'a> IntoIterator for &'a Words<'a> {
    type Item = &'a &'a str;
    type IntoIter = IterHelper<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IterHelper {
            iter: self.words.iter()
        }
    }
}

impl<'a> Iterator for IterHelper<'a> {
    type Item = &'a &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// 可变引用迭代器
struct IterMutHelper<'a> {
    iter: std::slice::IterMut<'a, &'a str>
}

impl<'a> IntoIterator for &'a mut Words<'a> {
    type Item = &'a mut &'a str;
    type IntoIter = IterMutHelper<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IterMutHelper { iter: self.words.iter_mut() }
    }
}

impl<'a> Iterator for IterMutHelper<'a> {
    type Item = &'a mut &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}


impl<'a> Words<'a> {
    fn iter(&'a self) -> IterHelper<'a> {
        self.into_iter()
    }

    fn iter_mut(&'a mut self) -> IterMutHelper<'a> {
        self.into_iter()
    }
}

/// FromIterator
impl<'a> FromIterator<&'a str> for Words<'a> {
    fn from_iter<T>(iter: T) -> Self
    where T: IntoIterator<Item = &'a str>
    {
        Words {
            words: iter.into_iter().collect(),
        }
    }
}

fn exam_run() {
    let mut names = vec!["Bruce", "Chal", "Grooy"];
    // print_type(&names.into_iter());
    for n in names {
        println!("{}", n);
        // print_type(&n);
    }
    // error: names has been moved
    // println!("{:?}", names);

    let mut names = ["Bruce", "Chal", "Grooy"];
    for n in &names {
        println!("{}", n);
        // print_type(&n);
    }
    // Ok: [&str;3] dont implement value iterator
    println!("{:?}", names);

    let names = ["Bruce", "WTF"];
    // println!("{:?}", (&names).next());

    let fib = Fibonacci::new();
    // println!("{:?} {:?} {:?}", fib.next(), fib.next(), fib.next());

    for f in fib.into_iter() {
        println!("{}", f);
        print_type(&f);
        break;
    }
    // Error: 这里是值迭代，不可以再借用
    // println!("{:?}", fib);

    // FromIterator
    let mut fib = Fibonacci::new();
    let v: Vec<_> = fib.take(3).collect();
    print!("{:?}", v);

    let mut fib = Fibonacci::new();
    println!("{:?}", fib.nth(0));

    let mut fib = Fibonacci::new();
    for f in fib.take(10).step_by(4) {
        println!("{:?}", f);
    }
}


fn main() {
    let mut word = Words::new("Hello Bruce I am WTFUCKER");
    for w in word.iter_mut() {
        println!("{} ", w);
    }

    let words: Words = vec!["Hello", "Bruce", "I", "am", "WTFUCKER"].into_iter().collect();
    let mut word = Words::new("Hello Bruce I am WTFUCKER");
    println!("{:?}", words);
    assert_eq!(word, words);

}