use std::fmt::Display;
use std::io::{Lines as iLines, BufReader, BufRead};
use std::str::Lines;
use std::borrow::Borrow;
use std::fs::File;

fn parse<T>(lines: T)
where T: IntoIterator, T::Item: Borrow<str>
{
    for l in lines {
        println!("{}", l.borrow());
    }
}

fn read(path: &str) {
    if path.is_empty() {
        let cont = "WTF\nMAN\nFUCK";
        parse(cont.lines());
    } else {
        let f = File::open(path).unwrap();
        let buf = BufReader::new(f);
        parse(buf.lines().map(|l| l.unwrap_or("".to_owned())));
    }
}


fn main () {
    let lines = "some\nniubi\nstring".lines();
    read("");
    read("Cargo.toml");     // src/another/Cargo.toml   .....wtf
    read("hell.txt");
}