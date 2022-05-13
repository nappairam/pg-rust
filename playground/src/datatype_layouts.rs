use std::arch::asm;
#[allow(dead_code)]
use std::mem;

fn crash_program() {
    let p: [u64; 3] = [0, 1, 1];
    let mut v: Vec<i32> = vec![];

    v = unsafe { mem::transmute(p) };
    println!("{:?}", v[0]);
}

fn integer_layout() {
    let a: u32 = 0x12345678;
    let b: [u8; 4];
    let c = "test".to_owned();

    b = unsafe { mem::transmute(a) };

    println!("{:x},{:x},{:x},{:x}", b[0], b[1], b[2], b[3]);
    println!("{:p} {:p} {:p}", &a, &b, &c);
}

fn vector_layout() {
    let p: [u64; 3];
    let mut v: Vec<i32> = vec![];
    println!("Size is {}", std::mem::size_of::<Vec<i32>>());

    v.push(10);
    println!("{:?}", &v[0] as *const i32);
    p = unsafe { mem::transmute(v) };
    println!("{:x}, {:x}, {:x}", p[0], p[1], p[2]);
}

fn string_layout() {
    let a: String = "maari".to_string();
    let b: String = "maari".to_owned();
    let mut p: [u64; 3] = [0; 3];
    println!("Size is {}", std::mem::size_of::<String>());

    println!("Address of a and b is {:p} {:p} {:p}", &a, &b, &p);

    p = unsafe { mem::transmute(a) };
    println!("{:x}, {:x}, {:x}", p[0], p[1], p[2]);
}

fn slice_layout() {
    let v: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let t = &v[3..7];
    let r: [u64; 2];

    println!("{:?}", &v[3] as *const i32);

    r = unsafe { mem::transmute(t) };

    println!("{:x}, {:x}", r[0], r[1]);
}

fn ownership_move() {
    let a: i32 = 10;
    let b: i32;

    b = a;
    println!("{:p} {:p} {} {}", &a, &b, a, b);

    // let sp: u64;
    // unsafe {
    //     asm( "mov %%rsp, %0" : "=rm" ( sp ));
    // }
    // assert_eq!(sp, 5);

    // println!("Current stack head is {}", cyclors::pthread_attr_getstackaddr);
    let _a = vec![10];
    let _b;

    println!("{:p} {:?}", &_a, _a);

    _b = _a;
    println!("{:p} {:?}", &_b, _b);
    // println!("{:p} {:?}", &_a, _a);
}

fn main() {
    println!("\nInteger Layout");
    integer_layout();

    println!("\nVector Layout");
    vector_layout();

    println!("\nString Layout");
    string_layout();

    println!("\nSlice Layout");
    slice_layout();

    println!("\nOwnership move");
    ownership_move();

    // crash_program();
}
