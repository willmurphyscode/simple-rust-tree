use helpers;
use std::fmt;

#[derive(Debug)]
pub struct Node {
    pub left_child: Option<Box<Node>>,
    pub right_child: Option<Box<Node>>,
    pub has_fruit: bool,
}

impl Node {
    pub fn random_tree() -> Node {
        let has_left = helpers::coin_flip();
        let has_right = helpers::coin_flip();
        let mut left: Option<Box<Node>> = None;
        if has_left {
            left = Some(Box::new(Node::random_tree()));
        }

        let mut right: Option<Box<Node>> = None;
        if has_right {
            right = Some(Box::new(Node::random_tree()));
        }

        Node {
            left_child: left,
            right_child: right,
            has_fruit: helpers::coin_flip(),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let has_left = self.left_child.is_some();
        let has_right = self.right_child.is_some();
        write!(
            f,
            "Node: has left: {}, right: {}, fruit: {}",
            has_left,
            has_right,
            self.has_fruit
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn name() {
        unimplemented!();
    }
}
