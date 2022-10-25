use std::array::IntoIter;
use std::fmt::Debug;
use std::rc::Rc;

/// An immutable binary tree trait.
/// Think of it as a HashMap<Vec<bool>, T>, except it can never be mutated.
trait BinaryTree<T: Clone + Debug + Default + Eq>
where
    Self: Default,
{
    /// Return a new Self that now associates the given bits with 't'
    fn insert<Bits: IntoIterator<Item = bool>>(&self, bits: Bits, t: T) -> Self;

    /// Get the 't' back out, if it's there.
    /// If you associates
    /// If you insert 't' at blablabla, then getting blablabla should equal 't'
    fn get<Bits: IntoIterator<Item = bool>>(&self, bits: Bits) -> Option<&T>;
}

/*
    Make a recursive struct (or enum) and implement BinaryTree<T> for it.
    Recursive fn calls are OK, don't worry about a stack overflow.
    It should pass as many of the tests as possible.
    Keep in mind that a 'T' can be stored anywhere in the tree, not just at the tips.
    If you absolutely can't implement it with a recursive type, then cheating is permited.
*/

type YourBinaryTree<T> = Tree<T>;

// type Bits = dyn IntoIterator<IntoIter = Vec<bool>, Item = bool>;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Eq)]
struct Tree<T> {
    root: Node<T>,
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Eq)]
struct Node<T, Bits: IntoIterator<Item = bool>> {
    bits: Rc<Bits>,
    text: Rc<T>,
    left: Option<Rc<Node<T, Bits>>>,
    right: Option<Rc<Node<T, Bits>>>,
}

impl<T> Node<T> {
    fn new<Bits: IntoIterator<Item = bool>>(bits: Bits, t: T) -> Self {
        Node {
            bits: Rc::new(bits),
            text: todo!(),
            left: todo!(),
            right: todo!(),
        }
    }
}

impl<T: Eq + Debug + Default + Clone + std::cmp::PartialOrd> BinaryTree<T> for Node<T> {
    // Inserts a new `Node` into the tree.
    fn insert<Bits: IntoIterator<Item = bool>>(&self, bits: Bits, t: T) -> Self {
        todo!()
    }

    fn get<Bits: IntoIterator<Item = bool>>(&self, bits: Bits) -> Option<&T> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_binary_tree_1() {
        let bt: YourBinaryTree<&str> = Default::default();

        // Added test for default state
        let bt1 = bt.insert(Some(false), "");
        let bt1 = bt.insert(Some(false), "foo");
        // let bt2 = bt.insert(Some(false), "bar");
        // assert_eq!(bt1.get(Some(false)), Some(&"foo"));
        // assert_eq!(bt2.get(Some(false)), Some(&"bar"));
        // assert_eq!(bt, Default::default());
        // assert_eq!(
        //     bt1.insert(Some(true), "true foo"),
        //     bt.insert(Some(false), "foo").insert(Some(true), "true foo")
        // );
    }

    // #[test]
    // fn test_binary_2() {
    //     let bt: YourBinaryTree<&str> = Default::default();

    //     let bt_a = bt.insert([false, false], "baz");

    //     assert_eq!(bt_a.get(Some(false)), None);
    //     assert_eq!(bt_a.get([false, false].into_iter()), Some(&"baz"));
    // }

    // #[test]
    // fn test_binary_3() {
    //     let bt: YourBinaryTree<&str> = Default::default();

    //     let bt1 = bt.insert(Some(false), "foo");
    //     let bt2 = bt.insert(Some(false), "bar");

    //     let bt_a = bt.insert([false, false], "baz");

    //     let bt1_a = bt1.insert([false, false].into_iter(), "baz");

    //     println!("{:?}", bt);

    //     println!("{:?}", bt_a.insert(Some(false), "hehe"));
    //     assert_eq!(bt_a.insert(Some(false), "foo"), bt1_a);
    // }
}
