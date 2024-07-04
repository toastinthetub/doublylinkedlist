use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    data: T,
    prev: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            prev: self.tail.clone(),
            next: None,
        }));

        if let Some(tail) = &self.tail {
            tail.borrow_mut().next = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
        }

        self.tail = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        // Simplified pop method
        self.head.take().map(|old_head| {
            if let Some(new_head) = old_head.borrow_mut().next.take() {
                new_head.borrow_mut().prev.take();
                self.head = Some(new_head);
            } else {
                self.tail.take();
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().data
        })
    }
}

fn main() {
    let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    // list.push(10);
    // list.push(20);
    // list.push(5);

    println!("Popped: {:?}", list.pop());
}
