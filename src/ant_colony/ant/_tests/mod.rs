#[cfg(test)]
mod graph_tests {
    use crate::ant_colony::ant::Ant;

    #[test]
    fn it_initialized_the_ant_with_visited_node_id() {
        let ant = Ant::new(42);

        assert!(ant.current_node == 42);
        assert!(!ant.has_visited(&42));
        assert!(!ant.has_visited(&43));
    }

    #[test]
    fn it_transitions_ant_to_next_node() {
        let ant = Ant::new(42).move_to_node(43);

        assert_eq!(ant.current_node, 43);
        assert!(ant.has_visited(&43));
    }
}
