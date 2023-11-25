#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn print_user(&self) {
        println!(
            "active:{} username:{} email:{} sign_in_count:{}",
            self.active, self.username, self.email, self.sign_in_count
        );
    }

    fn set_email(&mut self, email: String) {
        self.email = email;
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32{
        self.width * self.height
    }
}

// 没有命名字段的元组结构体
struct Point(i32, i32, i32);

fn main() {
    let mut user = User{
        active: true,
        username: String::from("someone"),
        email: String::from("someone@example.com"),
        sign_in_count: 10,
    };

    user.email = String::from("another@example.com");

    println!("user is {:?}", user);
    user.print_user();

    // 结构体更新语法
    let mut user2 = User{
        email: String::from("another@example.com"),
        ..user
    };
    println!("user2 is {:?}",user2);

    user2.set_email(String::from("1234@qq.com"));
    user2.print_user();

    let point = Point(1,2,3);
    println!("{}", point.1);

    println!("====================");

    let square = Rectangle::square(10);
    println!("area is {}", square.area());
}
