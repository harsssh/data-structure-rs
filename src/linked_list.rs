use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq)]
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug, PartialEq)]
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node { data, next: None }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None, tail: None }
    }
    pub fn push_front(&mut self, data: T) {
        let LinkedList { head, tail } = self;
        if let Some(node) = head {
            let new_node = Node { data, next: Some(node.clone()) };
            *head = Some(Rc::new(RefCell::new(new_node)));
        } else {
            let rc = Rc::new(RefCell::new(Node::new(data)));
            *head = Some(rc.clone());
            *tail = Some(rc);
        }
    }

    pub fn push_back(&mut self, data: T) {
        unimplemented!()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unimplemented!()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let list: LinkedList<i32> = LinkedList::new();

        assert_eq!(list.head, None);
        assert_eq!(list.tail, None);
    }

    #[test]
    fn push_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let head = list.head.as_ref().unwrap();
        assert_eq!(head.borrow().data, 3);
        assert_eq!(head.borrow().next.as_ref().unwrap().borrow().data, 2);
        assert_eq!(list.tail.as_ref().unwrap().borrow().data, 1);
    }

    #[test]
    fn push_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let head = list.head.as_ref().unwrap();
        assert_eq!(head.borrow().data, 1);
        assert_eq!(head.borrow().next.as_ref().unwrap().borrow().data, 2);
        assert_eq!(list.tail.as_ref().unwrap().borrow().data, 3);
    }

    #[test]
    fn pop_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn pop_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }
}
