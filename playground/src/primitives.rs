use std::mem;

fn as_bits(i: f64) -> u64 {
    unsafe { mem::transmute(i) }
}

#[test]
fn test_primitive() -> () {
    println!("Hello world!");
    let a = 1024;
    println!("A is {} {:b} 0x{:x}", a, a, a);
    let b: f64 = 0.45;
    println!(
        "B is {} {:b} 0x{:x} {}",
        b,
        as_bits(b),
        as_bits(b),
        b as u64
    );
    println!("B is {:x}", b.to_bits());

    println!("1 - 2 = {}", 1i32 - 2);
    // println!("1 - 2 = {}", 1u32 - 2);

    let bin = 0b10001001;
    println!("Binary number is {}", bin);
    let bin = b'F';
    println!("Binary number is {}", bin);
}
