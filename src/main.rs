use rand::thread_rng;

mod ant_colony;

use ant_colony::colony::{Colony, Config};
use ant_colony::graph::{AdjacencyListEntry, Graph, Node};

fn main() {
    let graph = Graph::from_vector(vec![
        Node {
            id: 0,
            adjacency_list: vec![
                AdjacencyListEntry {
                    from: 0,
                    to: 1,
                    distance: 1.0,
                },
                AdjacencyListEntry {
                    from: 0,
                    to: 2,
                    distance: 2.0,
                },
                AdjacencyListEntry {
                    from: 0,
                    to: 3,
                    distance: 10.0,
                },
            ],
        },
        Node {
            id: 1,
            adjacency_list: vec![
                AdjacencyListEntry {
                    from: 1,
                    to: 0,
                    distance: 1.0,
                },
                AdjacencyListEntry {
                    from: 1,
                    to: 2,
                    distance: 2.0,
                },
                AdjacencyListEntry {
                    from: 1,
                    to: 3,
                    distance: 5.0,
                },
            ],
        },
        Node {
            id: 2,
            adjacency_list: vec![
                AdjacencyListEntry {
                    from: 2,
                    to: 0,
                    distance: 2.0,
                },
                AdjacencyListEntry {
                    from: 2,
                    to: 1,
                    distance: 2.0,
                },
                AdjacencyListEntry {
                    from: 2,
                    to: 3,
                    distance: 6.0,
                },
            ],
        },
        Node {
            id: 3,
            adjacency_list: vec![
                AdjacencyListEntry {
                    from: 3,
                    to: 0,
                    distance: 10.0,
                },
                AdjacencyListEntry {
                    from: 3,
                    to: 1,
                    distance: 5.0,
                },
                AdjacencyListEntry {
                    from: 3,
                    to: 2,
                    distance: 6.0,
                },
            ],
        },
    ]);

    let config = Config {
        ant_count: 10,
        num_of_steps_per_cycle: 3,
        random: thread_rng(),
    };

    let colony = Colony::new(config, graph);

    colony.execute_n_cycles(10);
}
