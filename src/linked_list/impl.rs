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
    prev: Option<Rc<RefCell<Self>>>,
    next: Option<Rc<RefCell<Self>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node { data, prev: None, next: None }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None, tail: None }
    }

    pub fn push_front(&mut self, data: T) {
        let LinkedList { head, tail } = self;
        let rc = Rc::new(RefCell::new(Node::new(data)));
        let mut new_node = rc.borrow_mut();
        if let Some(node) = head {
            new_node.next = Some(node.clone());
            node.borrow_mut().prev = Some(rc.clone());
            *head = Some(rc.clone())
        } else {
            *head = Some(rc.clone());
            *tail = Some(rc.clone());
        }
    }

    pub fn push_back(&mut self, data: T) {
        let LinkedList { head, tail } = self;
        let rc = Rc::new(RefCell::new(Node::new(data)));
        let mut new_node = rc.borrow_mut();
        if let Some(node) = tail {
            new_node.prev = Some(node.clone());
            node.borrow_mut().next = Some(rc.clone());
            *tail = Some(rc.clone())
        } else {
            *head = Some(rc.clone());
            *tail = Some(rc.clone());
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow().next.as_ref() {
                next.borrow_mut().prev = None;
                self.head = Some(next.clone());
            } else {
                self.tail = None;
            }
            Rc::try_unwrap(head).ok().unwrap().into_inner().data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|tail| {
            if let Some(prev) = tail.borrow().prev.as_ref() {
                prev.borrow_mut().next = None;
                self.tail = Some(prev.clone());
            } else {
                self.head = None;
            }
            Rc::try_unwrap(tail).ok().unwrap().into_inner().data
        })
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
        assert_eq!(head.borrow().next.as_ref().unwrap().borrow().next.as_ref().unwrap().borrow().data, 1);

        let tail = list.tail.as_ref().unwrap();
        assert_eq!(tail.borrow().data, 1);
        assert_eq!(tail.borrow().prev.as_ref().unwrap().borrow().data, 2);
        assert_eq!(tail.borrow().prev.as_ref().unwrap().borrow().prev.as_ref().unwrap().borrow().data, 3);
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
        assert_eq!(head.borrow().next.as_ref().unwrap().borrow().next.as_ref().unwrap().borrow().data, 3);

        let tail = list.tail.as_ref().unwrap();
        assert_eq!(tail.borrow().data, 3);
        assert_eq!(tail.borrow().prev.as_ref().unwrap().borrow().data, 2);
        assert_eq!(tail.borrow().prev.as_ref().unwrap().borrow().prev.as_ref().unwrap().borrow().data, 1);
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
