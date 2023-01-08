//! [PriorityQueue]
//!
//! This data structure implements a Priority Queue with a comparator function to specify the Min/Max heap.
//! The queue is implemented as a heap of indexes.
//!
//! [Dijkstra]
//!
//! This algorithm implements a Dijkstra algorithm to compute the shortest path by given graph.
//!
//! [Graph]
//!
//! This data structure implements Graph algorithm with acyclic, bfs, dfs.
//!
//! [BinaryTree]
//! 
//! This data structure implements BinaryTree with depth, level order, left/right side view, complete tree and count nodes.
//! 
//! [BST]
//! 
//! This data structure implements BinarySearchTree with insert, validate, search and traversal preporder.
//! 
//! [LinkedList]
//! 
//! This data structure implements LinkedList with push_back, push_front, pop_back, pop_front and reverse
//! 
pub use self::priority_queue::PriorityQueue;
pub use self::dijkstra::Dijkstra;
pub use self::graph::Graph;
pub use self::binary_tree::BinaryTree;
pub use self::binary_search_tree::BST;
pub use self::linked_list::LinkedList;
pub use self::doubly_linked_list::DoublyLinkedList;

pub mod priority_queue;
pub mod dijkstra;
pub mod graph;
pub mod binary_tree;
pub mod binary_search_tree;
pub mod linked_list;
pub mod doubly_linked_list;
