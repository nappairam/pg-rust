fn main() {
    #[derive(Debug, Default)]
    struct Test {
        a: i32,
        b: u32,
    };

    let _a = Test { a: 9, b: 10 };
    let _b = Test { a: 1, b: 2 };

    let a: Option<Test> = Some(_a);
    let b: Option<Test> = Some(_b);
    println!("a is {:p}", &a);
    dbg!(&a);

    let mut ptr = &a;
    let mut pptr = &ptr;
    let int_ptr = a.as_ref();
    let int_ptr_deref = a.as_ref().unwrap();
    println!(
        "ptr is {:p} {:?} {:p}->{:p} ({:?}) {:p}->{:p} ({:?})",
        &a, a, &ptr, ptr, ptr, &pptr, pptr, pptr
    );
    println!(
        "int_ptr is {:p}->{:p} {:?}",
        &int_ptr, int_ptr_deref, int_ptr
    );
    // BREAKPOINT HERE:
    // And Check the ptr, &ptr, pptr and &pptr

    ptr = &b;
    println!("ptr after changing is {:p}->{:p} {:?}", &ptr, ptr, ptr);

    let a: Option<Test> = None;
    println!("{:p}", &a);
    dbg!(&a);
}
