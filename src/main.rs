mod stack;


use stack::*;
fn main() {
    let x: Stack<i32> = Stack::empty();
    let x = x.push(5);
    let x = x.push(4);
    let x = x.pop();
    println!("Here: {:?}", x.arr())
}
