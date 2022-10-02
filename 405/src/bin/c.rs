fn read() -> i64 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn main() {
    let a = read();
    let b = read();
    if a == 1 && b == 0 {
        println!("20221003"); // なんでもよい
    } else {
        println!("{}", b / (1 - a));
    }
}
