extern crate rust_learning;

use rust_learning::bst::BST;

fn main() {
    let mut b = BST::new();
    b.insert(50).insert(25).insert(75).insert(12).insert(95);

    println!("{:#?}", b);
    println!("DEPTH: {}", b.depth());

    // {
    //     let child = b.find(123);
    //     println!("{:#?}", child);
    // }

    // {
    //     let child = b.find(-1);
    //     println!("{:#?}", child);
    // }

    // println!("Max: {}", b.maximum().unwrap());

    let b = b.remove(50).remove(95);
    println!("{:#?}", b)
}
