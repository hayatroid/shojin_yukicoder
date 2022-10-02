fn read() -> Vec<u8> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().bytes().rev().map(|x| x - b'0').collect::<Vec<_>>()
}

fn main() {
    let a = read();
    let b = read();
    let mut ans = String::new();
    let mut carry = 0;
    for i in 0.. {
        if i >= a.len() && i >= b.len() && carry == 0 {
            break;
        }
        let mut tmp = carry;
        if i < a.len() {
            tmp += a[i];
        }
        if i < b.len() {
            tmp += b[i];
        }
        carry = tmp / 10;
        tmp -= tmp / 10 * 10;
        ans = format!("{}{}", tmp, ans);
    }
    println!("{}", ans);
}
