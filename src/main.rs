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