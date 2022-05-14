#![allow(unused_variables)]

use std::mem;

#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }
    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }
}

#[cfg(test)]

#[test]
fn test_linked_list() {
    let a: u32 = 10;
    let b: (u32,) = (10,);
    let empty: LinkedList<u32>= LinkedList::new();
    let mut ll: LinkedList<u32>= LinkedList::new();
    println!("empty = {:?} {:p} {:?} size:{:?}", empty, &empty, mem::discriminant(&empty.0), mem::size_of::<LinkedList<i32>>());
    println!("ll = {:?} {:p} {:?}", ll, &ll, mem::discriminant(&ll.0));

    ll.push_front(3);
    println!("ll = {:?} {:p} {:?}", ll, &ll, mem::discriminant(&ll.0));
    ll.push_front(1);
    ll.push_front(4);
    ll.push_back(8);

    println!("ll = {:?} {:p}", ll, &ll);

    let mut ll1: Box<LinkedList<u32>> = Box::new(LinkedList::new());
    println!("ll1 = {:?} {:p}", ll1, &ll1);

    ll1.push_front(3);
    ll1.push_back(8);
    ll1.push_front(1);

    println!("ll1 = {:?} {:p}", ll1, &ll1);
}

// challenge insert_sorted
// find the first element it goes before and put it infront.
//impl <T:PartialOrd> LinkedList<T>{ ... }
