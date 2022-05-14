pub trait AnyExt {
    fn type_name(&self) -> &'static str;
}

impl<T> AnyExt for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

fn is_copy<T: Copy>() {
    println!("Available in {}", std::any::type_name::<T>());
}

#[derive(Copy, Clone)]
struct Unit;

#[test]
fn test_is_copyable() {
    is_copy::<bool>();
    // is_copy::<&mut i32>();
    is_copy::<char>();
    is_copy::<i8>();
    is_copy::<i16>();
    is_copy::<i32>();
    is_copy::<i64>();
    is_copy::<u8>();
    is_copy::<u16>();
    is_copy::<u32>();
    is_copy::<u64>();
    is_copy::<isize>();
    is_copy::<usize>();
    is_copy::<f32>();
    is_copy::<f64>();
    is_copy::<fn()>();
    is_copy::<Unit>();
    let my_number = 32.90;
    println!("{}", my_number.type_name());
    let hello = &"Hello world";
    let hello_ref = &hello;
    println!("{}", hello.type_name());
    println!("{}", hello_ref.type_name());
    // let hello_mut_ref = &mut hello;
    // println!("{}",hello_mut_ref.type_name());
}
