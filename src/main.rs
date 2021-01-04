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

use rand::{prelude::StdRng, SeedableRng};

mod ant_colony;
mod common;

use ant_colony::ant_dispatcher::BasicAntDispatcher;
use ant_colony::colony::{Colony, Config};
use ant_colony::graph::Graph;
use ant_colony::pheromone_updater::ConstantPheromoneUpdater;

use common::utils::measure;

fn main() {
    let rng = StdRng::seed_from_u64(42);

    let graph = Graph::from_neighbour_tuples(vec![
        (0, 1, 1.0),
        (0, 2, 2.0),
        (0, 3, 10.0),
        (1, 2, 2.0),
        (1, 3, 5.0),
        (2, 3, 6.0),
    ]);

    let config = Config {
        ant_count: graph.get_amount_of_nodes(),
        num_of_steps_per_cycle: graph.get_amount_of_nodes(),
        pheromone_updater: ConstantPheromoneUpdater::new(1.0, 0.1),
        ant_dispatcher: BasicAntDispatcher::new(rng),
    };

    let (colony, duration_ms) = measure(|| Colony::new(config, graph).execute_n_cycles(20));

    println!("{}", colony);
    println!("execution time: {:>8}ms", duration_ms);

    cfg_if! {
        if #[cfg(feature = "profiler")] {
            let latest_file_name_html = ".profiles/_latest.html";
            let latest_file_name_json = ".profiles/_latest.json";
            f::dump_html(File::create(latest_file_name_html).unwrap()).unwrap();
            f::dump_json(&mut File::create(latest_file_name_json).unwrap()).unwrap();
        }
    }
}
