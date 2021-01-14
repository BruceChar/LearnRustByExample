fn main() {

    let v= vec![1, 2, 3];
    v.iter().map(|x| x).collect();
    'outer: loop {
        println!("Enter out loop");
        'inner: loop {
            println!("Enter inner loop");
            break 'outer;
        }
    }
    let mut cnt = 0;
    let re = loop {
        cnt+=1;
        if cnt == 10 {
            break cnt;
        }
    };
    println!("Exit loop {}", re);
}