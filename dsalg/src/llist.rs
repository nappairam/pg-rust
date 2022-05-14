use std::mem;

#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T: PartialOrd> LinkedList<T> {
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

    #[test]
    fn test_linked_list() {
        let _a: u32 = 10;
        let _b: (u32, ) = (10, );
        let empty: LinkedList<u32> = LinkedList::new();
        let mut ll: LinkedList<u32> = LinkedList::new();
        println!("empty = {:?} {:p} {:?} size:{:?}", empty, &empty, mem::discriminant(&empty.0), mem::size_of::<LinkedList<i32>>());
        println!("ll = {:?} {:p} {:?}", ll, &ll, mem::discriminant(&ll.0));

        ll.push_front(3);
        println!("ll = {:?} {:p} {:?}", ll, &ll, mem::discriminant(&ll.0));
        ll.push_front(1);
        ll.push_front(4);
        ll.push_back(8);

        println!("ll = {:?} {:p}", ll, &ll);

        let mut sorted_list: Box<LinkedList<u32>> = Box::new(LinkedList::new());
        println!("sorted_list = {:?} {:p}", sorted_list, &sorted_list);

        sorted_list.insert_sorted(9);
        sorted_list.insert_sorted(1);
        sorted_list.insert_sorted(10);
        sorted_list.insert_sorted(8);
        sorted_list.insert_sorted(5);

        println!("sorted list = {:?} {:p}", sorted_list, &sorted_list);
    }

    #[test]
    #[should_panic]
    fn test_linked_list_stack_blown() {
        let mut blown_stack: Box<LinkedList<u32>> = Box::new(LinkedList::new());
        let iterations = 1698;
        println!("Trying to blow stack with iterations: {}", iterations);
        for i in 0..iterations {
            blown_stack.insert_sorted(i);
        }
        println!("ll1 = {:?} {:p}", blown_stack, &blown_stack);
    }
}
