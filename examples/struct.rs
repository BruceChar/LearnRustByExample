use std::io::stdin;

fn main() {
  let m = Man{p:Person{name: "Bruce", age: 18}, sex: "male"};
  println!("Man:{:?}", m);
}

struct Unit;

#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8
}

#[derive(Debug)]
struct Man<'a> {
  p: Person<'a>,
  sex: &'a str
}
