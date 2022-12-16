//! [PriorityQueue]
//!
//! This data structure implements a Priority Queue with a comparator function to specify the Min/Max heap.
//! The queue is implemented as a heap of indexes.
//!
//! [Dijkstra]
//!
//! This algorithm implements a Dijkstra algorithm to compute the shortest path by given graph.
//!

pub use self::priority_queue::PriorityQueue;
pub use self::dijkstra::Dijkstra;

pub mod priority_queue;
pub mod dijkstra;
