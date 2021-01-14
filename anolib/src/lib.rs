pub fn add_two(x: i32) -> i32 {
    println!("call add two");
    x + 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
