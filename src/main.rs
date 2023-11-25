fn main() {
    let mut s1 = String::from("Hello");

    // 可变引用只能有一个
    // 不可变引用可以同时存在多个
    let len = calculate_length(&mut s1);

    println!("{s1}, {len}");


    let first_world = first_world(&s1);

    println!("first_world: {}", first_world);
    println!("first_world: {}", first_world2(&s1));

    // 字符串切片
    println!("{}", &s1[0..5]);
    println!("{}", &s1[6..11]);
}

// 引用，使用其值但不获取所有权
fn calculate_length(s: &mut String) -> usize {
    // *s = String::from("World");
    s.push_str(" World");
    s.len()
}

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}

fn first_world2(s: &str) -> &str {
    for (i, item) in s.chars().into_iter().enumerate() {
        if item == ' ' {
            return &s[0..i];
        }
    }
    s
}
