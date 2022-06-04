#[cfg(test)]
#[test]
fn test_option() {
    #[derive(Debug, Default)]
    struct Test<T>(Option<(T, T)>);

    let _a = Test(Some((10, 9)));
    let _b = Test(Some((-1, 0)));
    let _c: Test<i32> = Test(None);

    let a: Option<Test<_>> = Some(_a);
    let b: Option<Test<_>> = Some(_b);
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

    let a: Option<Test<i32>> = None;
    println!("{:p}", &a);
    dbg!(&a);
}
