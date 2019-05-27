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
