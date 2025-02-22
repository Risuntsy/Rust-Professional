/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::cmp::PartialOrd;


#[derive(Debug)]
struct Node<T> 
where T: PartialOrd 
{
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> 
where T: PartialOrd 
{
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> 
where T: PartialOrd 
{
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> 
where T: PartialOrd 
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> 
where 
    T: PartialOrd
{
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    fn take_head(&mut self) -> Option<T> {
        self.start.map(|node| {
            let node = unsafe { Box::from_raw(node.as_ptr()) };
            self.start = node.next;
            self.length -= 1;
            if self.start.is_none() {
                self.end = None;
            };
            node.val
        })
    }

    fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self {
        if list_a.length == 0 {
            return list_b;
        }
        if list_b.length == 0 {
            return list_a;
        }

        let (mut ptr_a, mut ptr_b) = if list_a.get(0) < list_b.get(0) {
            (list_a.start.unwrap(), list_b.start.unwrap())
        } else {
            (list_b.start.unwrap(), list_a.start.unwrap())
        };

        let mut merged_list: LinkedList<T> = LinkedList {
            length: list_a.length + list_b.length,
            start: Some(ptr_a),
            end: None,
        };

        loop {
            unsafe {
                while let Some(next_a) = ptr_a.as_ref().next {
                    if next_a.as_ref().val > ptr_b.as_ref().val {
                        break;
                    }
                    ptr_a = next_a;
                }

                if ptr_a.as_ref().next.is_none() {
                    ptr_a.as_mut().next = Some(ptr_b);
                    merged_list.end = Some(ptr_b);
                    break;
                }

                let temp = ptr_a.as_mut().next;
                ptr_a.as_mut().next = Some(ptr_b);
                ptr_a = ptr_b;
                ptr_b = temp.unwrap();
            }
        }

        merged_list
    }
}

impl<T:PartialOrd> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T:PartialOrd> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}