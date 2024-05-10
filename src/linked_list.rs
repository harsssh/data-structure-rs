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

    pub fn pop_front() -> Option<T> {
        unimplemented!()
    }

    pub fn pop_back() -> Option<T> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_front() {
        let mut list = LinkedList::new();
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);
        let want = {
            let n3 = Node::new(3);
            let r3 = Rc::new(RefCell::new(n3));
            let n2 = Node { data: 2, next: Some(r3.clone()) };
            let n1 = Node { data: 1, next: Some(Rc::new(RefCell::new(n2))) };
            LinkedList { head: Some(Rc::new(RefCell::new(n1))), tail: Some(r3) }
        };

        assert_eq!(list, want);
    }
}
