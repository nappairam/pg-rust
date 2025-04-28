use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
#[derive(Debug)]
struct Database(usize);


fn main() {
    let prot_map = Mutex::new(HashMap::new());

    {
        let mut map = prot_map.lock().unwrap();
        map.insert(10, Arc::new(Database(10)));
        map.insert(20, Arc::new(Database(20)));
    }

    println!("Program to demonstrate when mutex lock will be release in `if let` case");
    println!();

    println!("Map is {:?}", prot_map);
    println!();

    println!("Testing:");
    println!("if let Some(a) = prot_map.lock().unwrap().get(&10)");
    if let Some(a) = prot_map.lock().unwrap().get(&10) {
        println!("Found : {:?}", a);
        let lockable = prot_map.try_lock().is_ok();
        println!("Checking lock status inside `if let`: {:?}", lockable);
    }

    let lockable = prot_map.try_lock().is_ok();
    println!("Verifying lock status after `if let` block: {:?}", lockable);
    println!();

    println!("Testing:");
    println!("if let Some(a) = prot_map.lock().unwrap().get(&10).cloned()");
    if let Some(a) = prot_map.lock().unwrap().get(&10).cloned() {
        println!("Found : {:?}", a);
        let lockable = prot_map.try_lock().is_ok();
        println!("Checking lock status inside `if let`: {:?}", lockable);
    }

    let lockable = prot_map.try_lock().is_ok();
    println!("Verifying lock status after `if let` block: {:?}", lockable);
    println!();

    println!("Testing:");
    println!("if let Some(a) =  {{ prot_map.lock().unwrap().get(&10).cloned() }}");
    if let Some(a) = { prot_map.lock().unwrap().get(&10).cloned() } {
        println!("Found : {:?}", a);
        let lockable = prot_map.try_lock().is_ok();
        println!("Checking lock status inside `if let`: {:?}", lockable);
    }
}
