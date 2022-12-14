# PriorityQueue
[![crate](https://crates.io/crates/flex-algo)](https://crates.io/crates/flex-algo)

This crate implements a Priority Queue with a function to change the priority of an object.
Priority and items are stored in an `IndexMap` and the queue is implemented as a Heap of indexes.


Please read the [API documentation here](https://github.com/boyuan459/flex-algo)

## Usage

To use this crate, simply add the following string to your `Cargo.toml`:
```
flex-algo = "0.2.0"
```

Version numbers follow the [semver](https://semver.org/) convention.

Then use the data structure inside your Rust source code as in the following Example.

Remember that, if you need serde support, you should compile using `--features serde`.

## Example

```rust
use flex_algo::PriorityQueue;

fn main() {
  let distances = [1, 6, 14, 2, 7];
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