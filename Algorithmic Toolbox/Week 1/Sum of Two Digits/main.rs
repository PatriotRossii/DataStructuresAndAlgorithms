use std::io::Read;

fn main() {
    let mut buffer = String::new();
    
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut iter = buffer.split_whitespace();

    let a: i64 = iter.next().unwrap().parse().unwrap();
    let b: i64 = iter.next().unwrap().parse().unwrap();

    println!("{}", a + b);
}