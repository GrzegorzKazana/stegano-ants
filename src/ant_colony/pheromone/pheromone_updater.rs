use std::fmt::Display;

use crate::ant_colony::ant::Ant;
use crate::ant_colony::graph::{AdjacencyListEntry, Graph};

use super::pheromone::Pheromone;

pub trait PheromoneUpdater: Display {
    fn initialize(&self, pheromone: Pheromone, edges: Vec<&AdjacencyListEntry>) -> Pheromone;

    fn on_after_step(
        &self,
        pheromone: Pheromone,
        taken_edges: Vec<&AdjacencyListEntry>,
    ) -> Pheromone;

    fn on_after_cycle(&self, pheromone: Pheromone, ants: &Vec<Ant>, graph: &Graph) -> Pheromone;
}

pub struct BasicPheromoneUpdater {
    initial_value: f32,
    evaporation_rate: f32,
}

impl BasicPheromoneUpdater {
    pub fn new(initial_value: f32, evaporation_rate: f32) -> Self {
        BasicPheromoneUpdater {
            initial_value,
            evaporation_rate,
        }
    }
}

impl Display for BasicPheromoneUpdater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pheromone updater (Basic)\n\t\
            initial_value: {:>5}\n\t\
            evaporation:   {:>5.3}",
            self.initial_value, self.evaporation_rate
        )
    }
}

impl PheromoneUpdater for BasicPheromoneUpdater {
    fn initialize(&self, init_pheromone: Pheromone, edges: Vec<&AdjacencyListEntry>) -> Pheromone {
        edges.iter().fold(init_pheromone, |pheromone, edge| {
            pheromone.initialize_pheromone_for_edge(edge.from, edge.to, self.initial_value)
        })
    }

    #[cfg_attr(feature = "profiler", flame)]
    fn on_after_step(
        &self,
        pheromone: Pheromone,
        taken_edges: Vec<&AdjacencyListEntry>,
    ) -> Pheromone {
        let n_steps = taken_edges.len() as i32;

        let decay = 1.0 - self.evaporation_rate;
        let increment = self.evaporation_rate * self.initial_value;

        let accumulated_decay = decay.powi(n_steps);
        let decayed_pheromone = pheromone.scale_all_pheromone_values(accumulated_decay);

        taken_edges
            .iter()
            .fold(decayed_pheromone, |updated_pheromone, taken_edge| {
                updated_pheromone.increase_pheromone_value(
                    taken_edge.from,
                    taken_edge.to,
                    increment,
                )
            })
    }

    fn on_after_cycle(&self, pheromone: Pheromone, _ants: &Vec<Ant>, _graph: &Graph) -> Pheromone {
        pheromone
    }
}
