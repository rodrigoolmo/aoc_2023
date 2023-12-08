pub fn to_i32(s: &str) -> i32 {
    eprintln!("{:?}", s);
    s.parse::<i32>().unwrap()
}