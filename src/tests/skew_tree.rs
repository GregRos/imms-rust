use crate::skew_tree_vector::*;

#[test]
fn empty() {
    let empty = SkewTreeVector::<i32>::empty();
    assert_eq!(empty.len(), 0);
}

#[test]
fn push1() {
    let just_one = SkewTreeVector::empty().push(1);
    assert_eq!(just_one.len(), 1);
    assert_eq!(just_one.head(), Some(&1));
}

#[test]
fn push4() {
    let x = SkewTreeVector::empty().push(4);
    assert_eq!(x.head().cloned(), Some(4));
    let x = x.push(3);
    assert_eq!(x.head().cloned(), Some(3));
    let x = x.push(2);
    assert_eq!(x.head().cloned(), Some(2));
    let x = x.push(1);
    assert_eq!(x.head().cloned(), Some(1));

    assert_eq!(x.len(), 4);

}

#[test]
fn into_iter() {
    let just_three = SkewTreeVector::empty().push(3).push(2).push(1);
    let vec: Vec<i32> = just_three.iter().cloned().collect();
    assert_eq!(vec, vec![1, 2, 3])
}


#[test]
fn pop() {
    let three = SkewTreeVector::empty().push(3).push(2).push(1);
    let (x, tail) = three.pop();
    assert_eq!(*x, 1);
    let (x, tail) = tail.pop();
    assert_eq!(*x, 2);
    let (x, tail) = tail.pop();
    assert_eq!(*x, 3);
    assert_eq!(tail.len(), 0);
    assert_eq!(tail.iter().cloned().collect::<Vec<i32>>(), Vec::<i32>::new());
}

#[test]
fn from_iter() {
    let big_vec = (0..5000).collect::<Vec<i32>>();
    let big_skew: SkewTreeVector<i32> = big_vec.iter().cloned().collect::<SkewTreeVector<i32>>();
    assert_eq!(big_skew.iter().cloned().collect::<Vec<i32>>(), big_vec);
}

#[test]
fn indexing() {
    let big_skew: SkewTreeVector<i32>= (1..5001).map(|x| -x).collect();
    assert_eq!(big_skew.len(), 5000);
    assert_eq!(*big_skew.get(0), -1);
    assert_eq!(*big_skew.get(500), -501);
    assert_eq!(*big_skew.get(4999), -5000);


}
