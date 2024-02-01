pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use core::panic;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{width:8, height:7};
        let smaller = Rectangle{width:5, height:1};
        // assert!(larger.can_hold(&smaller));
        assert!(smaller.can_hold(&larger), "larger is {:?} and smaller is {:?}", larger, smaller);
    }

    #[test]
    #[should_panic(expected="Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        panic!("Guess value must be less than or equal to 100")
    }

    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
