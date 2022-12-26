use std::fmt::Debug;
use std::collections::HashSet;

/// Graph data structure
/// 
/// This create implements a Graph data structure
/// 
#[derive(Debug)]
pub struct Graph {
    adjacent_list: Vec<Vec<usize>>,
    indegree: Vec<usize>,
    num_nodes: usize,
}

impl Graph {

    /// Create a new graph with the given routes tuple(current, neighbor) current <--- neighbor
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::Graph;
    /// 
    /// let graph = Graph::new(6, vec![(1, 0), (2, 1), (2, 5), (0, 3), (4, 3), (3, 5), (4, 5)]);
    /// println!("graph: {:?}", graph);
    /// ```
    pub fn new(num_nodes: usize, routes: Vec<(usize, usize)>) -> Self {
        let mut adjacent_list = vec![Vec::<usize>::new(); num_nodes];
        let mut indegree = vec![0; num_nodes];
        for route in routes {
          let source = route.1;
          let target = route.0;
          adjacent_list[source].push(target);
          indegree[target] += 1;
        }
        Graph {
          adjacent_list,
          indegree,
          num_nodes,
        }
    }

    /// Breadth First Search algorithm to check if it's an acyclic graph,
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::Graph;
    /// 
    /// let graph = Graph::new(6, vec![(1, 0), (2, 1), (2, 5), (0, 3), (4, 3), (3, 5), (4, 5)]);
    /// println!("graph: {:?}", graph);
    /// assert_eq!(graph.is_acyclic_bfs(), true);
    /// 
    /// ```
    pub fn is_acyclic_bfs(&self) -> bool {
        for v in 0..self.num_nodes {
            // println!("start: {}", v);
            let mut queue = Vec::new();
            let mut seens = HashSet::new();
            let mut visit = Vec::new();
            visit.push(v);

            let adjacent = &self.adjacent_list[v];
            for &neighbor in adjacent {
                queue.push(neighbor);
            }

            while queue.len() > 0 {
                let vertex = queue.pop().unwrap();
                if vertex == v {
                  return false;
                }

                if !seens.contains(&vertex) {
                    // println!("traverse: {}", vertex);
                    visit.push(vertex);
                    seens.insert(vertex);
                }

                let adjacent = &self.adjacent_list[vertex];
                for neighbor in adjacent {
                    if !seens.contains(neighbor) {
                        queue.push(*neighbor);
                    }
                }
            }
            // println!("path: {:?}", visit);
        }
        true
    }

    /// Topological Sort algorithm to check if it's an acyclic graph
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::Graph;
    /// 
    /// let mut graph = Graph::new(6, vec![(1, 0), (2, 1), (2, 5), (0, 3), (4, 3), (3, 5), (4, 5)]);
    /// println!("graph: {:?}", graph);
    /// assert_eq!(graph.is_acyclic_top_sort(), true);
    /// 
    /// ```
    pub fn is_acyclic_top_sort(&mut self) -> bool {
        let mut queue = Vec::new();
        for i in 0..self.num_nodes {
            if self.indegree[i] == 0 {
                queue.push(i);
            }
        }
        let mut count = 0;
        while !queue.is_empty() {
            let vertex = queue.pop().unwrap();
            count += 1;
            let adjacent = &self.adjacent_list[vertex];
            for &neighbor in adjacent {
                self.indegree[neighbor] -= 1;
                if self.indegree[neighbor] == 0 {
                    queue.push(neighbor);
                }
            }
        }
        self.num_nodes == count
    }

    /// Return the path by breadth first search algo for the graph
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::Graph;
    /// 
    /// let mut graph = Graph::new(6, vec![(1, 0), (2, 1), (2, 5), (0, 3), (4, 3), (3, 5), (4, 5)]);
    /// let visit = graph.breadth_first_search(5);
    /// assert_eq!(visit, vec![5, 4, 3, 0, 1, 2]);
    /// 
    /// ```
    pub fn breadth_first_search(&self, start: usize) -> Vec<usize> {
        let mut queue = Vec::new();
        let mut visit = Vec::new();
        let mut seens = HashSet::new();
        queue.push(start);

        while !queue.is_empty() {
            // println!("queue: {:?}", queue);
            let vertex = queue.pop().unwrap();
            if seens.contains(&vertex) {
                continue;
            }
            seens.insert(vertex);
            visit.push(vertex);
            let adjacent = &self.adjacent_list[vertex];
            for neighbor in adjacent {
                if !seens.contains(neighbor) {
                    queue.push(*neighbor);
                }
            }
        }
        visit
    }

    /// Return the path by depth first search algo for the graph
    /// 
    /// # Example
    /// 
    /// ```
    /// use flex_algo::Graph;
    /// 
    /// let graph = Graph::new(8, vec![
    ///     (5, 4), (2, 4), (6, 4), 
    ///     (7, 5), (0, 2), (1, 2), (3, 6)
    /// ]);
    /// let visit = graph.depth_first_search(4);
    /// 
    /// assert_eq!(visit, vec![7, 5, 0, 1, 2, 3, 6, 4]);
    /// ```
    pub fn depth_first_search(&self, vertex: usize) -> Vec<usize> {
        let adjacent = &self.adjacent_list[vertex];
        if adjacent.is_empty() {
            return vec![vertex];
        }
        let mut visit = Vec::new();
        for &neighbor in adjacent {
            visit.append(&mut self.depth_first_search(neighbor));
        }
        visit.push(vertex);
        visit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        let graph = Graph::new(6, vec![(1, 0), (2, 1), (2, 5), (0, 3), (4, 3), (3, 5), (4, 5)]);
        println!("graph: {:?}", graph);
        // panic!();
    }

    #[test]
    fn test_is_acyclic_graph() {
        let graph = Graph::new(6, vec![(1, 0), (2, 1), (2, 5), (0, 3), (4, 3), (3, 5), (4, 5)]);
        println!("graph: {:?}", graph);
        assert_eq!(graph.is_acyclic_bfs(), true);
        // panic!();
    }

    #[test]
    fn test_is_acyclic_top_sort() {
        let mut graph = Graph::new(6, vec![(1, 0), (2, 1), (2, 5), (0, 3), (4, 3), (3, 5), (4, 5)]);
        println!("graph: {:?}", graph);
        assert_eq!(graph.is_acyclic_top_sort(), true);
    }

    #[test]
    fn test_bfs() {
        let graph = Graph::new(6, vec![(1, 0), (2, 1), (2, 5), (0, 3), (4, 3), (3, 5), (4, 5)]);
        let visit = graph.breadth_first_search(5);
        println!("graph: {:?}", graph);
        println!("bfs: {:?}", visit);
        assert_eq!(visit, vec![5, 4, 3, 0, 1, 2]);
    }

    #[test]
    fn test_dfs() {
        let graph = Graph::new(8, vec![
            (5, 4), (2, 4), (6, 4), 
            (7, 5), (0, 2), (1, 2), (3, 6)
        ]);
        let visit = graph.depth_first_search(4);
        println!("graph: {:?}", graph);
        println!("dfs: {:?}", visit);
        assert_eq!(visit, vec![7, 5, 0, 1, 2, 3, 6, 4]);
    }
}
