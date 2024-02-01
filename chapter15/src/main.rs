use std::{ops::Deref,rc::Rc};

use crate::List::{Cons, Nil};

// 如果不使用Box,rust将无法计算递归类型所需的大小
// Box<T>类型是一个智能指针，实现了 Deref trait，它允许 Box<T>值被当作引用对待
// Box<T> 类型实现了 Drop trait，因此在离开作用域时， box所指向的堆数据也会被清除
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

enum List {
    Cons(i32, Rc<List>),
    Nil
}
fn main() {
    let b = Box::new(5);
    println!("b={}", b);
}

#[test]
fn test_cons() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

struct MyBox<T>(T);

// 实现deref trait
impl<T> Deref for MyBox<T> {
    // 定义trait的关联类型
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data)
    }
}

#[test]
fn test_drop_trait() {
    let c = CustomSmartPointer {
        data: String::from("hello"),
    };
    println!("CustomSmartPointer created");
    // 提前释放资源
    drop(c);
    println!("CustomSmartPointer dropped before the end of the main");
}
