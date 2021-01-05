use super::AdjacencyListEntry;

/// This is just an alias for any list of edges
/// which are unrelated in their essence (i.e. do not belong to same Route)
///
/// It is mostly used when edges from different routes are
/// processed at the same time
pub type RouteBatch<'a> = Vec<&'a AdjacencyListEntry>;
