#![feature(slice_patterns)]

use std::sync::Arc;
use crate::stack::*;

enum Node<T> {
    Leaf(T),
    Parent(T, Arc<Node<T>>, Arc<Node<T>>),
}

impl <T> Node<T> {


}

use Node::*;
use core::borrow::Borrow;

struct Tree<T> {
    size: i32,
    root: Arc<Node<T>>,
}

impl<T> Tree<T> {
    fn just(x: T) -> Tree<T> {
        Tree {
            size: 1,
            root: Arc::new(Leaf(x)),
        }
    }

    fn join(v: T, left: Tree<T>, right: Tree<T>) -> Tree<T> {
        Tree {
            size: 1 + left.size + right.size,
            root: Arc::new(Parent(v, left.root, right.root)),
        }
    }

    fn head(&self) -> &T {
        match self.root.borrow() {
            Leaf(v) => v,
            Parent(v, _, _) => v
        }
    }

    pub fn get(&self, ix: i32) -> &T {
        let mut node = self.root.borrow();
        let mut size = self.size;
        let mut ix = ix;
        loop {
            match node {
                Leaf(v) | Parent(v, _, _) if ix == 0 => return v,
                Parent(v, l, r) => {
                    ix -= 1;
                    size = (size - 1) / 2;
                    if ix < size {
                        node = l;
                    } else {
                        ix -= size;
                        node = r;
                    }
                },
                _ => panic!("out of bounds")

            }
        }
    }
}

impl<T> Clone for Tree<T> {
    fn clone(&self) -> Self {
        Tree {
            size: self.size,
            root: self.root.clone()
        }
    }
}

enum TwoTaken<'a, T> {
    None,
    One(&'a T),
    Two(&'a T, &'a T, Stack<T>),
}

fn take2<T>(tail: &Stack<T>) -> TwoTaken<T> {
    if tail.is_empty() {
        return TwoTaken::None;
    }
    let (a, tail) = tail.pop();
    if tail.is_empty() {
        return TwoTaken::One(a);
    }
    let (b, tail) = tail.pop();
    return TwoTaken::Two(a, b, tail);

}

struct SkewTree<T> {
    list: Stack<Tree<T>>,
    size: i32
}

impl<T> SkewTree<T> {
    fn push(&self, x: T) -> SkewTree<T> {
        let SkewTree {list, ..} = self;
        let new_inner = {
            match take2(list) {
                TwoTaken::Two(a, b, ref tail) if a.size == b.size =>
                    tail.push(Tree::join(x, a.clone(), b.clone())),
                _ => list.push(Tree::just(x))
            }
        };
        Self {
            size: self.size + 1,
            list: new_inner
        }
    }

    fn pop(&self) -> (&T, SkewTree<T>) {
        let SkewTree {list, ..} = self;
        let (v, new_list) = {
            let (head, tail) = list.pop();
            match head.root.borrow() {
                Leaf(v) => (v, tail),
                Parent(v, l, r) => {
                    let child_size = (head.size - 1) / 2;
                    let new_list = tail.push(Tree {
                        size: child_size,
                        root: r.clone()
                    }).push(Tree {
                        size: child_size,
                        root: l.clone()
                    });
                    (v, new_list)
                }
            }
        };
        (v, Self {
            size: self.size - 1,
            list: new_list
        })
    }

    fn get(&self, ix: i32) -> &T {
        let SkewTree {list, ..}= self;
        let i = 0;
        let mut ix = 0;
        for x in list {
            if ix <= x.size {
                return x.get(ix)
            } else {
                ix -= x.size;
            }
        }
        panic!("out of bounds");
    }
}
