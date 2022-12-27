use std::fmt::Debug;

#[derive(Debug)]
pub struct BST<T>(Option<Box<BinaryNode<T>>>);

#[derive(Debug)]
pub struct BinaryNode<T> {
    data: T,
    left: BST<T>,
    right: BST<T>,
}

impl<T> BinaryNode<T> {
    pub fn new(data: T) -> Self {
        BinaryNode {
            data,
            left: BST(None),
            right: BST(None),
        }
    }
}

impl<T> BST<T> {
    pub fn new() -> Self {
      BST(None)
    }
}

impl<T> BST<T> 
where T: PartialOrd
{
    pub fn insert(&mut self, data: T) -> bool {
        match self.0 {
            Some(ref mut node) => {
                if node.data == data {
                    return false;
                }
                if data < node.data {
                    return node.left.insert(data);
                } else {
                    return node.right.insert(data);
                }
            }
            None => {
                self.0 = Some(Box::new(BinaryNode::new(data)));
            }
        }
        true
    }
}

impl<T: Debug> BST<T> {
    pub fn print_preorder(&self, depth: i32) {
        if let Some(ref node) = self.0 {
            node.left.print_preorder(depth + 1);
            let mut space = String::new();
            for _ in 0..depth {
                space.push('.');
            }
            println!("{}{:?}", space, node.data);
            node.right.print_preorder(depth + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst_insert() {
        let mut bst = BST::new();
        bst.insert(3);
        bst.insert(2);
        bst.insert(1);
        bst.print_preorder(0);
    }
}
