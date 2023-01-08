// Immutable outside, RefCell<T> provide ‘interior mutability’
// To use references instead of values, one must use the RefCell<T> type, acquiring a write lock before mutating
// RefCell<T> re for single-threaded scenarios
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct DbNode<T> {
    data: T,
    next: Option<Rc<RefCell<DbNode<T>>>>,
    prev: Option<Weak<RefCell<DbNode<T>>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    first: Option<Rc<RefCell<DbNode<T>>>>,
    last: Option<Weak<RefCell<DbNode<T>>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
      LinkedList {
            first: None,
            last: None,
        }
    }

    pub fn push_front(&mut self, data: T) {
        // Takes the value out of the option, leaving a None in its place.
        match self.first.take() {
            Some(r) => {
                // create new front object
                let new_front = Rc::new(RefCell::new(DbNode {
                    data,
                    next: Some(r.clone()),
                    prev: None,
                }));
                // set the old front prev to the new front
                let mut old_front = r.borrow_mut();
                old_front.prev = Some(Rc::downgrade(&new_front)); // Rc pointer downgrade to weak
                // set the new front as first
                self.first = Some(new_front);
            },
            None => {
                // create new object
                let new_node = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_node));
                self.first = Some(new_node);
            },
        }
    }

    pub fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(r) => {
                // create new last object
                let new_last = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: Some(r.clone()),
                }));
                // upgrade weak as weak cannot borrow_mut, we know r exist so can unwrap here
                let last_rc = Weak::upgrade(&r).unwrap();
                let mut old_last = last_rc.borrow_mut();
                // set the new last
                self.last = Some(Rc::downgrade(&new_last));
                // set the old last next to the new last object
                old_last.next = Some(new_last);
            },
            None => {
                // create new object
                let new_node = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_node));
                self.first = Some(new_node);
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_push() {
        let mut dl = LinkedList::new();
        dl.push_front(6);
        dl.push_back(5);
        dl.push_back(7);
        println!("linked list: {:?}", dl);
    }
}
