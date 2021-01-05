use super::{AdjacencyListEntry, NodeId};

/// Container for multiple edges that represent same route
#[derive(Debug, PartialEq)]
pub struct Route<'a>(Vec<&'a AdjacencyListEntry>);

impl<'a> Route<'a> {
    pub fn new(route_length: usize) -> Self {
        Route(Vec::with_capacity(route_length))
    }

    pub fn add_step(mut self, edge: &'a AdjacencyListEntry) -> Self {
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

    pub fn get_nodes(&self) -> Vec<NodeId> {
        let path_length = self.0.len() + 1;

        self.0
            .iter()
            .fold(Vec::with_capacity(path_length), |mut acc, edge| {
                if acc.len() == 0 {
                    acc.push(edge.from);
                }

                acc.push(edge.to);
                acc
            })
    }
}

impl<'a> Default for Route<'a> {
    fn default() -> Self {
        Route(Vec::new())
    }
}
