use alloc::boxed::Box;
use core::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> LinkedList<T> {
    pub const fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push_front(&mut self, data: T) {
        let node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(node);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            self.size -= 1;
            node.data
        })
    }

    pub fn find<F>(&self, predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        let mut curr = self.head.as_ref();
        while let Some(node) = curr {
            if predicate(&node.data) {
                return Some(&node.data);
            }
            curr = node.next.as_ref();
        }
        None
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut curr = self.head.take();

        while let Some(mut boxed_node) = curr {
            let next = boxed_node.next.take();
            boxed_node.next = prev;
            prev = Some(boxed_node);
            curr = next;
        }
        self.head = prev;
    }

    pub fn insert_after(&mut self, find_val: &T, new_val: T) -> bool
    where
        T: PartialEq,
    {
        let mut curr = self.head.as_mut();
        while let Some(node) = curr {
            if node.data == *find_val {
                let next_old = node.next.take();
                let new_node = Box::new(Node {
                    data: new_val,
                    next: next_old,
                });
                node.next = Some(new_node);
                self.size += 1;
                return true;
            }
            curr = node.next.as_mut();
        }
        false
    }

    pub fn remove_after(&mut self, find_val: &T) -> Option<T>
    where
        T: PartialEq,
    {
        let mut curr = self.head.as_mut();
        while let Some(node) = curr {
            if node.data == *find_val {
                let mut removed_node = node.next.take()?;
                let next_node = removed_node.next.take();
                node.next = next_node;
                self.size -= 1;
                return Some(removed_node.data);
            }
            curr = node.next.as_mut();
        }
        None
    }

    pub fn mergesort<F>(&mut self, cmp: &F)
    where
        F: Fn(&T, &T) -> Ordering,
    {
        fn split_in_half<T>(mut head: Box<Node<T>>) -> (Option<Box<Node<T>>>, Option<Box<Node<T>>>) {
            let mut slow: *mut Node<T> = &mut *head;
            let mut fast: *mut Node<T> = &mut *head;

            unsafe {
                while let Some(fast_next) = (*fast).next.as_ref() {
                    if fast_next.next.is_some() {
                        fast = (*fast).next.as_mut().unwrap().next.as_mut().unwrap().as_mut();
                        slow = (*slow).next.as_mut().unwrap().as_mut();
                    } else {
                        break;
                    }
                }

                let right = (*slow).next.take();
                (Some(head), right)
            }
        }

        fn merge_two_lists<T, F>(
            list1: Option<Box<Node<T>>>,
            list2: Option<Box<Node<T>>>,
            cmp: &F,
        ) -> Option<Box<Node<T>>>
        where
            F: Fn(&T, &T) -> Ordering,
        {
            match (list1, list2) {
                (None, None) => None,
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                (Some(mut l_box), Some(mut r_box)) => {
                    if cmp(&l_box.data, &r_box.data) != Ordering::Greater {
                        let l_next = l_box.next.take();
                        l_box.next = merge_two_lists(l_next, Some(r_box), cmp);
                        Some(l_box)
                    } else {
                        let r_next = r_box.next.take();
                        r_box.next = merge_two_lists(Some(l_box), r_next, cmp);
                        Some(r_box)
                    }
                }
            }
        }
        
        fn merge_sort_list<T, F>(
            head: Option<Box<Node<T>>>,
            cmp: &F,
        ) -> Option<Box<Node<T>>>
        where
            F: Fn(&T, &T) -> Ordering,
        {
            let head = match head {
                None => return None,
                Some(h) => h,
            };

            if head.next.is_none() {
                return Some(head);
            }

            let (left, right) = split_in_half(head);

            let left_sorted = merge_sort_list(left, cmp);
            let right_sorted = merge_sort_list(right, cmp);

            merge_two_lists(left_sorted, right_sorted, cmp)
        }

        self.head = merge_sort_list(self.head.take(), cmp);
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_push_front() {
        let mut list = LinkedList::new();
        list.push_front(10);
        list.push_front(20);

        assert_eq!(list.size(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.pop_front(), Some(10));
    }

    #[test]
    fn test_list_find() {
        let mut list = LinkedList::new();
        list.push_front(10);
        list.push_front(20);

        let found = list.find(|&x| x == 10);
        assert!(found.is_some());
        assert_eq!(*found.unwrap(), 10);
    }

    #[test]
    fn test_list_reverse() {
        let mut list = LinkedList::new();
        list.push_front(10);
        list.push_front(20);
        list.push_front(30);

        list.reverse();

        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.pop_front(), Some(30));
    }


    #[test]
    fn test_insert_after() {
        let mut list = LinkedList::new();
        list.push_front(10);
        list.push_front(20);

        assert!(list.insert_after(&20, 15));
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.pop_front(), Some(15));
        assert_eq!(list.pop_front(), Some(10));
    }

    #[test]
    fn test_remove_after() {
        let mut list = LinkedList::new();
        list.push_front(10);
        list.push_front(20);
        list.push_front(30);

        assert_eq!(list.remove_after(&30), Some(20));
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.pop_front(), Some(10));
    }

    #[test]
    fn test_mergesort() {
        let mut list = LinkedList::new();
        list.push_front(5);
        list.push_front(2);
        list.push_front(8);
        list.push_front(3);
        list.push_front(7);

        list.mergesort(&|a, b| a.cmp(b));

        let mut sorted_values = vec![];
        while let Some(value) = list.pop_front() {
            sorted_values.push(value);
        }

        assert_eq!(sorted_values, vec![2, 3, 5, 7, 8]);
    }
}
