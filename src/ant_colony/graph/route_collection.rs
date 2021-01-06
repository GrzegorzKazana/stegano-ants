use super::{AdjacencyListEntry, Route};

/// Represents multiple unrelated Routes
pub struct RouteCollection<'a>(Vec<Route<'a>>);

impl<'a> RouteCollection<'a> {
    pub fn new(ant_count: usize, route_length: usize) -> Self {
        RouteCollection((0..ant_count).map(|_| Route::new(route_length)).collect())
    }

    pub fn add_steps(self, taken_edges: &[&'a AdjacencyListEntry]) -> Self {
        let values = taken_edges
            .iter()
            .zip(self.0)
            .map(|(edge, route)| route.add_step(edge))
            .collect();

        RouteCollection(values)
    }

    pub fn get_shortest_route(&self) -> Option<&Route> {
        self.0
            .iter()
            .min_by(|a, b| a.get_distance().partial_cmp(&b.get_distance()).unwrap())
    }

    pub fn get_shortest_route_distance(&self) -> Option<f32> {
        self.get_shortest_route().map(|route| route.get_distance())
    }

    pub fn get_routes(&self) -> &[Route] {
        &self.0
    }

    pub fn get_average_route_distance(&self) -> f32 {
        self.0
            .iter()
            .fold(0.0, |acc, route| acc + route.get_distance())
    }
}

impl<'a> Default for RouteCollection<'a> {
    fn default() -> Self {
        RouteCollection(Vec::new())
    }
}
