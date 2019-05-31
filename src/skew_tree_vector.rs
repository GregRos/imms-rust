
use std::sync::Arc;
use crate::stack::*;
use std::iter;
use std::iter::FromIterator;
use core::borrow::Borrow;
use BinTreeNode::*;
/// Vector or random-access list implemented using a skew binary tree.
/// Can also be described as a list of complete binary trees.
pub struct SkewTreeVector<T> {
    list: Stack<BinTree<T>>,
    size: i32
}

impl<T> SkewTreeVector<T> {

    pub fn len(&self) -> i32 {
        self.size
    }

    pub fn empty() -> SkewTreeVector<T> {
        SkewTreeVector {
            list: Stack::empty(),
            size: 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn head(&self) -> Option<&T> {
        self.list.head().map(|x| x.head())
    }

    pub fn push(&self, x: T) -> SkewTreeVector<T> {
        let SkewTreeVector {list, size} = self;
        let new_list = {
            // If there are at least two trees,
            if list.len() >= 2 {
                let (a, tail) = list.pop();
                let (b, tail) = tail.pop();
                // and they have equal size
                if a.size == b.size {
                    // then they can be merged by setting
                    // the new `v` as the root.
                    tail.push(BinTree::join(Arc::new(x), a, b))
                } else {
                    // otherwise, we just add a singleton tree to the list
                    list.push(BinTree::just(x))
                }
            } else {
                list.push(BinTree::just(x))
            }
        };
        Self {
            size: self.size + 1,
            list: new_list
        }
    }

    pub fn iter(&self) -> SkewTreeIter<T> {
        self.into_iter()
    }

    pub fn pop(&self) -> (&T, SkewTreeVector<T>) {
        let SkewTreeVector {list, ..} = self;
        let new_list = {
            let (head, tail) = list.pop();
            match head.root.borrow() {
                // if the first element is a leaf, just remove it
                Leaf(_) => tail,
                // otherwise, decompose the node and push both its children
                Parent(_, l, r) => {
                    let child_size = (head.size - 1) / 2;
                    tail.push(BinTree {
                        size: child_size,
                        root: r.clone()
                    }).push(BinTree {
                        size: child_size,
                        root: l.clone()
                    })
                }
            }
        };
        (self.head().unwrap(), Self {
            size: self.size - 1,
            list: new_list
        })
    }

    pub fn get(&self, ix: i32) -> &T {
        let SkewTreeVector {list, ..}= self;
        let mut ix = ix;
        // first, find which complete binary tree the index is in,
        for x in list {
            if ix < x.size {
                // and then search that tree for the element.
                return x.get(ix)
            } else {
                ix -= x.size;
            }
        }
        panic!("out of bounds");
    }
}

/// Iterator for the skew binary tree.
pub struct SkewTreeIter<'a, T> {
    stack_iter: StackIter<'a, BinTree<T>>,
    tree_iter: BinTreeIter<'a, T>
}

impl <'a, T> Iterator for SkewTreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        loop {
            if let x @ Some(_) = self.tree_iter.next() {
                return x
            }
            match self.stack_iter.next() {
                Some(tree) => self.tree_iter = tree.iter(),
                None => return None
            }
        }
    }
}


impl <'a, T> IntoIterator for &'a SkewTreeVector<T> {
    type Item = &'a T;
    type IntoIter = SkewTreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SkewTreeIter {
            tree_iter: BinTreeIter { nodes: Vec::new() },
            stack_iter: self.list.iter()
        }
    }
}

impl<T> FromIterator<T> for SkewTreeVector<T> {
    fn from_iter<TIter: IntoIterator<Item=T>>(iter: TIter) -> Self {
        let vec: Vec<T> = iter.into_iter().collect::<Vec<T>>();
        vec.into_iter().rev().fold(Self::empty(), |acc, cur| {
            acc.push(cur)
        })
    }
}



/// Complete binary tree used by the skew binary tree.
struct BinTree<T> {
    size: i32,
    root: Arc<BinTreeNode<T>>,
}

enum BinTreeNode<T> {
    Leaf(Arc<T>),
    Parent(Arc<T>, Arc<BinTreeNode<T>>, Arc<BinTreeNode<T>>),
}

impl<T> BinTree<T> {
    fn just(x: T) -> BinTree<T> {
        BinTree {
            size: 1,
            root: Arc::new(Leaf(Arc::new(x))),
        }
    }

    fn join(v: Arc<T>, left: &BinTree<T>, right: &BinTree<T>) -> BinTree<T> {
        BinTree {
            size: 1 + left.size + right.size,
            root: Arc::new(Parent(v, left.root.clone(), right.root.clone())),
        }
    }

    fn head(&self) -> &T {
        let v_ref = match self.root.borrow() {
            Leaf(v) => v,
            Parent(v, _, _) => v
        };
        v_ref.borrow()
    }

    pub fn get(&self, ix: i32) -> &T {
        let mut node = self.root.borrow();
        let mut size = self.size;
        let mut ix = ix;
        loop {
            match node {
                Leaf(v) | Parent(v, _, _) if ix == 0 => return v.borrow(),
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

    fn iter(&self) -> BinTreeIter<T> {
        BinTreeIter {
            nodes: vec![self.root.borrow()]
        }
    }
}

impl<T> Clone for BinTree<T> {
    fn clone(&self) -> Self {
        BinTree {
            size: self.size,
            root: self.root.clone()
        }
    }
}

/// Iterator for the complete binary trees
pub struct BinTreeIter<'a, T> {
    nodes: Vec<&'a BinTreeNode<T>>
}

impl <'a, T> Iterator for BinTreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.nodes.pop() {
            Some(Leaf(v)) => Some(v.borrow()),
            Some(Parent(v, l, r)) => {
                self.nodes.push(r.borrow());
                self.nodes.push(l.borrow());
                Some(v.borrow())
            },
            None => None
        }
    }
}



