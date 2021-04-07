use super::{AdjacencyListEntry, NodeId};

/// Container for multiple edges that represent same route
#[derive(Debug, PartialEq, Clone)]
pub struct Route(Vec<AdjacencyListEntry>);

impl Route {
    pub fn new(route_length: usize) -> Self {
        Route(Vec::with_capacity(route_length))
    }

    pub fn add_step(mut self, edge: AdjacencyListEntry) -> Self {
        self.0.push(edge);

        Route(self.0)
    }

    pub fn get_length(&self) -> usize {
        self.0.len()
    }

    pub fn get_distance(&self) -> f32 {
        self.0
            .iter()
            .fold(0.0, |length, edge| length + edge.distance)
    }

    pub fn get_adjusted_distance(&self, target_num_of_steps: usize) -> f32 {
        let route_dist = self.get_distance();
        let route_len = self.get_length();

        route_dist / route_len as f32 * target_num_of_steps as f32
    }

    pub fn get_nodes(&self) -> Vec<NodeId> {
        let head = self.0.iter().take(1).map(|edge| edge.from);
        let tail = self.0.iter().map(|edge| edge.to);

        head.chain(tail).collect()
    }

    pub fn get_edges(&self) -> &[AdjacencyListEntry] {
        &self.0
    }
}

impl Default for Route {
    fn default() -> Self {
        Route(Vec::new())
    }
}
