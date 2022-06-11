
#[derive(PartialOrd, PartialEq)]
enum ArrayIndex {
    Underflow,
    Index(i32),
    Overflow,
}

#[test]
fn test_myenum() {
    let a = ArrayIndex::Underflow;
    let b = ArrayIndex::Index(-10);
    let c = ArrayIndex::Index(-1);
    let d = ArrayIndex::Index(0);
    let e = ArrayIndex::Index(1);
    let f = ArrayIndex::Index(10);
    let g = ArrayIndex::Overflow;

    assert!(a < b);
    assert!(b < c);
    assert!(c < d);
    assert!(d < e);
    assert!(e < f);
    assert!(f < g);
}