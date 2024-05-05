#[cfg(test)]
mod tests {
    use crate::Highway;
    use crate::data_prep::load_and_filter_highways;

    #[test]
    fn test_highway_filtering() {
        let data = "\
street_name,states,formed,removed,length_mi,southern_or_western_terminus,northern_or_eastern_terminus\n\
Highway 1,State1,1980,,100.0,City1,City2\n\
Highway 2,State2,1990,2000,150.0,City3,City4\n\
Highway 3,State3,2000,,200.0,City5,City6";

        let expected = vec![
            Highway {
                street_name: "Highway 1".to_string(),
                states: "State1".to_string(),
                formed: Some(1980.0),
                removed: None,
                length_mi: 100.0,
                southern_or_western_terminus: "City1".to_string(),
                northern_or_eastern_terminus: "City2".to_string(),
            },
            Highway {
                street_name: "Highway 3".to_string(),
                states: "State3".to_string(),
                formed: Some(2000.0),
                removed: None,
                length_mi: 200.0,
                southern_or_western_terminus: "City5".to_string(),
                northern_or_eastern_terminus: "City6".to_string(),
            },
        ];

        let result = load_and_filter_highways(data.as_bytes()).unwrap();
        assert_eq!(result, expected);
    }

    use crate::build_graph;
    use petgraph::algo::is_cyclic_undirected;




    #[test]
    fn test_build_graph() {
        // Mock data
        let highways = vec![
            Highway {
                street_name: "Route 66".to_string(),
                states: "[\"California\", \"Nevada\"]".to_string(),
                formed: Some(1926.0),
                removed:None,
                length_mi: 123.0,
                southern_or_western_terminus: "Los Angeles".to_string(),
                northern_or_eastern_terminus: "Las Vegas".to_string(),
            },
            Highway {
                street_name: "Route 1".to_string(),
                states: "[\"Nevada\", \"Utah\"]".to_string(),
                formed: Some(1926.0),
                removed: None,
                length_mi: 234.0,
                southern_or_western_terminus: "Las Vegas".to_string(),
                northern_or_eastern_terminus: "Salt Lake City".to_string(),
            }
        ];

        let graph_result = build_graph(highways);
        assert!(graph_result.is_ok());

        let graph = graph_result.unwrap();
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
        assert!(!is_cyclic_undirected(&graph));
    }
}
