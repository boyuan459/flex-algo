# Usage
This crate includes the most commonly used data structures and algorithms in Rust.

To use this crate, simply add the following string to your `Cargo.toml`:
```
flex-algo = "0.7.0"
```

Version numbers follow the [semver](https://semver.org/) convention.

Then use the data structure inside your Rust source code as in the following Example.

Remember that, if you need serde support, you should compile using `--features serde`.

# Documentation

Please read the [API documentation here](https://docs.rs/flex-algo/latest/flex_algo/)

## Dijkstra algorithm
<!-- [![crate](https://crates.io/crates/flex-algo)](https://crates.io/crates/flex-algo) -->

This crate implements a Dijkstra algorithm to compute the shortest path by given graph.

This is inspired by the javascript implementation, please reference [here](https://replit.com/@ZhangMYihua/Network-time-delay-Dijkstras-Algorithm-Solution)

### Example

```rust
use flex_algo::Dijkstra;

fn main() {
  let times = vec![
      (1, 2, 9), (1, 4, 2), (2, 5, 1), (4, 2, 4), (4, 5, 6), (3, 2, 3), (5, 3, 7), (3, 1, 5)
  ];
  let dijkstra = Dijkstra::new(5, times);
  let (max, path) =  dijkstra.shortest_path(1).unwrap();
  println!("shortest path: {:?}", path);
  assert_eq!(max, 14);
}
```

## PriorityQueue
<!-- [![crate](https://crates.io/crates/flex-algo)](https://crates.io/crates/flex-algo) -->

This crate implements a Priority Queue with a function to change the priority of an object.
Priorities are stored in a Vec and the queue is implemented as a Heap of indexes.

This is inspired by the javascript implementation, please reference [here](https://replit.com/@ZhangMYihua/priority-queue-class-implementation)

### Example

```rust
use flex_algo::PriorityQueue;

fn main() {
  // priorities
  let distances = [1, 6, 14, 2, 7];
  // queue
  let mut pq = PriorityQueue::new(|a: &usize,b: &usize| distances[*a] < distances[*b]);
  assert_eq!(pq.is_empty(), true);
  pq.push(0);
  pq.push(1);
  pq.push(2);
  pq.push(3);
  pq.push(4);
  let value = pq.pop().unwrap();
  println!("pop priority queue(closure): {:?}", value);
}
```

## Graph
<!-- [![crate](https://crates.io/crates/flex-algo)](https://crates.io/crates/flex-algo) -->

This crate implements a Graph data structure.

This is inspired by the javascript implementation, please reference [BFS](https://replit.com/@ZhangMYihua/Course-schedule-naive-BFS#index.js)
[Topological Sort](https://replit.com/@ZhangMYihua/Course-schedule-Topological-Sort-with-adjacency-list#main.js)
[DFS](https://replit.com/@ZhangMYihua/Adjacency-List-DFS#main.js)
### Example

```rust
use flex_algo::Graph;

fn main() {
    let mut graph = Graph::new(6, vec![(1, 0), (2, 1), (2, 5), (0, 3), (4, 3), (3, 5), (4, 5)]);
    println!("graph: {:?}", graph);
    assert_eq!(graph.is_acyclic_bfs(), true);
    assert_eq!(graph.is_acyclic_top_sort(), true);
}
```

## BinaryTree

This crate implements the BinaryTree data structure.

This is inspired by the javascript and rust implementation below:
[JS](https://replit.com/@ZhangMYihua/Maximum-depth#main.js, https://replit.com/@ZhangMYihua/Level-Order, https://replit.com/@ZhangMYihua/Binary-tree-right-side-view-DFS#index.js, https://replit.com/@ZhangMYihua/Number-of-Nodes-in-Complete-Binary-Tree#index.js)
[Rust](https://github.com/PacktPublishing/Hands-On-Data-Structures-and-Algorithms-in-Rust)

### Crete a binary tree
```rust
use flex_algo::BinaryTree;

fn main() {
    let mut tree = BinaryTree::new();
    let v = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5), Some(6)];
    tree.insert(&v);

    // print in preorder
    tree.print_preorder(0);

    // get the depth of the tree
    let depth = tree.depth();
    println!("depth: {}", depth);
    assert_eq!(depth, 4);

    // get the level order of the tree
    let level_order = tree.level_order();
    println!("level order: {:?}", level_order);
    assert_eq!(level_order, vec![vec![1], vec![2, 3], vec![4, 5], vec![6]].to_vec());

    // get the right side view
    let mut res: Vec<i32> = Vec::new();
    tree.right_side_view(0, &mut res);
    println!("right side view: {:?}", res);
    assert_eq!(res, vec![1, 3, 5, 6]);

    // get the left side view
    let mut res: Vec<i32> = Vec::new();
    tree.left_side_view(0, &mut res);
    println!("left side view: {:?}", res);
    assert_eq!(res, vec![1, 2, 4, 6]);
}
```

## BinarySearchTree(BST)

This crate implements the BinarySearchTree data structure.

This is inspired by the javascript and rust implementation below:
[JS](https://replit.com/@ZhangMYihua/Validate-Binary-Search-Tree#index.js)
[Rust](https://github.com/PacktPublishing/Hands-On-Data-Structures-and-Algorithms-in-Rust/blob/master/code/d3-linklists/v1-ll/src/v3_bintree.rs)

### Crete a binary tree
```rust
use flex_algo::BST;

fn main() {
    let mut bst = BST::new();
    bst.insert(3);
    bst.insert(2);
    bst.insert(1);
    
    let is_valid = bst.is_valid(i32::MIN, i32::MAX);
    assert_eq!(is_valid, true);

    bst.print_preorder(0);

    let none = bst.search(5);
    assert_eq!(none, None);

    let found = bst.search(2);
    assert_eq!(found, Some(2));
}
```