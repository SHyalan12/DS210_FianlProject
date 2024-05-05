// Struct Highway
// read and filter highway
// 


mod tests;
mod data_prep;

use serde::Deserialize;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::{Graph, Undirected};
use std::collections::HashMap;
use serde_json::Value;
use serde_json::Error as SerdeError;
use csv::Reader;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use crate::data_prep::load_and_filter_highways;
use std::env;


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


pub fn build_graph(highways: Vec<Highway>) -> UnGraph<String, ()> {
    let mut graph = UnGraph::<String, ()>::new_undirected();
    let mut node_indices = HashMap::new();

    for highway in highways {
        let mut prev_state: Option<String> = None;
        let states: Vec<String> = serde_json::from_str(&highway.states)
            .unwrap_or_else(|_| {
                eprintln!("Invalid JSON string for states: {}", &highway.states);
                Vec::new()
            });

        for state in states {
            if let Some(prev) = prev_state.take() {
                if !node_indices.contains_key(&prev) {
                    let node = graph.add_node(prev);
                    node_indices.insert(prev, node);
                }
                if !node_indices.contains_key(&state) {
                    let node = graph.add_node(state.clone());
                    node_indices.insert(state.clone(), node);
                }

                let from_index = node_indices[&prev];
                let to_index = node_indices[&state];
                graph.add_edge(from_index, to_index, ());
            }
            prev_state = Some(state);
        }
    }

    graph
}

pub fn count_vertices(graph: &Graph<String, (), Undirected>) -> usize {
    graph.node_count()
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
        // Handle the case where the graph construction fails
    };

    let vertex_count = count_vertices(&graph);
    println!("The graph has {} vertices.", vertex_count);
}