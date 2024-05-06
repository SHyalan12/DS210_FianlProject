mod tests;
mod data_prep;
mod calculate_centrality;

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

    let file_path = "us_highway.csv";
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let highways = load_and_filter_highways(reader).expect("Failed to load and filter highways");
    let graph = match build_graph(highways) {
        graph => graph,
    };
    
    let degree_centrality_scores = degree_centrality(&graph);
    println!("Degree Centrality:");
    for (state, score) in &degree_centrality_scores {
        println!("{}: {}", state, score);
    }

    let closeness_centrality_scores = closeness_centrality(&graph);
    println!("Closeness Centrality:");
    for (state, score) in &closeness_centrality_scores {
        println!("{}: {}", state, score);
    }
    let mut degree_vec: Vec<_> = degree_centrality_scores.iter().collect();
    degree_vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("Top three states by degree centrality:");
    for (state, score) in degree_vec.iter().take(3) {
        println!("{}: {}", state, score);
    }


    let mut closeness_vec: Vec<_> = closeness_centrality_scores.iter().collect();
    closeness_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    println!("Top three states by closeness centrality:");
    for (state, score) in closeness_vec.iter().take(3) {
        println!("{}: {}", state, score);
    }

}
