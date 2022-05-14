#[derive(Debug)]
pub struct RecursiveLinkedList<T>(Option<(T, Box<RecursiveLinkedList<T>>)>);

impl<T: PartialOrd> RecursiveLinkedList<T> {
    pub fn new() -> Self {
        RecursiveLinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(RecursiveLinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }

    pub fn insert_sorted(&mut self, data: T) {
        match &mut self.0 {
            Some((cur_data, ref mut child)) => {
                if data > *cur_data {
                    child.insert_sorted(data)
                } else {
                    self.push_front(data)
                }
            }
            None => self.push_front(data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_linked_list() {
        let _a: u32 = 10;
        let _b: (u32, ) = (10, );
        let empty: RecursiveLinkedList<u32> = RecursiveLinkedList::new();
        let mut ll: RecursiveLinkedList<u32> = RecursiveLinkedList::new();
        println!("empty = {:?} {:p} {:?} size:{:?}", empty, &empty, mem::discriminant(&empty.0), mem::size_of::<RecursiveLinkedList<i32>>());
        println!("ll = {:?} {:p} {:?}", ll, &ll, mem::discriminant(&ll.0));

        ll.push_front(3);
        println!("ll = {:?} {:p} {:?}", ll, &ll, mem::discriminant(&ll.0));
        ll.push_front(1);
        ll.push_front(4);
        ll.push_back(8);

        println!("ll = {:?} {:p}", ll, &ll);

        let mut sorted_list: Box<RecursiveLinkedList<u32>> = Box::new(RecursiveLinkedList::new());
        println!("sorted_list = {:?} {:p}", sorted_list, &sorted_list);

        sorted_list.insert_sorted(9);
        sorted_list.insert_sorted(1);
        sorted_list.insert_sorted(10);
        sorted_list.insert_sorted(8);
        sorted_list.insert_sorted(5);

        println!("sorted list = {:?} {:p}", sorted_list, &sorted_list);
    }

    #[test]
    #[ignore]
    fn test_linked_list_stack_blown() {
        let mut blown_stack: Box<RecursiveLinkedList<u32>> = Box::new(RecursiveLinkedList::new());
        let iterations = 1697;
        println!("Trying to blow stack with iterations: {}", iterations);
        for i in 0..iterations {
            // blown_stack.insert_sorted(i);
            blown_stack.push_back(i);
        }
        println!("ll1 = {:?} {:p}", blown_stack, &blown_stack);
    }
}
