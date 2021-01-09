use itertools::Itertools;

use super::{Route, RouteBatch};

use crate::common::utils::compare_float;

/// Represents multiple unrelated Routes
pub struct RouteCollection(Vec<Route>);

impl RouteCollection {
    pub fn new(ant_count: usize, route_length: usize) -> Self {
        RouteCollection((0..ant_count).map(|_| Route::new(route_length)).collect())
    }

    pub fn add_steps(self, taken_edges: &RouteBatch) -> Self {
        let values = taken_edges
            .iter()
            .zip_eq(self.0)
            .map(|(edge, route)| route.add_step(edge.to_owned()))
            .collect();

        RouteCollection(values)
    }

    pub fn get_shortest_route(&self) -> Option<Route> {
        self.0
            .iter()
            .min_by(|a, b| {
                let dist_a = a.get_distance();
                let dist_b = b.get_distance();

                compare_float(&dist_a, &dist_b)
            })
            .map(|route| route.clone())
    }

    pub fn get_shortest_route_distance(&self) -> Option<f32> {
        self.get_shortest_route().map(|route| route.get_distance())
    }

    pub fn get_routes(&self) -> &[Route] {
        &self.0
    }

    pub fn get_average_route_distance(&self) -> f32 {
        let sum = self
            .0
            .iter()
            .fold(0.0, |acc, route| acc + route.get_distance());

        sum / self.0.len() as f32
    }
}

impl Default for RouteCollection {
    fn default() -> Self {
        RouteCollection(Vec::new())
    }
}
