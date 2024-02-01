use std::{fs::{File, self}, io::{ErrorKind, Read, Error}, error};

fn main() {
    let f = File::open("hello.txt");
    // 使用 match 表达式处理错误
    let _f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("Problem create the file: {:?}", err),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // 等价于下面这种写法，可以消除嵌套 match
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

#[allow(unused)]
// fn test_err() -> Result<String, Error>{
//   // unwarp：如果Ok,返回Ok中的值，否则调用panic!
//   // let mut f = File::open("hello.txt").unwrap();
//   let mut f = File::open("hello.txt")?;
  
//   // 使用?传播错误
//   let mut s = String::new();
//   f.read_to_string(&mut s)?;
//   Ok(s)
// }

// 更简洁的写法
fn test_err() -> Result<String, Error> {
  fs::read_to_string("hello.txt")
}