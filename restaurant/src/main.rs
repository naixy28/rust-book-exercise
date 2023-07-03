fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = "_".to_string() + &s1 + &s2 + &s2;

    let s = format!("{}-{}-{}", s1, s2, s3);

    print!("{}", s);
}
