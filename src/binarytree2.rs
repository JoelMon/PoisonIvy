use std::fmt::Debug;
use std::rc::Rc;

type Bits = Vec<bool>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node<'a> {
    bits: Rc<Bits>,
    text: Rc<&'a str>,
    left: Option<Rc<WaxyNode<'a>>>,
    right: Option<Rc<WaxyNode<'a>>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct WaxyNode<'a> {
    node: Rc<Node<'a>>,
}

// Implementation for a leaf node.
impl<'a> WaxyNode<'a> {
    fn new() -> WaxyNode<'a> {
        Self {
            node: Rc::new(Node {
                bits: Default::default(),
                text: Default::default(),
                left: None,
                right: None,
            }),
        }
    }

    // Inserts a new leaf node in the correct branch
    fn insert(self, bits: Rc<Bits>, text: Rc<&'a str>) {
        if self.node.text == text {
            return;
        }

        // No longer relevant: Move out of an Rc error: https://stackoverflow.com/questions/72498867/cannot-move-out-of-an-rc-error-while-making-a-singly-linked-stack
        let mut branch: Option<Rc<WaxyNode>> = if text < self.node.text {
            self.node.left
        } else {
            self.node.right
        };

        match branch {
            Some(sub_node) => sub_node.insert(bits, text),

            None => {
                let new_node: Rc<WaxyNode> = Rc::new(WaxyNode {
                    node: Rc::new(Node {
                        bits,
                        text,
                        left: None,
                        right: None,
                    }),
                });
                branch = Some(new_node);
            }
        }
        todo!()
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn t_new_node() {
//         let got = Node::new();
//         let expect = Node {
//             bits: Rc::new(vec![]),
//             text: Rc::new(""),
//             left: None,
//             right: None,
//         };
//         assert_eq!(got, expect);

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
