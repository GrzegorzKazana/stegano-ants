use rand::thread_rng;

mod ant_colony;

use ant_colony::colony::{Colony, Config};
use ant_colony::graph::Graph;
use ant_colony::pheromone::BasicPheromoneUpdater;

fn main() {
    let graph = Graph::from_neighbour_tuples(vec![
        (0, 1, 1.0),
        (0, 2, 2.0),
        (0, 3, 10.0),
        (1, 2, 2.0),
        (1, 3, 5.0),
        (2, 3, 6.0),
    ]);

    let config = Config {
        ant_count: 10,
        num_of_steps_per_cycle: 3,
        random: thread_rng(),
        pheromone_updater: BasicPheromoneUpdater::new(1.0, 0.2),
    };

    let colony = Colony::new(config, graph);

    colony.execute_n_cycles(10);
}

// TODO:
// 2. delegate transition picking logic to separate trait/struct
