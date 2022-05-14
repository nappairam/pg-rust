#[test]
fn test_array_iter() {
    let owned = ["a".to_owned(), "e".to_owned()];
    let borrowed = ["a", "e"];
    for _owned in owned.into_iter() {
        for _borrowed in borrowed.into_iter() {
            let a = format!("{_owned}{_borrowed}");
            println!("{a}");
            let _a = "test";
            dbg!(_borrowed);
            dbg!(_borrowed);
        }
        dbg!(_owned);
        // dbg!(_owned);
    }
    // dbg!(owned);
    dbg!(borrowed);
}
