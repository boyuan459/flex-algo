//! PriorityQueue
//!
//! This data structure implements a Priority Queue with a comparator function to specify the Min/Max heap.
//! The queue is implemented as a heap of indexes.
///
/// # Example
///
/// ```
/// use flex_algo::priority_queue::PriorityQueue;
///
/// let mut pq = PriorityQueue::new(|a: &usize,b: &usize| a < b);
/// pq.push(0);
/// pq.push(1);
/// pq.push(2);
///
/// let value = pq.pop().unwrap();
/// assert_eq!(value, 0);
/// ```
///
use std::fmt::Debug;

/// PriorityQueue
///
/// This data structure implements a Priority Queue with a comparator function to specify the Min/Max heap.
/// The queue is implemented as a heap of indexes.
///
#[derive(Debug)]
pub struct PriorityQueue<F, T>
where
    F: Fn(&T, &T) -> bool,
    T: PartialOrd + Debug,
{
    heap: Vec<T>,
    comparator: F,
}

impl<F, T> PriorityQueue<F, T>
where
    F: Fn(&T, &T) -> bool,
    T: PartialOrd + Debug,
{
    /// Create a new PriorityQueue with a comparator function
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::priority_queue::PriorityQueue;
    /// 
    /// let mut pq = PriorityQueue::new(|a: &usize,b: &usize| a < b);
    /// 
    /// ```
    pub fn new(comparator: F) -> Self {
        PriorityQueue {
            heap: Vec::new(),
            comparator,
        }
    }

    /// Return the size of the PriorityQueue
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::priority_queue::PriorityQueue;
    /// 
    /// let mut pq = PriorityQueue::new(|a: &usize,b: &usize| a < b);
    /// assert_eq!(pq.size(), 0);
    /// ```
    pub fn size(&self) -> usize {
        self.heap.len()
    }

    /// Return true if the PriorityQueue is empty
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::priority_queue::PriorityQueue;
    /// 
    /// let mut pq = PriorityQueue::new(|a: &usize,b: &usize| a < b);
    /// assert_eq!(pq.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.heap.len() == 0
    }

    fn _parent(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn _left_child(&self, idx: usize) -> usize {
        2 * idx + 1
    }

    fn _right_child(&self, idx: usize) -> usize {
        2 * idx + 2
    }

    fn _compare(&self, i: usize, j: usize) -> bool {
        (self.comparator)(self.heap.get(i).unwrap(), self.heap.get(j).unwrap())
    }

    fn _swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
    }

    fn _sift_up(&mut self) {
        let mut node_index = self.size() - 1;
        while node_index > 0 && self._compare(node_index, self._parent(node_index)) {
            self._swap(node_index, self._parent(node_index));
            node_index = self._parent(node_index);
        }
    }

    /// Push element into Priority Queue and return the size of the PriorityQueue 
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::priority_queue::PriorityQueue;
    /// 
    /// let mut pq = PriorityQueue::new(|a: &usize,b: &usize| a < b);
    /// pq.push(14);
    /// pq.push(10);
    /// let len = pq.push(12);
    /// 
    /// assert_eq!(len, 3);
    /// ```
    pub fn push(&mut self, value: T) -> usize {
        self.heap.push(value);
        self._sift_up();
        self.heap.len()
    }

    fn _sift_down(&mut self) {
        let mut node_index = 0;
        while (self._left_child(node_index) < self.size()
            && self._compare(self._left_child(node_index), node_index))
            || (self._right_child(node_index) < self.size()
                && self._compare(self._right_child(node_index), node_index))
        {
            let mut greater_index = self._left_child(node_index);
            if self._right_child(node_index) < self.size()
                && self._compare(self._right_child(node_index), self._left_child(node_index))
            {
                greater_index = self._right_child(node_index);
            }
            self._swap(node_index, greater_index);
            node_index = greater_index;
        }
    }

    /// Return the first element of the heap, or `None` if it is empty.
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::priority_queue::PriorityQueue;
    /// 
    /// let mut pq = PriorityQueue::new(|a: &usize,b: &usize| a < b);
    /// pq.push(14);
    /// pq.push(10);
    /// pq.push(12);
    /// 
    /// assert_eq!(pq.pop().unwrap(), 10);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.size() > 1 {
            self._swap(0, self.size() - 1);
        }
        let value = self.heap.pop();
        self._sift_down();
        value
    }

    /// Return the first element of the heap, or `None` if it is empty without change the heap.
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::priority_queue::PriorityQueue;
    /// 
    /// let mut pq = PriorityQueue::new(|a: &usize,b: &usize| a < b);
    /// pq.push(14);
    /// pq.push(10);
    /// pq.push(12);
    /// 
    /// assert_eq!(pq.peek().unwrap(), &10);
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.heap.first()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare(a: &usize, b: &usize) -> bool {
        a > b
    }

    #[test]
    fn test_priority_queue_function() {
        let mut pq = PriorityQueue::new(compare);
        assert_eq!(pq.size(), 0);
        assert_eq!(pq._parent(1), 0);
        assert_eq!(pq._parent(2), 0);
        assert_eq!(pq._parent(3), 1);
        assert_eq!(pq._left_child(1), 3);
        assert_eq!(pq._right_child(1), 4);

        pq.push(14);
        pq.push(12);
        pq.push(5);
        pq.push(7);
        pq.push(8);
        pq.push(3);

        println!("priority queue: {:?}", pq.heap);
        assert_eq!(pq.heap.get(0).unwrap(), &14);
        assert_eq!(pq.peek().unwrap(), &14);
        assert_eq!(pq.pop().unwrap(), 14);
        // panic!();
    }

    #[test]
    fn test_priority_queue_closure() {
        let distances = [1, 6, 14, 2, 7];
        let mut pq = PriorityQueue::new(|a: &usize, b: &usize| distances[*a] < distances[*b]);
        assert_eq!(pq.is_empty(), true);
        pq.push(0);
        pq.push(1);
        pq.push(2);
        pq.push(3);
        pq.push(4);
        println!("priority queue(closure): {:?}", pq.heap);
        let value = pq.pop().unwrap();
        println!("priority queue(closure): {:?}", pq.heap);
        assert_eq!(value, 0);
        // panic!();
    }
}
