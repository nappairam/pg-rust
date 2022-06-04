#![cfg(test)]

use std::mem::size_of;

macro_rules! show_size {
    (header) => {
        println!(
            "{:<30} {:>4}   {:<12} {:4}",
            "Type", "T", "Option<T>", "Option<Option<T>>"
        );
    };
    ($t:ty) => {
        println!(
            "{:<30} {:>4}   {:<12} {:<4}",
            stringify!($t),
            size_of::<$t>(),
            size_of::<Option<$t>>(),
            size_of::<Option<Option<$t>>>()
        )
    };
}

#[test]
fn test_size_of() {
    pub struct SingleTuple<T>(T);
    pub struct DualTuple<T, S>(T, S);
    pub struct TupleWithBox<T>(T, Box<T>);
    pub struct OptionTupleWithBox<T>(Option<(T, Box<OptionTupleWithBox<T>>)>);

    show_size!(header);
    show_size!(i32);
    show_size!(&i32);
    show_size!(Box<i32>);
    show_size!(&[i32]);
    show_size!(Vec<i32>);
    show_size!(Result<(), Box<i32>>);
    show_size!(SingleTuple<Box<i32>>);
    show_size!(SingleTuple<i32>);
    show_size!(DualTuple<i32, i32>);
    show_size!(DualTuple<i32, i64>);
    show_size!(Option<DualTuple<i32, i64>>);
    show_size!(TupleWithBox<i32>);
    show_size!(Option<TupleWithBox<i32>>);
    show_size!(OptionTupleWithBox<i32>);
    show_size!(String);
    show_size!(SingleTuple<String>);
    show_size!(DualTuple<String, i32>);
    show_size!(DualTuple<String, i64>);
    show_size!(TupleWithBox<String>);
    show_size!(OptionTupleWithBox<String>);
}
