use std::sync::Arc;

enum Node<T> {
    Empty,
    Cons(T, Arc<Node<T>>),
}

use Node::*;
use core::borrow::Borrow;
use std::iter::FromIterator;

pub struct StackIter<'a, T> {
    cur: &'a Node<T>
}

impl <'a, T> Iterator for StackIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.cur {
            Empty => None,
            Cons(v, tail) => {
                self.cur = tail.borrow();
                Some(v)
            }
        }
    }
}

/// A persistent and immutable LIFO collection, or stack.
/// Internally uses Arc<T> pointers to implement shared ownership.
/// Note that the stack exposes its data as references. Because the data
/// is actually owned by Arc pointers, it can't directly expose it.
/// This also means that you can't implement `into_iter` on the type itself,
/// but only on a reference to it (since there is no way to express the
/// lifetime of the `&T` in this situation during compilation time).
pub struct Stack<T> {
    root: Arc<Node<T>>,
    len: i32,
}


impl<T: Clone> Stack<T> {

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    fn from_owned(node: Node<T>, len: i32) -> Self {
        Stack::from_shared(Arc::new(node), len)
    }

    fn from_shared(node: Arc<Node<T>>, len: i32) -> Self {
        Stack {
            root: node.clone(),
            len,
        }
    }

    pub fn len(&self) -> i32 {
        self.len
    }

    pub fn push(&self, v: T) -> Self {
        return Self::from_owned(Cons(v, self.root.clone()), self.len + 1);
    }

    pub fn tail(&self) -> Self {
        let (_, tail) = self.pop();
        tail
    }

    pub fn pop(&self) -> (T, Self) {
        return match self.root.borrow() {
            Empty => panic!("im empty"),
            Cons(v, tail) => (v, Self::from_shared(tail.clone(), self.len - 1))
        }
    }

    pub fn head(&self) -> Option<T> {
        match self.root.borrow() {
            Empty => None,
            Cons(v, _) => Some(v)
        }
    }

    pub fn empty() -> Self {
        return Stack::from_owned(Empty, 0);
    }

    pub fn iter(&self) -> StackIter<T> {
        StackIter {
            cur: self.root.borrow()
        }
    }

    pub fn just(x: T) -> Self {
        Stack::empty().push(x)
    }
}

impl<T: Clone> Stack<T> {
    pub fn to_vec(&self) -> Vec<T> {
        let mut vec: Vec<T> = Vec::new();
        self.iter().map(|x| x.clone()).collect::<Vec<T>>()
    }
}

impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = StackIter<'a, T>;

    fn into_iter(self) -> StackIter<'a, T> {
        self.iter()
    }
}

impl<T> Clone for Stack<T> {
    fn clone(&self) -> Self {
        Stack::from_shared(self.root.clone(), self.len)
    }
}

impl<T: Clone> From<&[T]> for Stack<T> {
    fn from(slice: &[T]) -> Self {
        slice.iter().map(|x| x.clone()).into_iter().collect::<Stack<T>>()
    }
}

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<TIter: IntoIterator<Item=T>>(iter: TIter) -> Self {
        let vec: Vec<T> = iter.into_iter().collect::<Vec<T>>();
        vec.into_iter().rev().fold(Self::empty(), |acc, cur| {
            acc.push(cur)
        })
    }
}
