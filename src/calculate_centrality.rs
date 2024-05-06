extern crate petgraph;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::IntoNodeIdentifiers;
use petgraph::algo::all_simple_paths;
use std::collections::HashMap;

pub fn degree_centrality(graph: &UnGraph<String, ()>) -> HashMap<String, usize> {
    graph
        .node_indices()
        .map(|node| {
            let state = &graph[node];
            (state.clone(), graph.neighbors(node).count())
        })
        .collect()
}
pub fn betweenness_centrality(graph: &UnGraph<String, ()>) -> HashMap<NodeIndex, f64> {
    let mut centrality: HashMap<NodeIndex, f64> = HashMap::new();
    let node_indices: Vec<NodeIndex> = graph.node_indices().collect();
    let node_count = node_indices.len() as f64;


    for node in &node_indices {
        centrality.insert(*node, 0.0);
    }

    for start in &node_indices {
        for end in &node_indices {
            if start != end {
                let all_paths = all_simple_paths::<Vec<_>, _>(graph, *start, *end, 0, None)
                    .collect::<Vec<_>>();
                let total_paths = all_paths.len() as f64;
                if total_paths > 0.0 {
                    for path in &all_paths {
                        for &node in path.iter().filter(|&&n| n != *start && n != *end) {
                            *centrality.get_mut(&node).unwrap() += 1.0 / total_paths;
                        }
                    }
                }
            }
        }
    }

    // Normalize the centrality scores
    for value in centrality.values_mut() {
        *value /= (node_count - 1.0) * (node_count - 2.0) / 2.0;
    }

    centrality
}

pub fn closeness_centrality(graph: &UnGraph<String, ()>) -> HashMap<String, f64> {
    let mut centrality = HashMap::new();
    let node_ids: Vec<NodeIndex> = graph.node_identifiers().collect();

    for &node in &node_ids {
        let path_lengths = dijkstra(graph, node, None, |_| 1);
        let total_distance: usize = path_lengths.values().sum();

        let state = &graph[node];
        centrality.insert(
            state.clone(),
            if total_distance > 0 {
                1.0 / total_distance as f64
            } else {
                0.0
            },
        );
    }

    centrality
}
