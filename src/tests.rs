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

    #[test]
    fn test_build_graph_empty_highways() {
        let highways: Vec<Highway> = vec![];
        let graph = build_graph(highways);
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }


    #[test]
    fn test_build_graph_multiple_highways() {
        let highway1 = Highway {
            street_name: String::from("I-5"),
            states: "[\"CA\", \"OR\", \"WA\"]".to_string(),
            formed: None,
            removed: None,
            length_mi: 1381.0,
            southern_or_western_terminus: String::from("San Diego"),
            northern_or_eastern_terminus: String::from("Seattle"),
        };
        let highway2 = Highway {
            street_name: String::from("I-10"),
            states: "[\"CA\", \"AZ\", \"NM\", \"TX\"]".to_string(),
            formed: None,
            removed: None,
            length_mi: 2000.0,
            southern_or_western_terminus: String::from("Santa Monica"),
            northern_or_eastern_terminus: String::from("Jacksonville"),
        };
        let highways = vec![highway1, highway2];
        let graph = build_graph(highways);
        assert_eq!(graph.node_count(), 6);
        assert_eq!(graph.edge_count(), 5);
    }
}