/// This is a safe LinkedList implementation using classical implementation.
/// As circular references is complicated in Safe Rust we have to make an extensive use
/// of Rc and RefCell for interior mutability.
use std::{cell::RefCell, ops::Deref, rc::Rc};

// Note: Cursor needs to use Weak instead of Rc not to hold ref back to the
// Nodes if the List is dropped. I don't bother implementing it with Weak as
// this implementation does not seem to be viable.
#[derive(Debug)]
pub struct Cursor<T>(Rc<RefCell<Node<T>>>);

#[allow(dead_code)]
fn print_cursor<T: std::fmt::Debug>(c: &Cursor<T>) {
    println!(
        "val={:?} prev={:?} next={:?}",
        c.borrow().value,
        c.borrow().prev.as_ref().map(|n| n.borrow().value.clone()),
        c.borrow().next.as_ref().map(|n| n.borrow().value.clone()),
    )
}

impl<T> PartialEq for Cursor<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ptr() == other.0.as_ptr()
    }
}

impl<T> Clone for Cursor<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Deref for Cursor<T> {
    type Target = Rc<RefCell<Node<T>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<Rc<RefCell<Node<T>>>> for Cursor<T> {
    fn from(value: Rc<RefCell<Node<T>>>) -> Self {
        Self(value)
    }
}

impl<T> Cursor<T> {
    fn value(&self) -> Rc<T> {
        self.borrow().value.clone()
    }
}

/// TODO
#[derive(Debug)]
pub struct Node<T> {
    prev: Option<Cursor<T>>,
    next: Option<Cursor<T>>,
    value: Rc<T>,
}

/// TODO
#[derive(Debug, Clone)]
pub struct LinkedList<T> {
    head: Option<Cursor<T>>,
    tail: Option<Cursor<T>>,
    len: usize,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(cursor) = current {
            let mut node = cursor.borrow_mut();
            let next = node.next.take();
            node.prev.take();
            current = next;
        }
        self.tail.take();
    }
}

/// TODO
pub struct Iter<T> {
    next: Option<Cursor<T>>,
    prev: Option<Cursor<T>>,
    len: usize,
}

impl<T> Iterator for Iter<T> {
    type Item = Rc<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        let v = self.next.as_ref()?.borrow();
        let out = v.value.clone();
        let next = v.next.clone();
        drop(v);

        self.next = next;

        self.len -= 1;
        Some(out)
    }
}

impl<T> DoubleEndedIterator for Iter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        let v = self.prev.as_ref()?.borrow();
        let out = v.value.clone();
        let prev = v.prev.clone();
        drop(v);

        self.prev = prev;

        self.len -= 1;
        Some(out)
    }
}

impl<T> FromIterator<T> for LinkedList<T>
where
    T: std::fmt::Debug,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut l = LinkedList::new();
        for t in iter {
            l.push_back(t);
        }
        l
    }
}

impl<T> LinkedList<T>
where
    T: std::fmt::Debug,
{
    pub fn new() -> Self {
        Default::default()
    }

    /// TODO
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.clone(),
            prev: self.tail.clone(),
            len: self.len,
        }
    }

    /// TODO
    pub fn push_back(&mut self, v: T) -> Cursor<T> {
        let new = Rc::new(RefCell::new(Node {
            prev: self.tail.clone(),
            next: None,
            value: Rc::new(v),
        }));

        if self.head.is_none() {
            self.head = Some(Cursor(new.clone()));
        }

        if let Some(tail) = self.tail.as_mut() {
            let mut tail = tail.borrow_mut();
            tail.next = Some(Cursor(new.clone()));
        }

        self.tail = Some(Cursor(new.clone()));

        self.len += 1;
        Cursor(new)
    }

    /// TODO
    pub fn push_front(&mut self, v: T) -> Cursor<T> {
        let new = Rc::new(RefCell::new(Node {
            prev: None,
            next: self.head.clone(),
            value: Rc::new(v),
        }));

        if self.tail.is_none() {
            self.tail = Some(Cursor(new.clone()));
        }

        if let Some(head) = self.head.as_mut() {
            let mut head = head.borrow_mut();
            head.prev = Some(Cursor(new.clone()));
        }

        self.head = Some(Cursor(new.clone()));

        self.len += 1;
        Cursor(new)
    }

    #[inline(always)]
    fn unlink(&mut self, c: Cursor<T>) {
        let mut node = c.borrow_mut();

        let mut saved_prev = node.prev.take();
        let mut saved_next = node.next.take();

        if let Some(next) = saved_next.as_mut() {
            next.borrow_mut().prev = saved_prev.clone();
        }

        if let Some(prev) = saved_prev.as_mut() {
            prev.borrow_mut().next = saved_next;
        }
    }

    /// TODO
    pub fn move_front(&mut self, c: Cursor<T>) {
        if Some(&c) == self.head.as_ref() {
            return;
        }

        // fix previous node of head
        if let Some(hc) = self.head.as_mut() {
            hc.borrow_mut().prev = Some(c.clone());
        }

        let mut node = c.borrow_mut();

        // node becomes new head so we can take prev
        let mut saved_prev = node.prev.take();
        let mut saved_next = node.next.clone();
        node.next = self.head.take();

        if Some(&c) == self.tail.as_ref() {
            self.tail = saved_prev.clone()
        }

        // node->next->prev == node->prev
        if let Some(next) = saved_next.as_mut() {
            next.borrow_mut().prev = saved_prev.clone();
        }

        if let Some(prev) = saved_prev.as_mut() {
            prev.borrow_mut().next = saved_next;
        }

        drop(node);

        self.head = Some(c);
    }

    pub fn get(&self, c: Cursor<T>) -> Rc<T> {
        c.value()
    }

    /// TODO
    pub fn pop(&mut self, c: Cursor<T>) -> Rc<T> {
        if self.head.as_ref() == Some(&c) {
            self.head = c.borrow_mut().next.clone();
        }

        if self.tail.as_ref() == Some(&c) {
            self.tail = c.borrow_mut().prev.clone();
        }

        let out = c.borrow().value.clone();
        self.unlink(c);

        out
    }

    /// TODO
    pub fn pop_back(&mut self) -> Option<Rc<T>> {
        let tail = self.tail.clone();
        Some(self.pop(tail?))
    }

    /// TODO
    pub fn pop_front(&mut self) -> Option<Rc<T>> {
        let head = self.head.clone();
        Some(self.pop(head?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_cursor<T: std::fmt::Debug>(c: &Cursor<T>) {
        println!(
            "val={:?} prev={:?} next={:?}",
            c.0.borrow().value,
            c.0.borrow().prev.as_ref().map(|n| n.borrow().value.clone()),
            c.0.borrow().next.as_ref().map(|n| n.borrow().value.clone()),
        )
    }

    #[test]
    fn push_back_and_print() {
        let mut list = LinkedList::new();
        print_cursor(&list.push_back(1));
        let two = list.push_back(2);
        print_cursor(&two);
        let three = list.push_back(3);
        print_cursor(&list.push_back(4));

        list.move_front(three.clone());
        list.move_front(two.clone());
        print_cursor(&three);
        list.pop(three);

        for value in list.iter() {
            println!("{}", value);
        }
    }

    #[test]
    fn test_move_front_from_middle() {
        let mut list = LinkedList::new();
        let one = list.push_back(1);
        let two = list.push_back(2);
        let three = list.push_back(3);
        let four = list.push_back(4);

        // Move node 3 to front
        list.move_front(three.clone());

        for value in list.iter() {
            println!("{}", value);
        }

        // Verify 3 is now head
        assert!(list.head.as_ref() == Some(&three));
        // 3's next should be 1
        assert!(three.borrow().next.as_ref() == Some(&one));
        // 1's prev should be 3
        print_cursor(&one);
        assert!(one.borrow().prev.as_ref() == Some(&three));
        // 3's prev should be None
        assert!(three.borrow().prev.is_none());
        // 2 and 4 should still be linked
        assert!(two.borrow().next.as_ref() == Some(&four));
        assert!(four.borrow().prev.as_ref() == Some(&two));
    }

    #[test]
    fn test_move_front_already_head() {
        let mut list = LinkedList::new();
        let one = list.push_back(1);
        let two = list.push_back(2);

        // Move head to front (should be no-op for structure)
        list.move_front(one.clone());

        // 1 should still be head
        assert!(list.head.as_ref() == Some(&one));
        assert!(one.borrow().prev.is_none());
        assert!(one.borrow().next.as_ref() == Some(&two));
    }

    #[test]
    fn test_move_front_tail() {
        let mut list = LinkedList::new();
        let one = list.push_back(1);
        let two = list.push_back(2);
        let three = list.push_back(3);

        // Move tail (3) to front
        list.move_front(three.clone());

        // 3 should be head, 2 should be tail
        assert!(list.head.as_ref() == Some(&three));
        assert!(list.tail.as_ref() == Some(&two));
        // 3 -> 1 -> 2
        assert!(three.borrow().next.as_ref() == Some(&one));
        assert!(one.borrow().next.as_ref() == Some(&two));
        assert!(two.borrow().next.is_none());
        assert!(one.borrow().prev.as_ref() == Some(&three));
        assert!(two.borrow().prev.as_ref() == Some(&one));
        assert!(three.borrow().prev.is_none());
    }

    #[test]
    fn test_move_front_single_node() {
        let mut list = LinkedList::new();
        let one = list.push_back(1);

        // Move the only node to front
        list.move_front(one.clone());

        // Should still be the only node
        assert!(list.head.as_ref() == Some(&one));
        assert!(list.tail.as_ref() == Some(&one));
        assert!(one.borrow().prev.is_none());
        assert!(one.borrow().next.is_none());
    }

    #[test]
    fn test_pop_head() {
        let mut list = LinkedList::new();
        let one = list.push_back(1);
        let two = list.push_back(2);
        let three = list.push_back(3);

        // Pop the head node (1)
        let result = list.pop(one.clone());
        assert_eq!(*result, 1);

        // Verify 2 is now the head
        assert!(list.head.as_ref() == Some(&two));
        assert!(list.tail.as_ref() == Some(&three));
        assert!(two.borrow().prev.is_none());
        assert!(three.borrow().prev.as_ref() == Some(&two));
    }

    #[test]
    fn test_pop_tail() {
        let mut list = LinkedList::new();
        let one = list.push_back(1);
        let two = list.push_back(2);
        let three = list.push_back(3);

        // Pop the tail node (3)
        let result = list.pop(three.clone());
        assert_eq!(*result, 3);

        // Verify 2 is now the tail
        assert!(list.head.as_ref() == Some(&one));
        assert!(list.tail.as_ref() == Some(&two));
        assert!(two.borrow().next.is_none());
        assert!(one.borrow().next.as_ref() == Some(&two));
    }

    #[test]
    fn test_pop_middle() {
        let mut list = LinkedList::new();
        let one = list.push_back(1);
        let two = list.push_back(2);
        let three = list.push_back(3);
        let four = list.push_back(4);

        // Pop middle node (2)
        let result = list.pop(two.clone());
        assert_eq!(*result, 2);

        // Verify list integrity: 1 <-> 3 <-> 4
        assert!(list.head.as_ref() == Some(&one));
        assert!(list.tail.as_ref() == Some(&four));
        assert!(one.borrow().next.as_ref() == Some(&three));
        assert!(three.borrow().prev.as_ref() == Some(&one));
        assert!(three.borrow().next.as_ref() == Some(&four));
        assert!(four.borrow().prev.as_ref() == Some(&three));
    }

    #[test]
    fn test_pop_single_node() {
        let mut list = LinkedList::new();
        let one = list.push_back(1);

        // Pop the only node
        let result = list.pop(one.clone());
        assert_eq!(*result, 1);

        // List should be empty
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn test_pop_front() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Pop from front
        let result = list.pop_front();
        assert!(result.is_some());
        assert_eq!(*result.unwrap(), 1);

        // Verify list: 2 <-> 3
        assert!(list.head.as_ref().unwrap().value().as_ref() == &2);
        assert!(list.tail.as_ref().unwrap().value().as_ref() == &3);

        // Pop again
        let result = list.pop_front();
        assert_eq!(*result.unwrap(), 2);

        // Pop last
        let result = list.pop_front();
        assert_eq!(*result.unwrap(), 3);

        // Empty
        assert!(list.pop_front().is_none());
    }

    #[test]
    fn test_pop_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Pop from back
        let result = list.pop_back();
        assert!(result.is_some());
        assert_eq!(*result.unwrap(), 3);

        // Verify list: 1 <-> 2
        assert_eq!(list.head.as_ref().unwrap().value().as_ref(), &1);
        assert_eq!(list.tail.as_ref().unwrap().value().as_ref(), &2);

        // Pop again
        let result = list.pop_back();
        assert_eq!(*result.unwrap(), 2);

        // Pop last
        let result = list.pop_back();
        assert_eq!(*result.unwrap(), 1);

        // Empty
        assert!(list.pop_back().is_none());
    }

    #[test]
    fn test_pop_front_back_single() {
        let mut list = LinkedList::new();
        list.push_back(42);

        // Both should work on single element
        let result = list.pop_front();
        assert_eq!(*result.unwrap(), 42);
        assert!(list.pop_back().is_none());
    }
}
