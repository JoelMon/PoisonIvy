use std::fmt::Debug;
use std::rc::Rc;

type Bits = Vec<bool>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node<'a> {
    bits: Rc<Bits>,
    text: Rc<&'a str>,
    left: Option<Rc<Node<'a>>>,
    right: Option<Rc<Node<'a>>>,
}

impl<'a> Node<'a> {
    fn new() -> Self {
        Self {
            bits: Default::default(),
            text: Default::default(),
            left: None,
            right: None,
        }
    }

    fn insert(&self, bits: Rc<Bits>, text: Rc<&'a str>) {
        if self.text == text {
            return;
        }
        let target = if text < self.text {
            &self.left
        } else {
            &self.right
        };
        match target {
            Some(subnode) => subnode.insert(bits, text),
            None => {
                let new_node = Node {
                    bits,
                    text,
                    left: None,
                    right: None,
                };
                let wrapped = Some(new_node);

                *target = wrapped;
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
}
