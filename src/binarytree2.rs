use std::fmt::Debug;
use std::rc::Rc;

type Bits = Vec<bool>;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Node<'a> {
    bits: Rc<Bits>,
    text: Rc<&'a str>,
    left: Option<Rc<Node<'a>>>,
    right: Option<Rc<Node<'a>>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Tree<'a> {
    root: Option<Rc<Node<'a>>>,
}

// Implementation for a leaf node.
impl<'a> Node<'a> {
    fn new() -> Self {
        Self {
            bits: Default::default(),
            text: Default::default(),
            left: None,
            right: None,
        }
    }
}
impl<'a> From<Node<'a>> for Option<Rc<Node<'a>>> {
    fn from(node: Node<'a>) -> Self {
        Some(Rc::new(node))
    }
}

impl<'a> Tree<'a> {
    fn new() -> Self {
        Self {
            root: Node::new().into(),
        }
    }

    // Inserts a new leaf node in the correct branch
    fn insert(self, bits: Rc<Bits>, text: Rc<&'a str>) -> Self {

        // Recursive terminating statement needed here

        match self.root {
            None => Tree::new().into(), // Return an new node if tree is empty
            Some(mut node) => {
                if Rc::clone(&node.text) < text { // Iter over bits goes here
                    // 
                    match node.left {
                        Some(_) => todo!(),
                        None => Rc::make_mut(&mut node),
                    }
                }else {
                    todo!()
                };
            todo!()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_new_node() {
        let got = Node::new();
        let expect = Node {
            bits: Rc::new(vec![]),
            text: Rc::new(""),
            left: None,
            right: None,
        };
        assert_eq!(got, expect);
    }

    #[test]
    fn t_new_tree() {
        let got = Tree::new();
        let expect = Tree {
            root: Some(Rc::new(Node {
                bits: vec![].into(),
                text: "".into(),
                left: None,
                right: None,
            })),
        };
        assert_eq!(got, expect);
    }
}

//         #[test]
//         fn t_1_node() {
//             let got = Node::new().insert(vec![true].into(), "wow".into());
//             let expected = Node {
//                 bits: Rc::new(vec![]),
//                 text: Rc::new(""),
//                 left: None,
//                 right: Some(Rc::new(Node {
//                     bits: Rc::new(vec![true]),
//                     text: Rc::new("wow"),
//                     left: None,
//                     right: None,
//                 })),
//             };
//             assert_eq!(got, expected);
//         }
//     }
// }
