use std::collections::HashMap;

fn main() {
    test_hashmap();
    // panic!("crash and burn");
}

#[allow(unused)]
fn test_string() {
    // 字符串是 UTF-8 编码的
    // let hello = String::from("السلام عليكم");
    // println!("{}", hello);
    // let hello = String::from("Dobrý den");
    // println!("{}", hello);
    // let hello = String::from("Hello");
    // println!("{}", hello);
    // let hello = String::from("שָׁלוֹם");
    // println!("{}", hello);
    // let hello = String::from("नमस्ते");
    // println!("{}", hello);
    // let hello = String::from("こんにちは");
    // println!("{}", hello);
    // let hello = String::from("안녕하세요");
    // println!("{}", hello);
    // let hello = String::from("你好");
    // println!("{}", hello);
    // let hello = String::from("Olá");
    // println!("{}", hello);
    // let hello = String::from("Здравствуйте");
    // println!("{}", hello);
    // let hello = String::from("Hola");
    // println!("{}", hello);

    // 追加字符串 push_str()
    let mut hello = "hello".to_string();
    hello.push_str(" world");
    println!("{}", hello);

    // 追加字符 push()
    hello.push('!');
    println!("{}", hello);

    // 拼接字符串
    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    // let s3 = s1 + &s2; // 获取s1的所有权  使用+拼接字符串时使用了add函数
    // 使用如下方式不会获取所有权
    let s3 = format!("{}-{}", s1, s2);
    println!("{}", s3);

    // 字符串不支持索引，String内部使用Vec<u8>实现
    let hello = "Здравствуйте";
    let answer = hello.chars().nth(1); // д
                                       // let answer = hello.bytes().nth(1); // 151
    println!("{}", answer.unwrap());
}

#[allow(unused)]
fn test_vec() {
    // let v: Vec<i32> = Vec::new();

    // 使用 vec! 宏来创建
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);

    // 读取 vec 元素
    let third = &v[2];
    println!("{}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("none"),
    }

    // 遍历 vec
    for i in &mut v {
        println!("{}", *i);
        *i += 10;
    }
    for (i, item) in v.iter().enumerate() {
        println!("{}-{}", i, item);
    }
}

#[allow(unused)]
fn test_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    // 使用元组的 collect 方法
    let teams = vec!["Blue", "Yellow"];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // 访问 hashmap
    // get 方法返回 Option<>，没有值返回 None
    let score = scores.get(&"Blue");
    match score {
        Some(i) => println!("{}", i),
        None => println!("None"),
    }

    // 遍历 hashmap
    for (key, val) in &scores {
        println!("{}-{}", key, val);
    }

    // 更新 map
    // 重复插入将会覆盖
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    // println!("{}", scores.get("Blue").unwrap());
    println!("{:?}", scores);

    // 不存在就插入
    scores.entry(String::from("Blue")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(30);
    println!("{:?}", scores);

    // 根据旧值更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert方法返回了val的可变引用
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
