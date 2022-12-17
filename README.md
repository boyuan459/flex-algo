# Usage

To use this crate, simply add the following string to your `Cargo.toml`:
```
flex-algo = "0.4.0"
```

Version numbers follow the [semver](https://semver.org/) convention.

Then use the data structure inside your Rust source code as in the following Example.

Remember that, if you need serde support, you should compile using `--features serde`.

## Dijkstra algorithm
<!-- [![crate](https://crates.io/crates/flex-algo)](https://crates.io/crates/flex-algo) -->

This crate implements a Dijkstra algorithm to compute the shortest path by given graph.

Please read the [API documentation here](https://docs.rs/flex-algo/latest/flex_algo/)

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


Please read the [API documentation here](https://docs.rs/flex-algo/latest/flex_algo/)

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


Please read the [API documentation here](https://docs.rs/flex-algo/latest/flex_algo/)

This is inspired by the javascript implementation, please reference [BFS](https://replit.com/@ZhangMYihua/Course-schedule-naive-BFS#index.js)
[Topological Sort](https://replit.com/@ZhangMYihua/Course-schedule-Topological-Sort-with-adjacency-list#main.js)
[DFS](https://replit.com/@ZhangMYihua/Adjacency-List-DFS#main.js)
### Example

```rust
use use flex_algo::Graph;

fn main() {
    let mut graph = Graph::new(6, vec![(1, 0), (2, 1), (2, 5), (0, 3), (4, 3), (3, 5), (4, 5)]);
    println!("graph: {:?}", graph);
    assert_eq!(graph.is_acyclic_bfs(), true);
    assert_eq!(graph.is_acyclic_top_sort(), true);
}
```
