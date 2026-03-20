// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

use std::ptr::NonNull;

pub struct Node<T> {
    data: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,

    pub count: usize,
}

pub struct Cursor<'a, T> {
    src: *mut LinkedList<T>,
    curr: Option<NonNull<Node<T>>>,
    phantom: std::marker::PhantomData<&'a T>,
}

pub struct Iter<'a, T> {
    cur: Option<NonNull<Node<T>>>,
    phantom: std::marker::PhantomData<&'a T>,
}

unsafe impl<T> Send for LinkedList<T> {}
unsafe impl<T> Sync for LinkedList<T> {}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            count: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn cursor_front(&mut self) -> Cursor<T> {
        Cursor {
            src: self,
            curr: self.head,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn cursor_back(&mut self) -> Cursor<T> {
        Cursor {
            src: self,
            curr: self.tail,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            cur: self.head,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Cursor<'_, T> {
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.curr
            .as_mut()
            .map(|node| unsafe { &mut node.as_mut().data })
    }

    pub fn next(&mut self) -> Option<&mut T> {
        match self.curr {
            None => None,
            Some(curr) => {
                let next = unsafe { curr.as_ref().next };
                self.curr = next;
                next.map(|mut node| unsafe { &mut node.as_mut().data })
            }
        }
    }

    pub fn prev(&mut self) -> Option<&mut T> {
        match self.curr {
            None => None,
            Some(curr) => {
                let prev = unsafe { curr.as_ref().prev };
                self.curr = prev;
                prev.map(|mut node| unsafe { &mut node.as_mut().data })
            }
        }
    }

    pub fn take(&mut self) -> Option<T> {
        let curr = self.curr.take()?;
        unsafe {
            (*self.src).count -= 1;

            let node = curr.as_ptr();
            let next = (*node).next;
            let prev = (*node).prev;

            if let Some(p) = prev {
                (*p.as_ptr()).next = next;
            } else {
                (*self.src).head = next;
            }

            if let Some(n) = next {
                (*n.as_ptr()).prev = prev;
            } else {
                (*self.src).tail = prev;
            }

            self.curr = next.or(prev);

            Some(Box::from_raw(node).data)
        }
    }

    pub fn insert_after(&mut self, element: T) {
        unsafe {
            (*self.src).count += 1;
        }

        let new_node = Box::into_raw(Box::new(Node {
            data: element,
            next: None,
            prev: None,
        }));
        let new_node = NonNull::new(new_node).unwrap();

        match self.curr {
            None => {
                self.curr = Some(new_node);
                unsafe {
                    (*self.src).head = Some(new_node);
                    (*self.src).tail = Some(new_node);
                }
            }
            Some(curr) => unsafe {
                let next = curr.as_ref().next;
                if let Some(n) = next {
                    (*n.as_ptr()).prev = Some(new_node);
                } else {
                    (*self.src).tail = Some(new_node);
                }
                (*new_node.as_ptr()).next = next;
                (*new_node.as_ptr()).prev = Some(curr);
                (*curr.as_ptr()).next = Some(new_node);
            },
        }
    }

    pub fn insert_before(&mut self, element: T) {
        unsafe {
            (*self.src).count += 1;
        }

        let new_node = Box::into_raw(Box::new(Node {
            data: element,
            next: None,
            prev: None,
        }));
        let new_node = NonNull::new(new_node).unwrap();

        match self.curr {
            None => {
                self.curr = Some(new_node);
                unsafe {
                    (*self.src).head = Some(new_node);
                    (*self.src).tail = Some(new_node);
                }
            }
            Some(curr) => unsafe {
                let prev = curr.as_ref().prev;
                if let Some(p) = prev {
                    (*p.as_ptr()).next = Some(new_node);
                } else {
                    (*self.src).head = Some(new_node);
                }
                (*new_node.as_ptr()).next = Some(curr);
                (*new_node.as_ptr()).prev = prev;
                (*curr.as_ptr()).prev = Some(new_node);
            },
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.cur.map(|cur| {
            let node = unsafe { cur.as_ref() };
            self.cur = node.next;
            &node.data
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut curr = self.head;
        while let Some(node) = curr {
            unsafe {
                let next = node.as_ref().next;
                drop(Box::from_raw(node.as_ptr()));
                curr = next;
            }
        }
    }
}
