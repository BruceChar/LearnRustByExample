fn main() {
  // 通用模式，标量变量可以，即实现了Display trait的变量，复杂变量比如Tuple、Vec、struct不行
  // In general, the `{}` will be automatically replaced with any
  // arguments. These will be stringified.
  println!("int:{}, str:{}, float: {}", 3, "WTF".to_string(), 3.1415);

  // Without a suffix, 31 becomes an i32. You can change what type 31 is
  // by providing a suffix. The number 31i64 for example has the type i64.

  // 位置参数
  // There are various optional patterns this works with. 
  // Positional arguments can be used.
  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

  // 命名参数
  // As can named arguments.
  println!("{subject} {verb} {object}",
           object="the lazy dog",
           subject="the quick brown fox",
           verb="jumps over");

  // 格式控制，通过{:pat}中的:pat 控制
  // 类型: 二进制{:b} 八进制{:o|O}  16进制{:x|X}
  println!("二进制：{0:b}, 八进制：{0:o}, 16进制：{0:x} {0:X}", 212);
  // 宽度 width:
  println!("宽度：\n{1:*^9}\n{0:#<wid$}\n{:d>2$.3e}\n", 3.4567, 1.23456, wid=5);

  // sign/#/0
  println!("{:08}", -23_i32);

  // 精度控制
  println!("Hello {} is {2:.*}",    "x", 5, 1.234);

  // 打印花括号{}
  // println!("{{ Hello}}");

  // Special formatting can be specified after a `:`.
  println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

  // You can right-align text with a specified width. This will output
  // "     1". 5 white spaces and a "1".
  println!("{number:>width$}", number=1, width=6);

  // You can pad numbers with extra zeroes. This will output "000001".
  println!("{number:>0width$}", number=1, width=6);

  // Rust even checks to make sure the correct number of arguments are
  // used.
  println!("My name is {0}, {1} {0}", "Bond", "James");

  // 自定义结构体使用 Debug宏，可以使用`{:?}`打印出结果，
  #[allow(dead_code)]
  #[derive(Debug)]
  struct Structure(i32);  

  #[derive(Debug)]
  struct Deep(Structure);

  #[derive(Debug)]
  struct Person<'a> {
    name: &'a str,
    age: u8
  }


  // However, custom types such as this structure require more complicated
  // handling. This will not work.
  println!("Now This struct `{:?}` will print...", Structure(3));
  println!("Struct deep: {0:?}", Deep(Structure(8)));
  println!("Person: {:?}", Person{name:"Bruce", age:19});
  println!("Person: {:#?}", Person{name:"Bruce", age:19});  //可读性

}