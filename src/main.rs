mod my;
use mylib;
use another;

fn print(a: i32) -> i32 {
  println!("got {}", a);
  a+1
}

fn main() {
    println!("Hello, world!");
    my::my();
    my::wtf::wtf();
    mylib::wtf();
    mylib::wtf::hhe();
    another::wtf();
}

// #[cfg(test)]
mod tests{
  use super::*;

  #[test]
  fn pass() {
      let r = print(3);
      assert_eq!(4, r);
  }
  #[test]
  fn fail() {
      let r = print(1);
      assert_eq!(1,r);
  }
  #[test]
  #[ignore]
  fn test_pass() {
    assert_eq!(1,1);
  }
}