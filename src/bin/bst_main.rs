extern crate rust_learning;

use rust_learning::bst::BST;

fn main() {
    let mut b = BST::new();
    b.insert(50).insert(25).insert(75).insert(12).insert(95);

    println!("{:#?}", b);
    println!("Max: {:#?}", b.find_max().value());
    // println!("DEPTH: {}", b.depth());

    {
        let child = b.find(&25);
        println!("{:#?}", child);
        println!("Max: {:#?}", child.find_max().value());
    }

    {
        let child = b.find(&-1);
        println!("{:#?}", child);
    }

    let b = b.remove(50).remove(95);
    println!("{:#?}", b)
}
