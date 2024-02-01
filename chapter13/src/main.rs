fn main() {
    println!("Hello, world!");
}

#[allow(unused)]
#[test]
fn test_iter() {
    let v1 = vec![1, 2, 3];
    // map方法会为每个迭代项调用闭包方法
    // collect方法会将迭代器转为一个新的集合
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);

    // filter
    let v3: Vec<_> = v1.iter().filter(|x| *x % 2 != 0).collect();
    println!("{:?}", v3);
}

// 自定义迭代器
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
#[test]
fn test_custom_iter() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    // zip方法将两个迭代器整合，返回一个元组迭代器，元组的第一个元素来自第一个迭代项，第二个元素来自第二个迭代项
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}