fn read() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn main() {
    let n = read();
    let mut m = read();
    let mut ans = String::new();
    if m == 0 {
        println!("0");
        return;
    }
    while m > 0 {
        ans = format!("{}{}", m % n, ans);
        m /= n;
    }
    println!("{}", ans);
}
