use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Tree<'a> {
    tree: Option<Vec<Node<'a>>>,
}

impl<'a> Tree<'a> {
    fn new() -> Tree<'a> {
        Tree { tree: None }
    }
}

type Bits = Vec<bool>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node<'a> {
    bits: Rc<Bits>,
    text: Rc<&'a str>,
    left: Option<Rc<Node<'a>>>,
    right: Option<Rc<Node<'a>>>,
}

impl<'a> Node<'a> {
    fn new(
        bits: Rc<Bits>,
        text: Rc<&'a str>,
        left: Option<Rc<Node<'a>>>,
        right: Option<Rc<Node<'a>>>,
    ) -> Self {
        Self {
            bits,
            text,
            left,
            right,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_new_node_empty() {
        let new_tree = Tree::new();
        assert!(new_tree.tree.is_none());
    }
}
