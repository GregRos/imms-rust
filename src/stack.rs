use std::sync::Arc;
enum Node<T> {
    Empty,
    Tailed(T, Arc<Node<T>>),
}

use Node::*;
use core::borrow::Borrow;

pub struct Stack<T> {
    root: Arc<Node<T>>
}

fn mk_fresh<T>(node: Node<T>) -> Stack<T> {
    Stack {
        root: Arc::new(node)
    }
}

fn mk_shared<T>(node: Arc<Node<T>>) -> Stack<T> {
    Stack {
        root: node.clone()
    }
}

impl<T: Clone> Stack<T> {
    pub fn push(&self, v: T) -> Self {
        return mk_fresh(Tailed(v, self.root.clone()));
    }

    pub fn pop(&self) -> Self {
        return mk_shared(match self.root.borrow() {
            Empty => panic!("boo"),
            Tailed(_, tail) => tail.clone()
        });
    }

    pub fn arr(&self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut node = &self.root;
        while let Tailed(v, tail) = node.borrow() {
            vec.push(v.clone());
            node = tail;
        }
        vec
    }

    pub fn empty() -> Self {
        return mk_fresh(Empty);
    }
}
