use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

type Bits = Vec<bool>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node<'a> {
    bits: Rc<Bits>,
    text: Rc<&'a str>,
    left: Option<Rc<Node<'a>>>,
    right: Option<Rc<Node<'a>>>,
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
    // Inserts a new leaf node in the correct branch
    fn insert(self, bits: Rc<Bits>, text: Rc<&'a str>)  {
        if self.text == text {
            return;
        }

        let mut target = if text < self.text {
            self.left
        } else {
            self.right
        };

        match target {
            Some(subnode) => subnode.insert(bits, text)
            ,
            None => {
                let new_node = Rc::new(Node {
                    bits,
                    text,
                    left: None,
                    right: None,
                });
                let wrapped = Some(new_node);
                target = wrapped;
            }
        }
        todo!()
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

        #[test]
        fn t_1_node() {
            let got = Node::new().insert(vec![true].into(), "wow".into());
            let expected= Node {
                bits: Rc::new(vec![]),
                text: Rc::new(""),
                left: None,
                right: Some(Rc::new(Node {
                    bits: Rc::new(vec![true]),
                    text: Rc::new("wow"),
                    left: None,
                    right: None,
                })),
            };
            assert_eq!(got, expected);
        }
    }
}
