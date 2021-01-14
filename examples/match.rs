
#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
  // --snip--
}

#[derive(Debug)]
enum Coin {
  Penney,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
  {
    use Coin::*;
    match coin {
      Penney => 1,
      Quarter(state) => {
        println!("Us state is: {:?}", state);
        25
      },
      // 我们遗漏Dime，Nickel 处理逻辑，可以用通配符来处理
      _ => {
        println!("不知道什么鬼！");
        0
      }
    }
  }
}

// 变量匹配
fn var_match() {
  let x = Some(5);
  let y = 10;

  match x {
      Some(50) => println!("Got 50"),
      Some(5) => println!("Got 5"),
      Some(y) => println!("Matched, y = {:?}", y),
      _ => println!("Default case, x = {:?}", x),
  }

  println!("at the end: x = {:?}, y = {:?}", x, y);
}


// 变量绑定
fn other_bind(coin: Coin) {
  match coin {
    Coin::Dime => {
      println!("It is Dime: {:?}", Coin::Dime);
    },
    other => {
      println!("Any other coin except Dime: {:?}", other);
    }
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1)
  }
}

// 统计非Quarter的硬币
fn count_without_quarter(coins: &Vec<Coin>) -> i32 {
  let mut cnt = 0;
  use Coin::*;
  coins.iter().for_each(|c| {
    if let Quarter(state) = c {
      println!("State quarter from {:?}", state);
    } else {
      cnt += 1;
      println!("new count: {}", cnt);
    }
  });
  cnt
}

// while let
fn count_while(coins: &mut Vec<Coin>) -> i32 {
  let mut cnt = 0;
  use Coin::*;
  while let Some(coin) = coins.pop() {
    if let Quarter(state) = coin {
      println!("State quarter from {:?}", state);
    } else {
      cnt += 1;
      println!("new count: {}", cnt);
    }
  };
  cnt
}

fn main() {
  // let coin = Coin::Quarter(UsState::Alabama);
  // let cent = value_in_cents(coin);
  // println!("Cnets {}", cent);

  // let six = plus_one(Some(5));
  // let six = six.unwrap();
  // println!("six: {:?}", six);
  // use Coin::*;
  // let mut coins = vec![Dime, Penney, Quarter(UsState::Alabama), Nickel, Dime, Penney];

  // let s = String::new();
  // println!("string: {}", s);
  

  // let s1 = String::from("Hello, ");
  // let s2 = String::from("world!");
  // let s4 = format!("{}-{}", s1, s2);
  // let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
  // println!("{} {} ", s3, s4);

  // other_bind(Coin::Dime);
  // other_bind(Coin::Quarter(UsState::Alabama));

  // count_while(&mut coins);
  // let cnt = count_without_quarter(&coins);
  // println!("Total coin without quarter is {}", cnt);

  var_match();

}