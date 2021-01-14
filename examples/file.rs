use std::fs::File;
use std::io::ErrorKind;

fn main() {

    let_open();
  match_open();
}

fn match_open() {
  match File::open("hello.txt") {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => {
          println!("File not found");
          match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
          }
        },
        other_error => panic!("Problem opening the file: {:?}", other_error),
    },
  };
}

fn let_open() {
  let f = File::open("hello.txt").unwrap_or_else(|err| {
    if err.kind() == ErrorKind::NotFound {
      File::create("another/hell.txt").unwrap_or_else(|err| {
        panic!("Problem creating file:{:?}", err);
      })
    } else {
      panic!("Problem open file: {:?}", err);
    }
  });
}
