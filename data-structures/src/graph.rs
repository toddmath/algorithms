use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use itertools::Itertools;
use num_traits::PrimInt;

/// Simple Error type for when a node is not in the graph.
#[derive(Debug, Clone, Copy, Default)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub type AdjacencyTable<T> = HashMap<String, Vec<(String, T)>>;

/// Directed Graph Data Structure
#[derive(Debug)]
pub struct DirectedGraph<T = usize>
where
    T: PrimInt,
{
    adjacency_table: AdjacencyTable<T>,
}

/// Undirected Graph Data Structure
#[derive(Debug)]
pub struct UndirectedGraph<T = usize>
where
    T: PrimInt,
{
    adjacency_table: AdjacencyTable<T>,
}

/// Graph Trait
#[allow(missing_docs)]
pub trait Graph<T: PrimInt> {
    fn new() -> Self;
    fn adjacency_table(&self) -> &AdjacencyTable<T>;
    fn adjacency_table_mutable(&mut self) -> &mut AdjacencyTable<T>;

    fn add_node<S: AsRef<str>>(&mut self, node: S) -> bool {
        match self.adjacency_table().get(node.as_ref()) {
            None => {
                self.adjacency_table_mutable()
                    .insert(node.as_ref().to_string(), Vec::new());
                true
            }
            _ => false,
        }
    }

    fn add_edge<S: AsRef<str>>(&mut self, edge: (S, S, T)) {
        let (e0, e1) = (edge.0.as_ref(), edge.1.as_ref());
        self.add_node(e0);
        self.add_node(e1);
        self.adjacency_table_mutable()
            .entry(e0.to_string())
            .and_modify(|e| e.push((e1.to_string(), edge.2)));
    }

    fn neighbors<S: AsRef<str>>(&self, node: S) -> Result<&Vec<(String, T)>, NodeNotInGraph> {
        self.adjacency_table()
            .get(node.as_ref())
            .ok_or(NodeNotInGraph)
    }

    #[inline]
    fn contains<S: AsRef<str>>(&self, node: S) -> bool {
        self.adjacency_table().get(node.as_ref()).is_some()
    }

    #[inline]
    fn nodes<'a>(&'a self) -> HashSet<&'a String>
    where
        T: 'a,
    {
        self.adjacency_table().keys().collect()
    }

    fn edges<'a>(&'a self) -> Vec<(&'a String, &'a String, T)>
    where
        T: 'a,
    {
        // let mut edges = Vec::with_capacity(self.adjacency_table().values().map(|v|
        // v.len()).sum());
        // for (from_node, from_node_neighbors) in self.adjacency_table() {
        //     for (to_node, weight) in from_node_neighbors {
        //         edges.push((from_node, to_node, *weight));
        //     }
        // }
        // edges

        self.adjacency_table()
            .iter()
            .flat_map(|(from_node, from_node_neighbors)| {
                from_node_neighbors
                    .iter()
                    .map(move |(to_node, weight)| (from_node, to_node, *weight))
            })
            .collect_vec()
    }
}

impl<T: PrimInt> Graph<T> for DirectedGraph<T> {
    fn new() -> Self {
        Self {
            adjacency_table: AdjacencyTable::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut AdjacencyTable<T> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &AdjacencyTable<T> {
        &self.adjacency_table
    }
}

impl<T: PrimInt> Graph<T> for UndirectedGraph<T> {
    fn new() -> Self {
        Self {
            adjacency_table: AdjacencyTable::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut AdjacencyTable<T> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &AdjacencyTable<T> {
        &self.adjacency_table
    }

    fn add_edge<S: AsRef<str>>(&mut self, edge: (S, S, T)) {
        let (e0, e1) = (edge.0.as_ref(), edge.1.as_ref());
        self.add_node(e0);
        self.add_node(e1);

        self.adjacency_table
            .entry(e0.to_string())
            .and_modify(|e| e.push((e1.to_string(), edge.2)));

        self.adjacency_table
            .entry(e1.to_string())
            .and_modify(|e| e.push((e0.to_string(), edge.2)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_undirected_graph {
        use super::{Graph, UndirectedGraph};

        #[test]
        fn test_add_edge() {
            let mut graph = UndirectedGraph::new();
            graph.add_edge(("a", "b", 5));
            graph.add_edge(("b", "c", 10));
            graph.add_edge(("c", "a", 7));

            for edge in &[
                (&String::from("a"), &String::from("b"), 5),
                (&String::from("b"), &String::from("a"), 5),
                (&String::from("c"), &String::from("a"), 7),
                (&String::from("a"), &String::from("c"), 7),
                (&String::from("b"), &String::from("c"), 10),
                (&String::from("c"), &String::from("b"), 10),
            ] {
                assert!(graph.edges().contains(edge));
            }
        }

        #[test]
        fn test_neighbours() {
            let mut graph = UndirectedGraph::new();
            graph.add_edge(("a", "b", 5));
            graph.add_edge(("b", "c", 10));
            graph.add_edge(("c", "a", 7));

            assert_eq!(graph.neighbors("a").unwrap(), &vec![
                ("b".into(), 5),
                ("c".into(), 7)
            ]);
        }
    }

    #[cfg(test)]
    mod test_directed_graph {
        use super::{DirectedGraph, Graph};

        #[test]
        fn test_add_node() {
            let mut graph = DirectedGraph::<isize>::new();
            graph.add_node("a");
            graph.add_node("b");
            graph.add_node("c");
            let expected = [&"a".to_string(), &"b".to_string(), &"c".to_string()];
            assert_eq!(graph.nodes(), expected.into_iter().collect());
        }

        #[test]
        fn test_add_edge() {
            let mut graph = DirectedGraph::new();
            graph.add_edge(("a", "b", 5));
            graph.add_edge(("c", "a", 7));
            graph.add_edge(("b", "c", 10));

            for edge in &[
                (&String::from("a"), &String::from("b"), 5),
                (&String::from("c"), &String::from("a"), 7),
                (&String::from("b"), &String::from("c"), 10),
            ] {
                assert!(graph.edges().contains(edge));
            }
        }

        #[test]
        fn test_neighbours() {
            let mut graph = DirectedGraph::new();
            graph.add_edge(("a", "b", 5));
            graph.add_edge(("b", "c", 10));
            graph.add_edge(("c", "a", 7));

            assert_eq!(graph.neighbors("a").unwrap(), &vec![(String::from("b"), 5)]);
        }

        #[test]
        fn test_contains() {
            let mut graph = DirectedGraph::<usize>::new();
            graph.add_node("a");
            graph.add_node("b");
            graph.add_node("c");

            assert!(graph.contains("a"));
            assert!(graph.contains("b"));
            assert!(graph.contains("c"));
            assert!(!graph.contains("d"));
        }
    }
}
