use std::{collections::HashSet, vec};
use std::fmt::Debug;
use crate::priority_queue::PriorityQueue;

/// Dijkstra algorithm
///
/// This crate implements a Dijkstra algorithm to compute the shortest path by given graph.
/// 
#[derive(Debug)]
pub struct Dijkstra {
    adjacent_list: Vec<Vec<(usize, usize)>>,
    num_nodes: usize,
}

impl Dijkstra {
    /// Create a new Dijkstra graph with edges tuple(current, neighbor, weight) Vec
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::Dijkstra;
    /// 
    /// let times = vec![(0, 1, 9), (0, 3, 2), (1, 4, 1), (3, 1, 4), (3, 4, 6), (2, 1, 3), (4, 2, 7), (2, 0, 5)];
    /// let dijkstra = Dijkstra::new(5, times);
    /// 
    /// ```
    pub fn new(num_nodes: usize, edges: Vec<(usize, usize, usize)>) -> Self {
        let mut adjacent_list = vec![Vec::new(); num_nodes];
        for edge in edges {
            let source = edge.0;
            let target = edge.1;
            adjacent_list[source].push((target, edge.2));
        }
        Dijkstra {
            adjacent_list,
            num_nodes,
        }
    }

    /// Return the shortest path
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::Dijkstra;
    /// 
    /// let times = vec![(0, 1, 9), (0, 3, 2), (1, 4, 1), (3, 1, 4), (3, 4, 6), (2, 1, 3), (4, 2, 7), (2, 0, 5)];
    /// let dijkstra = Dijkstra::new(5, times);
    /// let (max, path) =  dijkstra.shortest_path(0).unwrap();
    /// println!("shortest path: {:?}", path);
    /// assert_eq!(max, 14);
    /// 
    /// ```
    pub fn shortest_path(&self, node: usize) -> Option<(usize, Vec<usize>)> {
        let mut distances = vec![usize::MAX; self.num_nodes];
        distances[node] = 0;
        let distances_ptr = distances.as_mut_ptr();
        let mut heap = PriorityQueue::new(|a: &usize,b:&usize| distances.get(*a).cloned() < distances.get(*b).cloned());
        let mut seens = HashSet::new();
        let mut visit = Vec::new();
        heap.push(node);

        while !heap.is_empty() {
            let vertex = heap.pop().unwrap();
            if !seens.contains(&vertex) {
                visit.push(vertex);
                seens.insert(vertex);
                let adjacent = &self.adjacent_list[vertex];
                for pair in adjacent {
                    let neighbor_vertex = pair.0;
                    let weight = pair.1;
                    if distances[vertex] + weight < distances[neighbor_vertex] {
                        unsafe {
                            *distances_ptr.add(neighbor_vertex) = distances[vertex] + weight;
                        }
                        heap.push(neighbor_vertex);
                    }
                }
            }
        }
        distances.sort();
        let max = distances.pop().unwrap();
        if max < usize::MAX {
            return Some((max, visit));
        }
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_dijkstra() {
        let times = vec![
          (0, 1, 9), (0, 3, 2), (1, 4, 1), (3, 1, 4), (3, 4, 6), (2, 1, 3), (4, 2, 7), (2, 0, 5)
        ];
        let dijkstra = Dijkstra::new(5, times);
        println!("Dijkstra: {:?}", dijkstra);
        // panic!()
    }

    #[test]
    fn test_shortest_path() {
        let times = vec![
          (0, 1, 9), (0, 3, 2), (1, 4, 1), (3, 1, 4), (3, 4, 6), (2, 1, 3), (4, 2, 7), (2, 0, 5)
        ];
        let dijkstra = Dijkstra::new(5, times);
        let (max, path) =  dijkstra.shortest_path(0).unwrap();
        println!("shortest path: {:?}", path);
        assert_eq!(max, 14);
        // panic!();
    }
}
