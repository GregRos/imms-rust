
use crate::stack::*;

#[test]
fn empty() {
    assert_eq!(Stack::<i32>::empty().len(), 0);
}

#[test]
fn add() {
    let mut one = Stack::empty();
    one = one.push(1);
    assert_eq!(one.len(), 1);
}
#[test]
fn into_iter() {
    let stk: Stack<i32> = Stack::empty();
    let stk = stk.push(3).push(2).push(1);
    let v: Vec<i32> = stk.into_iter().map(|x| x.clone()).collect::<Vec<i32>>();
    assert_eq!(v, vec![1i32, 2, 3]);

    let mut v = Vec::new();
    for x in &stk {
        v.push(*x);
    }
    assert_eq!(v, vec![1i32, 2, 3]);
}

#[test]
fn from_iter() {
    let vec = vec![1, 2, 3];
    let vec2 = vec.clone();
    let stk = vec.into_iter().collect::<Stack<i32>>();
    assert_eq!(stk.to_vec(), vec2);
}

#[test]
fn from_slice() {
    let arr = [1, 2, 3];
    let vec = arr.to_vec();
    let stk = Stack::from(&arr[..]);
    assert_eq!(stk.to_vec(), vec)
}

