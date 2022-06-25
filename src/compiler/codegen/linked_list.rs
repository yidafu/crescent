use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct LinkedList<T> {
    pub head: Link<T>,
    pub tail: Link<T>,
}

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;


#[derive(Debug, Clone)]
pub struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
        }
    }
    pub fn top(&mut self) -> Link<T> {
        self.head.take()
    }

    pub fn push(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: self.head.take(),
        }));
        if self.tail.is_none() {
            self.tail = Some(new_node.clone());
        }
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        let first_link = self.head.take();
        let next_link = match first_link {
            Some(ref node) => node.borrow_mut().next.take(),
            None => None,
        };

        self.head = next_link;
        if self.head.is_none() {
            self.tail.take();
        }

        first_link.map(|f| Rc::try_unwrap(f).ok().unwrap().into_inner().data)
    }

    pub fn peek(&mut self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|f| Ref::map(f.borrow(), |node| &node.data))
    }

    pub fn peek_node_value(node: Link<T>) -> Option<T> {
      node.map(|f| Rc::try_unwrap(f).ok().unwrap().into_inner().data)
    }

    pub fn next_node(node: Link<T>) -> Link<T> {
        Rc::try_unwrap(node.unwrap())
            .ok()
            .unwrap()
            .into_inner()
            .next
    }
}

