# Rust-Learning

学习文档：https://www.rustwiki.org.cn/zh-CN/book

Cargo 创建新项目

```rust
  cargo new project_name
```

Rust 特殊字面量：

- `b"bytes"`是一个字节字符串。
- `b'a'`是一个字节字符。
- `"string".to_string()`是一个`String`。
- `r"raw string"`是原生字符串。
- `br"raw bytes"`是原生字节字符串。



Rust 常用命令解析：

- rustc main.rs

  编译源文件并输出一个二进制可执行文件

- cargo build

  创建一个可执行文件 `target/debug/project_name`

- cargo build --release

  发布构建，创建一个可执行文件 `target/release/project_name`

- cargo run 

  同时编译代码并运行生成的可执行文件

- cargo check

  快速检查代码确保其可以编译，但并不产生可执行文件
  
- cargo doc --open

  构建当前的crate文档的HTML并在浏览器中打开，文档注释采用 ///，支持markdown，示例代码：
  
  ```rust
  /// Adds one to the number given.
  ///
  /// # Examples
  ///
  /// ```
  /// let arg = 5;
  /// let answer = my_crate::add_one(arg);
  ///
  /// assert_eq!(6, answer);
  /// ```
  pub fn add_one(x: i32) -> i32 {
      x + 1
  }
  ```
  
  



## chapter4 所有权

- 引用 &    
- 解引用 \*

创建一个引用的行为称为**借用** borrowing

注意事项：

- 不能同时定义多个可变引用 & mut
- 不能同时定义一个变量的可变引用和不可变引用

如何理解这里的同时？

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);
}
```

**一个引用的作用域从声明的地方开始到最后一次使用为止**



Rust不会产生悬垂引用，编译期间禁止

```rust
fn main() {
    let reference_to_nothing = dangle();
}

// 错误写法：
fn dangle() -> &String { // dangle 返回一个字符串的引用
    let s = String::from("hello"); // s 是一个新字符串
    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！

// 正确做法是直接返回字符串：
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```

## chapter5 使用结构体组织关联数据

## chapter6 枚举和模式匹配

定义枚举：

```rust
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// 实现枚举方法
impl Message {
    fn call(&self) {
        println!("function call...");
    }
}
```

如何提取枚举中的值-->模式匹配 match

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
      None => None,
      Some(i) => Some(i + 1),
    }
}
```

Match必须穷尽所有的可能分支：

- 通配模式 other

  ```rust
  let dice_roll = 9;
  match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
  }
  ```

- 占位符_

  ```rust
  let dice_roll = 9;
  match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
  }
  ```

if let 简单控制流   <==>  match 语法糖

```rust
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

## chapter7 使用包、crate

`cargo new --lib restaurant`

执行以上代码将创建一个名为`restaurant`的库(lib)

Src/lib.rs-->crate 根

```rust
#![allow(unused)]
fn main() {
  // 定义模块
  mod front_of_house {
    	// 嵌套其他模块
      mod hosting {
          fn add_to_waitlist() {}
          fn seat_at_table() {}
      }

      mod serving {
          fn take_order() {}
          fn server_order() {}
          fn take_payment() {}
      }
  }
}
```

对应的模块树：

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

Rust中默认所有项都是私有的，父模块不能使用子模块中的私有项，但是子模块可以使用父模块中的私有项

`pub`关键字可以申明为公共项

- `pub mod{...}`只能声明模块为公共的，并不代表齐内部的函数也是公共的，如果需要引用其内部的函数，必须使函数也用`pub`进行声明

- 结构体`struct`也是如此，`pub`只能声明结构体是公有的，但是字段也需要使用`pub`来进行声明

- 枚举`enum`只需要声明枚举为公有的，那么它的枚举字段都将变为公有

重要的概念：

- 绝对路径：从 `crate` 根部开始，以 `crate` 名或者字面量 `crate` 开头
- 相对路径：从当前模块开始，以 `self`、`super` 或当前模块的标识符开头

```rust
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

- `super`构建相对路径，类似于文件路径的`..`，可以引用父模块的函数

- `use`可以将路径一次性引入作用域，类似于`import`

  ```rust
  mod front_of_house {
      pub mod hosting {
          pub fn add_to_waitlist() {}
      }
  }
  
  use crate::front_of_house::hosting;
  
  pub fn eat_at_restaurant() {
      hosting::add_to_waitlist();
      hosting::add_to_waitlist();
      hosting::add_to_waitlist();
  }
  ```

  注意：

  1. `use`引入函数一般会引入到最小化的模块，而不会直接引入函数

  2. `use`引入结构体或者枚举一般会直接引入：

     ```rust
     use std::collections::HashMap;
     
     fn main() {
         let mut map = HashMap::new();
         map.insert(1, 2);
     }
     ```

- `pub use` 重导出名称，可以将项引入作用域并使其他代码可以引入自己的作用域

- 重复的项可以使用`as`重命名

  ```rust
  #![allow(unused)]
  fn main() {
    use std::fmt::Result;
    use std::io::Result as IoResult;
  
    fn function1() -> Result {
        // --snip--
        Ok(())
    }
  
    fn function2() -> IoResult<()> {
        // --snip--
        Ok(())
    }
  }
  ```

- 嵌套路径引入

  ```rust
  use std::{cmp::Ordering, io};
  ```

- `glob`运算符引入所有公有定义

  ```
  use std::collections::*;
  ```



## chapter9 错误处理

### panic

- 默认会展开，回溯栈并清理数据

- 直接终止：abort，可以减小项目编译的二进制文件，在cargo.toml中进行如下修改：

  ```toml
  [profile.release]
  panic = 'abort'
  ```

### 使用backtrace

使用backtrace追踪错误函数信息时必须开启debug标识，当不使用 --release 参数运行时debug标识默认启用，使用如下命令执行时会展开所有的backtrace:

```shell
RUST_BACKTRACE=1 cargo run
```

## chapter13 闭包和迭代器

迭代器是惰性的，必须要进行消费：`iter().next()`

- Iter() 迭代不可变引用
- Into_iter() 获取所有权并返回所有权
- Iter_mut() 迭代可变引用

创建自定义迭代器只需要实现next方法，当迭代结束时，该方法返回None：

```rust
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
```

## chapter15 智能指针

box 允许将一个值放在堆上而不是栈上，留在栈上的是指向数据的指针。

`box`的类型是`Box<T>`

- 递归类型：无法在编译时期知道大小

  `cons list`每一项都包含两个元素：当前项的值和下一项，最后一项值包含一个`Nil`值并且没有下一项，它通过递归调用`cons`函数产生

  ```rust
  enum List {
      Cons(i32, Box<List>),
      Nil,
  }
  ```

  `Box<T>`类型是一个智能指针，实现了 `Deref trait`，它允许 `Box<T>`值被当作引用对待
  `Box<T>` 类型实现了 `Drop trait`，因此在离开作用域时，`box`所指向的堆数据也会被清除

- `Deref trait` 

  允许我们重载**解引用运算符***。通过这种方式实现的 `Deref trai`t 的智能指针可以被当作常规引用来对待
  
  `Deref`由标准库提供，要求实现名为`deref`的方法，借用`self`返回一个内部数据的引用
  
  ```rust
  // 实现deref trait
  impl<T> Deref for MyBox<T> {
      // 定义trait的关联类型
      type Target = T;
  
      fn deref(&self) -> &Self::Target {
          &self.0
      }
  }
  ```
  
  `String`实现的`Deref trait`，其关联类型为`&str`，所以两者之间可以发生类型强制转换
  
- `Drop trait`

  指定在值离开作用域时应该执行的代码，这个`trait`要求实现一个`drop`方法
  
  变量以被创建时相反的顺序被丢弃
  
  ```rust
  struct CustomSmartPointer {
      data: String,
  }
  
  impl Drop for CustomSmartPointer {
      fn drop(&mut self) {
          println!("Dropping CustomSmartPointer with data `{}`",self.data)
      }
  }
  
  #[test]
  fn test_drop_trait() {
      let c =  CustomSmartPointer { data: String::from("hello") };
      let d =  CustomSmartPointer { data: String::from("world") };
      println!("CustomSmartPointer created");
  }
  ```
  
  `Drop`方法并不能被开发者主动调用，如果我们需要提前释放资源，可以通过`std::mem::drop`实现
  
- `Rc<T>`引用计数智能指针

  用于当我们希望在堆上分配一些内存供程序的多个部分读取，并且无法在编译时期确定程序的哪一部分会最终结束使用它的场景
  
  注意：只能用于单线程的场景
  
  ```rust
  use crate::List::{Cons, Nil};
  
  fn main() {
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
  ```
  
  使用`Rc::clone()`会增加引用计数(strong_count)

- `RefCell<T>`和内部可变性模式

  可以用于当开发者确信代码遵守借用规则，但是编译器无法理解和确定的时候

  `RefCell<T>`只能用于单线程场景

  如果违反了借用规则，代码将直接在运行时`panic`

  `RefCell<T>` 记录当前有多少个活动的 `Ref<T>` 和 `RefMut<T>` 智能指针。每次调用 `borrow`，`RefCell<T>` 将活动的不可变借用计数加一。当 `Ref<T>` 值离开作用域时，不可变借用计数减一。RefCell<T>在任何时候都是满足借用规则的：即只允许有多个不可变借用或一个可变借用。

  `RefCell<T>`的`borrow`方法返回一个不可变引用，`borrow_mut`方法返回一个可变引用，因为实现了`deref trait`，所以可以当做常规引用使用。

## chapter16 无畏并发

### **创建子线程**

```rust
use std::{thread, time::Duration};

fn main() {
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(500));
    }

    // 等待子线程结束,阻塞当前线程直到子线程完成
    handler.join().unwrap();
}
```

**使用`move`闭包**会覆盖`rust`默认的保守借用，获取变量的所有权

```rust
fn main() {
    let v = vec![1,2,3];
    let handler = thread::spawn(move || {
        println!("{:?}",v);
    });
    // 等待子线程结束,阻塞当前线程直到子线程完成
    handler.join().unwrap();
}
```

### **通过通道传递消息**

`mpsc` 是 **多个生产者，单个消费者**（*multiple producer, single consumer*）的缩写。简而言之，Rust 标准库实现通道的方式意味着一个通道可以有多个产生值的 **发送**（*sending*）端，但只能有一个消费这些值的 **接收**（*receiving*）端

```rust
#[test]
fn test_channel() {
    let (tx,rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("Hello").unwrap();
    });

    let msg = rx.recv().unwrap();
    println!("{msg}");
}
```

接收端有两个方法接收消息：

- recv()

  阻塞线程直到从通道中接收一个值。

  一旦发送了一个值，`recv` 会在一个 `Result<T, E>` 中返回它。当通道发送端关闭，`recv` 会返回一个错误表明不会再有新的值到来了

- try_recv()

  不会阻塞，立刻返回一个 `Result<T, E>`：`Ok` 值包含可用的信息，而 `Err` 值代表此时没有任何消息。 

**当将值发送到通道中，是不允许再次使用这个值的。**

**可以通过`clone()`方法复制发送端**

### 互斥器/锁

```rust
#[test]
fn test_mutex() {
    let m = Mutex::new(5);

    {
        // lock方法会阻塞当前线程，直到拥有锁为止
        // MutexGuard 是一个智能指针，实现了deref trait，可以直接解引用
        let mut num = m.lock().unwrap();
        *num = 6;
        // MutexGuard 实现了drop trait，会在离开作用域时自动释放锁
    }

    println!("m = {:?}", m);
}
```

