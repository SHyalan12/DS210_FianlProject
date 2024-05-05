mod tests;
mod data_prep;
mod calculate_centrality;

use petgraph::dot::{Config, Dot};
use serde::Deserialize;
use petgraph::graph::UnGraph;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use crate::data_prep::load_and_filter_highways;
use std::env;
use crate::calculate_centrality::{degree_centrality, betweenness_centrality, closeness_centrality};

#[derive(Debug, Deserialize, PartialEq)]
pub struct Highway {
    pub street_name: String,
    pub states: String,
    pub formed: Option<f64>,
    pub removed: Option<f64>,
    pub length_mi: f64,
    pub southern_or_western_terminus: String,
    pub northern_or_eastern_terminus: String,
}

impl Highway {
    pub fn describe(&self) -> String {
        format!(
            "Highway Information:\n\
             Street Name: {}\n\
             States: {}\n\
             Year Formed: {}\n\
             Year Removed: {}\n\
             Length (miles): {:.2}\n\
             Southern/Western Terminus: {}\n\
             Northern/Eastern Terminus: {}",
            self.street_name,
            self.states,
            self.formed.map_or(String::from("N/A"), |year| year.to_string()),
            self.removed.map_or(String::from("N/A"), |year| year.to_string()),
            self.length_mi,
            self.southern_or_western_terminus,
            self.northern_or_eastern_terminus
        )
    }
}


fn build_graph(highways: Vec<Highway>) -> UnGraph<String, ()> {
    let mut graph = UnGraph::new_undirected();
    let mut node_indices = HashMap::new();

    for highway in highways {
        let states_str = highway.states;
        let states_vec: Vec<String> = states_str
            .trim_matches(|c| c == '[' || c == ']' || c == '\\')
            .trim()
            .split(',')
            .map(|s| s.trim().trim_matches('\'').to_string())
            .collect();

        for i in 0..states_vec.len() {
            let state = &states_vec[i];
            if !node_indices.contains_key(state) {
                let node = graph.add_node(state.clone());
                node_indices.insert(state.clone(), node);
            }

            if i > 0 {
                let prev_state = &states_vec[i - 1];
                let from_index = node_indices[prev_state];
                let to_index = node_indices[state];
                graph.add_edge(from_index, to_index, ());
            }
        }
    }

    graph
}

fn main() {
    let current_path = env::current_dir().unwrap();
    println!("Current directory: {:?}", current_path);

    let file_path = "us_highway.csv"; // Adjust this path based on the current directory
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let highways = load_and_filter_highways(reader).expect("Failed to load and filter highways");
    let graph = match build_graph(highways) {
        graph => graph,
    };
    // Calculate centrality measures
    println!("Degree Centrality: {:?}", degree_centrality(&graph));
    println!("Betweenness Centrality: {:?}", betweenness_centrality(&graph));
    println!("Closeness Centrality: {:?}", closeness_centrality(&graph));
    let dot_graph = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    println!("{:?}", dot_graph);
    
}

