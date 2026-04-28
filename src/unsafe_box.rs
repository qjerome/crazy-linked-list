/// Traditional linked list implementation with a little bit of unsafe blocks
/// but that should not be a safety concern. It also expose a Cursor APIs to
/// foster the development of linked list based structures needing an access
/// to cursors (such as LRU structures).
use std::{hash::Hash, marker::PhantomData, ops::Deref, ptr::NonNull};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("cursor is poisoned")]
    Poisoned,
}

/// This should never be cloned or copy
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Cursor<'c, T> {
    c: Option<UnsafeCursor<'c, T>>,
}

impl<'c, T> From<UnsafeCursor<'c, T>> for Cursor<'c, T> {
    fn from(value: UnsafeCursor<'c, T>) -> Self {
        Self { c: Some(value) }
    }
}

impl<'c, T> From<&UnsafeCursor<'c, T>> for Cursor<'c, T> {
    fn from(value: &UnsafeCursor<'c, T>) -> Self {
        Self { c: Some(*value) }
    }
}

impl<'c, T> Cursor<'c, T> {
    #[inline(always)]
    fn poison(&mut self) {
        self.c = None
    }

    pub fn value(&self) -> Option<&T> {
        self.c
            .as_ref()
            .map(|n| unsafe { n.as_node() })
            .map(|n| &n.value)
    }

    pub fn next(&self) -> Option<Cursor<'c, T>> {
        self.c
            .as_ref()
            .map(|n| unsafe { n.as_node() })
            .and_then(|n| n.next)
            .map(Cursor::from)
    }

    pub fn prev(&self) -> Option<Cursor<'c, T>> {
        self.c
            .as_ref()
            .map(|n| unsafe { n.as_node() })
            .and_then(|n| n.prev)
            .map(Cursor::from)
    }

    #[inline(always)]
    pub fn is_poisoned(&self) -> bool {
        self.c.is_none()
    }
}

#[derive(Debug)]
pub(crate) struct UnsafeCursor<'c, T> {
    ptr: NonNull<Node<'c, T>>,
    _marker: PhantomData<&'c Node<'c, T>>,
}

impl<T> Clone for UnsafeCursor<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Hash for UnsafeCursor<'_, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.ptr.as_ptr() as usize);
    }
}

impl<T> Copy for UnsafeCursor<'_, T> {}

impl<'n, T> From<Node<'n, T>> for UnsafeCursor<'n, T> {
    fn from(value: Node<'n, T>) -> Self {
        let b = Box::new(value);
        // Safe: Box::leak cannot be null
        UnsafeCursor {
            ptr: unsafe { NonNull::new_unchecked(Box::into_raw(b)) },
            _marker: PhantomData,
        }
    }
}

impl<'c, T> UnsafeCursor<'c, T> {
    #[inline(always)]
    unsafe fn take_node(mut self) -> Box<Node<'c, T>> {
        let node = unsafe { self.as_node_mut() };
        if let Some(next) = node.next.as_mut() {
            unsafe { next.as_node_mut().prev.take() };
        }

        if let Some(prev) = node.prev.as_mut() {
            unsafe { prev.as_node_mut().next.take() };
        }

        unsafe { Box::from_raw(self.ptr.as_ptr()) }
    }

    unsafe fn as_node_mut(&mut self) -> &mut Node<'c, T> {
        &mut *(unsafe { self.ptr.as_mut() })
    }

    unsafe fn as_node(&self) -> &Node<'c, T> {
        unsafe { &*(self.ptr.as_ptr() as *const Node<T>) }
    }
}

impl<T> PartialEq for UnsafeCursor<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl<T> Eq for UnsafeCursor<'_, T> {}

// replace by unsafe fn node()
impl<'c, T> Deref for UnsafeCursor<'c, T> {
    type Target = Node<'c, T>;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.ptr.as_ptr() as *const Node<T>) }
    }
}

pub struct Node<'list, T> {
    next: Option<UnsafeCursor<'list, T>>,
    prev: Option<UnsafeCursor<'list, T>>,
    value: T,
}

#[derive(Debug)]
pub struct LinkedList<'list, T> {
    head: Option<UnsafeCursor<'list, T>>,
    tail: Option<UnsafeCursor<'list, T>>,
    len: usize,
}

impl<T> Drop for LinkedList<'_, T> {
    fn drop(&mut self) {
        let mut next = self.head;
        while let Some(n) = next {
            let node = unsafe { n.take_node() };
            next = node.next;
        }
    }
}

impl<T> Default for LinkedList<'_, T> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }
}

impl<'list, T> FromIterator<T> for LinkedList<'list, T>
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

impl<'list, T> LinkedList<'list, T>
where
    T: std::fmt::Debug,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_front(&mut self, v: T) -> Cursor<'list, T> {
        let new = Node {
            prev: None,
            next: self.head,
            value: v,
        };

        let c = UnsafeCursor::from(new);

        if let Some(head) = self.head.as_mut() {
            // safe: head is always under control
            unsafe {
                head.as_node_mut().prev = Some(c);
            }
        }

        if self.tail.is_none() {
            self.tail = Some(c);
        }

        self.head = Some(c);

        self.len += 1;
        c.into()
    }

    pub fn push_back(&mut self, v: T) -> Cursor<'list, T> {
        let new = Node {
            prev: self.tail,
            next: None,
            value: v,
        };

        let c = UnsafeCursor::from(new);

        if self.head.is_none() {
            self.head = Some(c);
        }

        if let Some(tail) = self.tail.as_mut() {
            unsafe { tail.as_node_mut().next = Some(c) };
        }

        self.tail = Some(c);

        self.len += 1;
        c.into()
    }

    #[inline(always)]
    unsafe fn unlink(&mut self, node: &mut Node<T>) {
        let mut saved_prev = node.prev.take();
        let mut saved_next = node.next.take();

        if let Some(next) = saved_next.as_mut() {
            unsafe { next.as_node_mut().prev = saved_prev };
        }

        if let Some(prev) = saved_prev.as_mut() {
            unsafe { prev.as_node_mut().next = saved_next };
        }
    }

    pub(crate) unsafe fn move_front_inner(&mut self, c: UnsafeCursor<'list, T>) {
        if Some(&c) == self.head.as_ref() {
            return;
        }

        // fix previous node of head
        if let Some(hc) = self.head.as_mut() {
            unsafe { hc.as_node_mut().prev = Some(c) };
        }

        let mut node = c;

        // node becomes new head so we can take prev
        let mut saved_prev = unsafe { node.as_node_mut().prev.take() };
        let mut saved_next = node.next;
        unsafe { node.as_node_mut().next = self.head.take() };

        if Some(&c) == self.tail.as_ref() {
            self.tail = saved_prev
        }

        // node->next->prev == node->prev
        if let Some(next) = saved_next.as_mut() {
            unsafe { next.as_node_mut().prev = saved_prev };
        }

        if let Some(prev) = saved_prev.as_mut() {
            unsafe { prev.as_node_mut().next = saved_next };
        }

        self.head = Some(c);
    }

    pub fn move_front(&mut self, c: &Cursor<'list, T>) -> Result<(), Error> {
        if let Some(c) = c.c {
            unsafe { self.move_front_inner(c) };
            Ok(())
        } else {
            Err(Error::Poisoned)
        }
    }

    pub(crate) unsafe fn pop_inner(&mut self, mut c: UnsafeCursor<'list, T>) -> T {
        if self.head.as_ref() == Some(&c) {
            self.head = c.next;
        }

        if self.tail.as_ref() == Some(&c) {
            self.tail = c.prev;
        }

        unsafe { self.unlink(c.as_node_mut()) };
        let out = unsafe { c.take_node() };
        self.len -= 1;

        out.value
    }

    pub fn get(&self, c: &'list Cursor<'list, T>) -> Option<&T> {
        c.value()
    }

    pub fn pop(&mut self, sc: &mut Cursor<'list, T>) -> Result<T, Error> {
        if let Some(c) = sc.c {
            let out = unsafe { self.pop_inner(c) };
            sc.poison();
            Ok(out)
        } else {
            Err(Error::Poisoned)
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let tail = self.tail;
        // safe to use, we control tail
        Some(unsafe { self.pop_inner(tail?) })
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let head = self.head;
        // safe to use, we control head
        Some(unsafe { self.pop_inner(head?) })
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_ref(),
            prev: self.tail.as_ref(),
            len: self.len,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a UnsafeCursor<'a, T>>,
    prev: Option<&'a UnsafeCursor<'a, T>>,
    len: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        let v = self.next?;
        let out = &v.value;
        let next = v.next.as_ref();

        self.next = next;

        self.len -= 1;
        Some(out)
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        let v = self.prev?;
        let out = &v.value;
        let prev = v.prev.as_ref();

        self.prev = prev;

        self.len -= 1;
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_cursor<T: std::fmt::Debug>(c: &Cursor<T>) {
        println!(
            "val={:?} prev={:?} next={:?}",
            c.c.unwrap().value,
            c.c.unwrap().prev.as_ref().map(|n| &n.value),
            c.c.unwrap().next.as_ref().map(|n| &n.value),
        )
    }

    #[test]
    fn push_back_and_print() {
        let mut list = LinkedList::new();
        print_cursor(&list.push_back(1));
        let two = list.push_back(2);
        print_cursor(&two);
        let mut three = list.push_back(3);
        print_cursor(&list.push_back(4));

        list.move_front(&three).unwrap();
        list.move_front(&two).unwrap();
        print_cursor(&three);
        list.pop(&mut three).unwrap();

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
        let _ = list.move_front(&three);

        for value in list.iter() {
            println!("{}", value);
        }

        // Verify 3 is now head
        assert!(Cursor::from(list.head.as_ref().unwrap()) == three);
        // 3's next should be 1
        assert!(Cursor::from(three.c.unwrap().next.as_ref().unwrap()) == one);
        // 1's prev should be 3
        print_cursor(&one);
        assert!(Cursor::from(one.c.unwrap().prev.as_ref().unwrap()) == three);
        // 3's prev should be None
        assert!(three.c.unwrap().prev.is_none());
        // 2 and 4 should still be linked
        assert!(Cursor::from(two.c.unwrap().next.as_ref().unwrap()) == four);
        assert!(Cursor::from(four.c.unwrap().prev.as_ref().unwrap()) == two);
    }

    #[test]
    fn test_move_front_already_head() {
        let mut list = LinkedList::new();
        let one = list.push_back(1);
        let two = list.push_back(2);

        // Move head to front (should be no-op for structure)
        list.move_front(&one).unwrap();

        // 1 should still be head
        assert!(Cursor::from(list.head.as_ref().unwrap()) == one);
        assert!(one.c.unwrap().prev.is_none());
        assert!(Cursor::from(one.c.unwrap().next.as_ref().unwrap()) == two);
    }

    #[test]
    fn test_move_front_tail() {
        let mut list = LinkedList::new();
        let one = list.push_back(1);
        let two = list.push_back(2);
        let three = list.push_back(3);

        // Move tail (3) to front
        list.move_front(&three).unwrap();

        // 3 should be head, 2 should be tail
        assert!(Cursor::from(list.head.as_ref().unwrap()) == three);
        assert!(Cursor::from(list.tail.as_ref().unwrap()) == two);
        // 3 -> 1 -> 2
        assert!(Cursor::from(three.c.unwrap().next.as_ref().unwrap()) == one);
        assert!(Cursor::from(one.c.unwrap().next.as_ref().unwrap()) == two);
        assert!(two.c.unwrap().next.is_none());
        assert!(Cursor::from(one.c.unwrap().prev.as_ref().unwrap()) == three);
        assert!(Cursor::from(two.c.unwrap().prev.as_ref().unwrap()) == one);
        assert!(three.c.unwrap().prev.is_none());
    }

    #[test]
    fn test_move_front_single_node() {
        let mut list = LinkedList::new();
        let one = list.push_back(1);

        // Move the only node to front
        list.move_front(&one).unwrap();

        // Should still be the only node
        assert!(Cursor::from(list.head.as_ref().unwrap()) == one);
        assert!(Cursor::from(list.tail.as_ref().unwrap()) == one);
        assert!(one.c.unwrap().prev.is_none());
        assert!(one.c.unwrap().next.is_none());
    }

    #[test]
    fn test_pop_head() {
        let mut list = LinkedList::new();
        let mut one = list.push_back(1);
        let two = list.push_back(2);
        let three = list.push_back(3);

        // Pop the head node (1)
        let result = list.pop(&mut one).unwrap();
        assert_eq!(result, 1);

        // Verify 2 is now the head
        assert!(Cursor::from(list.head.as_ref().unwrap()) == two);
        assert!(Cursor::from(list.tail.as_ref().unwrap()) == three);
        assert!(two.c.unwrap().prev.is_none());
        assert!(Cursor::from(three.c.unwrap().prev.as_ref().unwrap()) == two);
    }

    #[test]
    fn test_pop_tail() {
        let mut list = LinkedList::new();
        let one = list.push_back(1);
        let two = list.push_back(2);
        let mut three = list.push_back(3);

        // Pop the tail node (3)
        let result = list.pop(&mut three).unwrap();
        assert_eq!(result, 3);

        // Verify 2 is now the tail
        assert!(Cursor::from(list.head.as_ref().unwrap()) == one);
        assert!(Cursor::from(list.tail.as_ref().unwrap()) == two);
        assert!(two.c.unwrap().next.is_none());
        assert!(Cursor::from(one.c.unwrap().next.as_ref().unwrap()) == two);
    }

    #[test]
    fn test_pop_middle() {
        let mut list = LinkedList::new();
        let one = list.push_back(1);
        let mut two = list.push_back(2);
        let three = list.push_back(3);
        let four = list.push_back(4);

        // Pop middle node (2)
        let result = list.pop(&mut two).unwrap();
        assert_eq!(result, 2);

        // Verify list integrity: 1 <-> 3 <-> 4
        assert!(Cursor::from(list.head.as_ref().unwrap()) == one);
        assert!(Cursor::from(list.tail.as_ref().unwrap()) == four);
        assert!(Cursor::from(one.c.unwrap().next.as_ref().unwrap()) == three);
        assert!(Cursor::from(three.c.unwrap().prev.as_ref().unwrap()) == one);
        assert!(Cursor::from(three.c.unwrap().next.as_ref().unwrap()) == four);
        assert!(Cursor::from(four.c.unwrap().prev.as_ref().unwrap()) == three);
    }

    #[test]
    fn test_pop_single_node() {
        let mut list = LinkedList::new();
        let one = list.push_back(1);
        let mut two = one;

        // Pop the only node
        let result = list.pop(&mut two).unwrap();
        assert_eq!(result, 1);

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
        assert_eq!(result.unwrap(), 1);

        // Verify list: 2 <-> 3
        assert!(list.head.as_ref().unwrap().value == 2);
        assert!(list.tail.as_ref().unwrap().value == 3);

        // Pop again
        let result = list.pop_front();
        assert_eq!(result.unwrap(), 2);

        // Pop last
        let result = list.pop_front();
        assert_eq!(result.unwrap(), 3);

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
        assert_eq!(result.unwrap(), 3);

        // Verify list: 1 <-> 2
        assert_eq!(list.head.as_ref().unwrap().value, 1);
        assert_eq!(list.tail.as_ref().unwrap().value, 2);

        // Pop again
        let result = list.pop_back();
        assert_eq!(result.unwrap(), 2);

        // Pop last
        let result = list.pop_back();
        assert_eq!(result.unwrap(), 1);

        // Empty
        assert!(list.pop_back().is_none());
    }

    #[test]
    fn test_pop_front_back_single() {
        let mut list = LinkedList::new();
        list.push_back(42);

        // Both should work on single element
        let result = list.pop_front();
        assert_eq!(result.unwrap(), 42);
        assert!(list.pop_back().is_none());
    }

    #[test]
    fn test_cursor_poisoned_after_pop() {
        let mut list = LinkedList::new();
        let mut one = list.push_back(1);

        // Pop one - should poison the cursor
        let result = list.pop(&mut one).unwrap();
        assert_eq!(result, 1);

        // Try to use poisoned cursor - should fail
        assert!(one.is_poisoned());
        assert!(list.move_front(&one).is_err());
        assert!(list.pop(&mut one).is_err());

        // List should still have two
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_cursor_poisoned_after_move_front() {
        let mut list = LinkedList::new();
        let two = list.push_back(2);

        // Move two to front
        list.move_front(&two).unwrap();

        // Cursor should still be valid after move_front
        assert!(!two.is_poisoned());

        // Can move it again (no-op since it's already at front)
        list.move_front(&two).unwrap();
        assert!(!two.is_poisoned());
    }

    #[test]
    fn test_multiple_pops_same_cursor() {
        let mut list = LinkedList::new();
        let mut one = list.push_back(1);

        // First pop should work
        let result = list.pop(&mut one).unwrap();
        assert_eq!(result, 1);

        // Cursor is now poisoned
        assert!(one.is_poisoned());

        // Second pop with same cursor should fail
        assert!(list.pop(&mut one).is_err());

        // Third pop should also fail
        assert!(list.pop(&mut one).is_err());
    }

    #[test]
    fn test_cursor_valid_before_any_operation() {
        let mut list = LinkedList::new();
        let one = list.push_back(1);
        let two = list.push_back(2);

        // Cursors should be valid initially
        assert!(!one.is_poisoned());
        assert!(!two.is_poisoned());
    }

    #[test]
    fn test_push_front_empty_list() {
        let mut list = LinkedList::new();

        // Push to empty list
        let one = list.push_front(1);

        // Verify list state
        assert!(list.head.as_ref() == Some(&one.c.unwrap()));
        assert!(list.tail.as_ref() == Some(&one.c.unwrap()));
        assert_eq!(list.len(), 1);
        assert_eq!(one.c.unwrap().value, 1);
        assert!(one.c.unwrap().prev.is_none());
        assert!(one.c.unwrap().next.is_none());
    }

    #[test]
    fn test_push_front_non_empty_list() {
        let mut list = LinkedList::new();
        let two = list.push_front(2);

        // Push another element to front
        let one = list.push_front(1);

        // Verify 1 is now head, 2 is tail
        assert!(list.head.as_ref() == Some(&one.c.unwrap()));
        assert!(list.tail.as_ref() == Some(&two.c.unwrap()));
        assert_eq!(list.len(), 2);

        // Verify linking: 1 -> 2, 2->prev = 1
        assert!(one.c.unwrap().next.as_ref() == Some(&two.c.unwrap()));
        assert!(two.c.unwrap().prev.as_ref() == Some(&one.c.unwrap()));
        assert!(one.c.unwrap().prev.is_none());
        assert!(two.c.unwrap().next.is_none());
    }

    #[test]
    fn test_push_front_multiple() {
        let mut list = LinkedList::new();

        // Push multiple elements
        let three = list.push_front(3);
        let two = list.push_front(2);
        let one = list.push_front(1);

        // Verify order: 1 -> 2 -> 3
        assert!(list.head.as_ref() == Some(&one.c.unwrap()));
        assert!(list.tail.as_ref() == Some(&three.c.unwrap()));
        assert_eq!(list.len(), 3);

        assert!(one.c.unwrap().next.as_ref() == Some(&two.c.unwrap()));
        assert!(two.c.unwrap().prev.as_ref() == Some(&one.c.unwrap()));
        assert!(two.c.unwrap().next.as_ref() == Some(&three.c.unwrap()));
        assert!(three.c.unwrap().prev.as_ref() == Some(&two.c.unwrap()));
        assert!(three.c.unwrap().next.is_none());
        assert!(one.c.unwrap().prev.is_none());
    }

    #[test]
    fn test_push_front_and_iter() {
        let mut list = LinkedList::new();
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);

        // Verify iteration order (should be 1, 2, 3)
        let values: Vec<_> = list.iter().collect();
        assert_eq!(values, vec![&1, &2, &3]);
    }

    #[test]
    fn test_push_front_mixed_with_push_back() {
        let mut list = LinkedList::new();

        // push_back(1), push_front(2), push_back(3), push_front(4)
        let one = list.push_back(1);
        let two = list.push_front(2);
        let three = list.push_back(3);
        let four = list.push_front(4);

        // Verify list: 4 -> 2 -> 1 -> 3
        assert!(list.head.as_ref() == Some(&four.c.unwrap()));
        assert!(list.tail.as_ref() == Some(&three.c.unwrap()));
        assert_eq!(list.len(), 4);

        assert!(four.c.unwrap().next.as_ref() == Some(&two.c.unwrap()));
        assert!(two.c.unwrap().prev.as_ref() == Some(&four.c.unwrap()));
        assert!(two.c.unwrap().next.as_ref() == Some(&one.c.unwrap()));
        assert!(one.c.unwrap().prev.as_ref() == Some(&two.c.unwrap()));
        assert!(one.c.unwrap().next.as_ref() == Some(&three.c.unwrap()));
        assert!(three.c.unwrap().prev.as_ref() == Some(&one.c.unwrap()));
        assert!(three.c.unwrap().next.is_none());
    }
}
