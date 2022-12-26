use std::fmt::{Debug, Display, Formatter, Result};

/// BinaryTree
/// 
/// This crate implements a BinaryTree data structure with depth,
/// level order, left/right side view, build a complete tree with count nodes algorithms.
/// 
#[derive(Debug)]
pub struct BinaryTree<T>(Option<Box<BinaryNode<T>>>);

#[derive(Debug)]
pub struct BinaryNode<T> {
    data: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T> BinaryNode<T> {
    pub fn new(data: T) -> Self {
        BinaryNode {
            data,
            left: BinaryTree(None),
            right: BinaryTree(None),
        }
    }
}

impl<T> BinaryTree<T> 
where T: Copy + Debug + 'static
{
    /// Create a new BinaryTree
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::BinaryTree;
    /// 
    /// let mut tree = BinaryTree::new();
    /// let v = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5), Some(6)];
    /// tree.insert(&v);
    /// ```
    /// 
    pub fn new() -> Self {
        BinaryTree(None)
    }

    /// Build a BinaryTree by a Vector
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::BinaryTree;
    /// 
    /// let mut tree = BinaryTree::new();
    /// let v = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5), Some(6)];
    /// tree.insert(&v);
    /// 
    /// ```
    /// 
    pub fn insert(&mut self, elements: &Vec<Option<T>>) {
        if elements.len() == 0 {
            return;
        }
        // let sides = vec!["left", "right"];
        let mut queue = Vec::new();
        // root node
        self.0 = Some(Box::new(BinaryNode {
            data: elements[0].unwrap(),
            left: BinaryTree(None),
            right: BinaryTree(None),
        }));
        let mut i = 1;
        queue.push(self);

        while queue.len() > 0 {
            queue.rotate_left(1);
            let current = queue.pop().unwrap();
            if let Some(ref mut node) = current.0 {
                // val is not None, insert left child
                if let Some(val) = elements.get(i).unwrap() {
                    node.left = BinaryTree(Some(Box::new(BinaryNode {
                        data: *val,
                        left: BinaryTree(None),
                        right: BinaryTree(None),
                    })));
                }
                i += 1;
                if i >= elements.len() {
                    return;
                }
                // check if left is None
                if node.left.0.is_some() {
                    queue.push(&mut node.left);
                }
                // if val is not None, insert right child
                if let Some(val) = elements.get(i).unwrap() {
                    node.right = BinaryTree(Some( Box::new(BinaryNode {
                        data: *val,
                        left: BinaryTree(None),
                        right: BinaryTree(None),
                    })));
                }
                i += 1;
                if i >= elements.len() {
                    return;
                }
                if node.right.0.is_some() {
                    queue.push(&mut node.right);
                }
            }
        }
    }

    /// Build a complete BinaryTree by a Vector
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::BinaryTree;
    /// 
    /// let mut tree = BinaryTree::new();
    /// let v = vec![1, 2, 3, 4, 5, 6, 7];
    /// tree.insert_as_complete(&v);
    /// 
    /// ```
    /// 
    pub fn insert_as_complete(&mut self, elements: &Vec<T>) {
        if elements.len() == 0 {
            return;
        }
        self.0 = Some(Box::new(BinaryNode {
            data: *elements.get(0).unwrap(),
            left: BinaryTree(None),
            right: BinaryTree(None),
        }));
        let mut queue = Vec::new();
        queue.push(self);
        let mut count = 1;

        while queue.len() > 0 {
            queue.rotate_left(1);
            let current = queue.pop().unwrap();

            if let Some(ref mut node) = current.0 {
                // insert left child
                if let Some(&val) = elements.get(count) {
                    node.left = BinaryTree(Some(Box::new(BinaryNode::new(val))));
                }
                count += 1;
                if count >= elements.len() {
                    return;
                }
                if node.left.0.is_some() {
                    queue.push(&mut node.left);
                }
                // insert right child
                if let Some(&val) = elements.get(count) {
                    node.right = BinaryTree(Some(Box::new(BinaryNode::new(val))));
                }
                count += 1;
                if count >= elements.len() {
                    return;
                }
                if node.right.0.is_some() {
                    queue.push(&mut node.right);
                }
            }
        }
    }

    /// Traverse a BinaryTree in preorder
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::BinaryTree;
    /// 
    /// let mut tree = BinaryTree::new();
    /// let v = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5), Some(6)];
    /// tree.insert(&v);
    /// tree.print_preorder(0);
    /// 
    /// ```
    /// 
    pub fn print_preorder(&self, depth: usize) {
        if let Some(node) = &self.0 {
            // print the left
            node.left.print_preorder(depth + 1);
            // print the data
            let mut space = String::new();
            for _ in 0..depth {
                space.push('.');
            }
            println!("{}{:?}", space, node.data);
            // print the right
            node.right.print_preorder(depth + 1);
        }
    }

    /// Get the depth of the BinaryTree
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::BinaryTree;
    /// 
    /// let mut tree = BinaryTree::new();
    /// let v = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5), Some(6)];
    /// tree.insert(&v);
    /// 
    /// let depth = tree.depth();
    /// println!("depth: {}", depth);
    /// assert_eq!(depth, 4);
    /// ```
    /// 
    pub fn depth(&self) -> i32 {
        if let Some(ref node) = self.0 {
            if node.left.0.is_none() && node.right.0.is_none() {
                return 1;
            }
            let left_depth = 1 + node.left.depth();
            let right_depth = 1 + node.right.depth();
            if left_depth > right_depth {
                return left_depth;
            }
            return right_depth;
        }
        0
    }

    /// Get the level order of the BinaryTree
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::BinaryTree;
    /// 
    /// let mut tree = BinaryTree::new();
    /// let v = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5), Some(6)];
    /// tree.insert(&v);
    /// 
    /// let level_order = tree.level_order();
    /// println!("level order: {:?}", level_order);
    /// assert_eq!(level_order, vec![vec![1], vec![2, 3], vec![4, 5], vec![6]].to_vec());
    /// ```
    /// 
    pub fn level_order(&self) -> Vec<Vec<T>> {
        if self.0.is_none() {
            return Vec::new();
        }
        let mut queue = Vec::new();
        let mut levels = Vec::new();
        queue.push(self);

        while queue.len() > 0 {
            let mut count = 0;
            let current_level_size = queue.len();
            let mut current_level_values = Vec::new();

            // traverse all the nodes of current level, after look, the queue will hold the nodes of next level
            while count < current_level_size {
                queue.rotate_left(1);
                let current = queue.pop().unwrap();
                if let Some(ref node) = current.0 {
                    current_level_values.push(node.data);
                    count += 1;

                    if node.left.0.is_some() {
                        queue.push(&node.left);
                    }
                    if node.right.0.is_some() {
                        queue.push(&node.right);
                    }
                }
            }
            levels.push(current_level_values);
        }
        levels
    }

    /// Get the right side view of the BinaryTree
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::BinaryTree;
    /// 
    /// let mut tree = BinaryTree::new();
    /// let v = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5), Some(6)];
    /// tree.insert(&v);
    /// 
    /// let mut res: Vec<i32> = Vec::new();
    /// tree.right_side_view(0, &mut res);
    /// println!("right side view: {:?}", res);
    /// assert_eq!(res, vec![1, 3, 5, 6]);
    /// ```
    /// 
    pub fn right_side_view(&self, depth: usize, res: &mut Vec<T>) {
        if let Some(ref node) = self.0 {
            let last_level = res.len();
            if depth >= last_level {  
                res.push(node.data);
            }
            if node.right.0.is_some() {
                node.right.right_side_view(depth + 1, res);
            }
            if node.left.0.is_some() {
                node.left.right_side_view(depth + 1, res);
            }
        } else {
            return;
        }
    }

    /// Get the left side view of the BinaryTree
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::BinaryTree;
    /// 
    /// let mut tree = BinaryTree::new();
    /// let v = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5), Some(6)];
    /// tree.insert(&v);
    /// 
    /// let mut res: Vec<i32> = Vec::new();
    /// tree.left_side_view(0, &mut res);
    /// println!("left side view: {:?}", res);
    /// assert_eq!(res, vec![1, 2, 4, 6]);
    /// ```
    ///
    pub fn left_side_view(&self, depth: usize, res: &mut Vec<T>) {
        if let Some(ref node) = self.0 {
            let current_level = res.len();
            if depth >= current_level {
                res.push(node.data);
            }
            if node.left.0.is_some() {
                node.left.left_side_view(depth + 1, res);
            }
            if node.right.0.is_some() {
                node.right.left_side_view(depth + 1, res);
            }
        } else {
            return;
        }
    }

    fn node_exists(&self, idx_to_find: i32, height: i32) -> bool {
        let mut left = 0;
        let mut level = 0;
        let mut right = (2 as i32).pow(height as u32) as i32 - 1;
        let mut node = self;

        while level < height {
            let mid = (((left + right) as f32)/(2 as f32)).ceil() as i32;
            if idx_to_find >= mid {
                left = mid;
                if let Some(ref binary_node ) = node.0 {
                    node = &binary_node.right;
                }
            } else {
                right = mid - 1;
                if let Some(ref binary_node) = node.0 {
                    node = &binary_node.left;
                }
            }
            level += 1;
        }
        if node.0.is_none() {
            return false;
        }
        true
    }

    /// Get the height of the BinaryTree
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::BinaryTree;
    /// 
    /// let mut tree = BinaryTree::new();
    /// let v = vec![1, 2, 3, 4, 5, 6, 7];
    /// tree.insert_as_complete(&v);
    /// 
    /// let height = tree.height();
    /// println!("height: {}", 2);
    /// assert_eq!(height, 2);
    /// ```
    /// 
    pub fn height(&self) -> i32{
        self.depth() - 1
    }

    /// Get the nodes count of the complete BinaryTree
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::BinaryTree;
    /// 
    /// let mut tree = BinaryTree::new();
    /// let v = vec![1, 2, 3, 4, 5, 6, 7];
    /// tree.insert_as_complete(&v);
    /// 
    /// let count = tree.count_nodes();
    /// println!("count: {}", count);
    /// assert_eq!(count, 7);
    /// ```
    /// 
    pub fn count_nodes(&self) -> i32 {
        if self.0.is_none() {
            return 0;
        }
        let height = self.height();
        if height == 0 {
            return 1;
        }
        let mut left = 0;
        let upper_count = (2 as i32).pow(height as u32) - 1;
        let mut right = upper_count;

        while left < right {
            let idx_to_find = (((left + right) as f32)/(2 as f32)).ceil() as i32;

            if self.node_exists(idx_to_find, height) {
                left = idx_to_find;
            } else {
                right = idx_to_find - 1;
            }
        }
        upper_count + left + 1
    }
}

impl<T: Copy + Debug + 'static> Display for BinaryTree<T> {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_binary_node() {
        let node = BinaryNode::new(1);
        println!("node: {:?}", node);
        // panic!();
    }

    #[test]
    fn test_binary_tree() {
        let mut tree = BinaryTree::new();

        let v = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)];
        tree.insert(&v);
        println!("tree: {}", tree);
        tree.print_preorder(0);
        // panic!();
    }

    #[test]
    fn test_binary_tree_none() {
        let mut tree = BinaryTree::new();
        let v = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5), Some(6)];
        tree.insert(&v);
        println!("tree: {:?}", tree);
        tree.print_preorder(0);
        // panic!();
    }

    #[test]
    fn test_binary_tree_depth() {
        let mut tree = BinaryTree::new();
        let v = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5), Some(6)];
        tree.insert(&v);
        tree.print_preorder(0);
        let depth = tree.depth();
        println!("depth: {}", depth);
        assert_eq!(depth, 4);
        // panic!();
    }

    #[test]
    fn test_binary_tree_level_order() {
        let mut tree = BinaryTree::new();
        let v = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5), Some(6)];
        tree.insert(&v);
        let level_order = tree.level_order();
        println!("level order: {:?}", level_order);
        assert_eq!(level_order, vec![vec![1], vec![2, 3], vec![4, 5], vec![6]].to_vec());
        // panic!();
    }

    #[test]
    fn test_binary_tree_right_side_view() {
        let mut tree = BinaryTree::new();
        let v = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5), Some(6)];
        tree.insert(&v);
        let mut res: Vec<i32> = Vec::new();
        tree.right_side_view(0, &mut res);
        println!("right side view: {:?}", res);
        assert_eq!(res, vec![1, 3, 5, 6]);
        // panic!();
    }

    #[test]
    fn test_binary_tree_left_side_view() {
        let mut tree = BinaryTree::new();
        let v = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5), Some(6)];
        tree.insert(&v);
        let mut res: Vec<i32> = Vec::new();
        tree.left_side_view(0, &mut res);
        println!("left side view: {:?}", res);
        assert_eq!(res, vec![1, 2, 4, 6]);
        // panic!();
    }

    #[test]
    fn test_binary_tree_insert_as_complete() {
        let mut tree = BinaryTree::new();
        let v = vec![1, 2, 3, 4, 5, 6, 7];
        tree.insert_as_complete(&v);
        tree.print_preorder(0);
        // panic!();
    }

    #[test]
    fn test_binary_tree_count_nodes() {
        let mut tree = BinaryTree::new();
        let v = vec![1, 2, 3, 4, 5, 6, 7];
        tree.insert_as_complete(&v);
        tree.print_preorder(0);
        let count = tree.count_nodes();
        println!("count: {}", count);
        assert_eq!(count, 7);
        // panic!();
    }
}
