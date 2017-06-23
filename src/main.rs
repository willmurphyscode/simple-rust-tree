extern crate rand;

mod node; 
mod helpers;

fn recursive_count_fruit(node: Option<Box<node::Node>>) -> i32 {
    match node {
        Some(val) => {
            let our_fruit = if val.has_fruit { 1 } else { 0 };
            let unboxed = *val; 
            println!("{}", unboxed);
            (our_fruit 
                + recursive_count_fruit(unboxed.left_child)
                + recursive_count_fruit(unboxed.right_child))
        },
        None => {
            0
        }
    }
}

fn count_fruit(root: node::Node) -> i32 {
    recursive_count_fruit(Some(Box::new(root)))
}

fn main() {
    let root = node::Node::random_tree();
    let count = count_fruit(root);
    println!("There were {} pieces of fruit", count);
}
