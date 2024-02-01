fn main() {
    let mut s = String::from("hello world");

    // 字符串slice
    // let hello = &s[0..5]; 等价于下面的写法：-->
    let hello = &s[..5];
    // let world = &s[6..11];  等价于下面的写法：-->
    let world = &s[6..];
    println!("{} {}", hello, world);

    let world = first_word(&s);
    println!("{}", world);

    // 清空字符串
    s.clear();
    // 再次输出报错，因为clear获取了可变引用， world拥有不可变引用，这是不允许的
    // println!("{}", world);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
