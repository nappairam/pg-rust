fn main() {
    let a: Vec<u32> = vec![1, 2, 4];
    println!("{:?}", a);
    for i in a.iter() {
        println!("{:?}", a);
        println!("{:?}", i);
    }
    for i in a.into_iter() {
        // println!("{:?}", &a);
        println!("{:?}", i);
    }
    // dbg!(a);

    let _a = [1, 2, 4];
    for i in _a.iter() {
        println!("{:?}", _a);
        println!("{:?}", i);
    }
    for i in _a.into_iter() {
        println!("{:?}", &_a);
        println!("{:?}", i);
    }
    dbg!(_a);

    let names = vec!["Jane", "Jill", "Jack", "John"];
    let b = "maari".to_owned();
    let c = "maari";

    dbg!(b);
    dbg!(c);

    for name in names.iter() {
        println!("{}", name);
        println!("Sister");
    }

    let total_bytes = names
        .iter()
        .map(|name| name.len())
        .fold(0, |acc, len| acc + len);

    assert_eq!(total_bytes, 16);
}
