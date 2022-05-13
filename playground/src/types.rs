use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl From<u32> for Number {
    fn from(item: u32) -> Self {
        Number { value: item as i32 }
    }
}

#[derive(Debug)]
struct Number1 {
    value: i32,
}

impl From<i32> for Number1 {
    fn from(item: i32) -> Self {
        Number1 { value: item }
    }
}

fn main() -> () {
    let my_str = "Hello";
    let my_str = String::from(my_str);
    println!("{} world!", my_str);

    let num = Number::from(30u32);
    println!("My number is {:?} and value is {}", num, num.value);

    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into();
    println!("My number is {:?} and values is {}", num, num.value);

    let num: Number1 = int.into();
    println!("My number is {:?} and values is {}", num, num.value);
}
