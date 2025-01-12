use alloc::boxed::Box;

pub struct Node<T> {
    pub data: T,
    pub next: Option<*mut Node<T>>,
}

pub struct LinkedList<T> {
    head: Option<*mut Node<T>>,
    size: usize,
}

impl<T> LinkedList<T> {
    pub const fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let node = Box::into_raw(Box::new(Node {
            data,
            next: self.head,
        }));
        self.head = Some(node);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|node_ptr| {
            unsafe {
                let node = Box::from_raw(node_ptr);
                self.head = node.next;
                self.size -= 1;
                node.data
            }
        })
    }

    pub fn find<F>(&self, predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        let mut current = self.head;
        while let Some(node_ptr) = current {
            unsafe {
                let node = &*node_ptr;
                if predicate(&node.data) {
                    return Some(&node.data);
                }
                current = node.next;
            }
        }
        None
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head;

        while let Some(node_ptr) = current {
            unsafe {
                let node = &mut *node_ptr;
                let next = node.next;
                node.next = prev;
                prev = Some(node_ptr);
                current = next;
            }
        }
        self.head = prev;
    }

    pub fn size(&self) -> usize {
        self.size
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
}
