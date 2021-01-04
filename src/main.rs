use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "profiler")] {
        extern crate flame;
        #[macro_use]
        extern crate flamer;
        use flame as f;
        use std::fs::File;
    }
}

use rand::{distributions::Uniform, prelude::StdRng, Rng, SeedableRng};
use std::time::Instant;

mod ant_colony;

use ant_colony::ant::BasicAntDispatcher;
use ant_colony::colony::{Colony, Config};
use ant_colony::graph::Graph;
use ant_colony::pheromone::BasicPheromoneUpdater;

fn random_tsp_graph(nodes: u32) -> Graph {
    let rng = StdRng::seed_from_u64(42);

    let distances = rng.sample_iter(Uniform::from(0.1..9.9));

    let tuples = (0..nodes - 1)
        .flat_map(|from| (from + 1..nodes).map(move |to| (from, to)))
        .zip(distances)
        .map(|((from, to), distance)| (from, to, distance))
        .collect();

    Graph::from_neighbour_tuples(tuples)
}

fn main() {
    let rng = StdRng::seed_from_u64(42);

    let graph = random_tsp_graph(100);

    // let graph = Graph::from_neighbour_tuples(vec![
    //     (0, 1, 1.0),
    //     (0, 2, 2.0),
    //     (0, 3, 10.0),
    //     (1, 2, 2.0),
    //     (1, 3, 5.0),
    //     (2, 3, 6.0),
    // ]);

    let config = Config {
        ant_count: 10,
        num_of_steps_per_cycle: graph.get_amount_of_nodes() - 1,
        pheromone_updater: BasicPheromoneUpdater::new(1.0, 0.1),
        ant_dispatcher: BasicAntDispatcher::new(rng),
    };

    let start = Instant::now();
    let colony = Colony::new(config, graph).execute_n_cycles(1);
    let duration = start.elapsed().as_millis();

    println!("{}", colony);
    println!("execution time: {:>8}ms", duration);

    cfg_if! {
        if #[cfg(feature = "profiler")] {
            let latest_file_name_html = ".profiles/_latest.html";
            let latest_file_name_json = ".profiles/_latest.json";
            f::dump_html(File::create(latest_file_name_html).unwrap()).unwrap();
            f::dump_json(&mut File::create(latest_file_name_json).unwrap()).unwrap();
        }
    }
}
