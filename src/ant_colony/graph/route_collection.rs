use itertools::Itertools;

use super::{Route, RouteBatchWithHoles};

use crate::common::utils::compare_float;

/// Represents multiple unrelated Routes
pub struct RouteCollection(Vec<Route>);

impl RouteCollection {
    pub fn new(ant_count: usize, route_length: usize) -> Self {
        RouteCollection((0..ant_count).map(|_| Route::new(route_length)).collect())
    }

    pub fn add_steps(self, taken_edges: &RouteBatchWithHoles) -> Self {
        let values = taken_edges
            .iter()
            .zip_eq(self.0)
            .map(|(maybe_edge, route)| match maybe_edge {
                Option::Some(edge) => route.add_step(edge.to_owned()),
                Option::None => route,
            })
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

    /// calculates how many routes are shorter than other ones
    /// this is be caused by the fact that `add_steps` accepts RouteBatchWithHoles
    pub fn get_ratio_of_incomplete_routes(&self) -> f32 {
        let lengths = self.0.iter().map(|route| route.get_length());
        let maybe_max_length = lengths.clone().max();

        match maybe_max_length {
            Option::Some(max_length) => {
                let incomplete_routes = lengths.filter(|route_len| *route_len < max_length).count();

                incomplete_routes as f32 / max_length as f32
            }
            Option::None => 1.0,
        }
    }
}

impl Default for RouteCollection {
    fn default() -> Self {
        RouteCollection(Vec::new())
    }
}
