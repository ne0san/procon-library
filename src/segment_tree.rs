#[derive(Debug, Clone)]
pub struct SegmentTree {
    graph: Vec<UnionFindNode>,
    root_count: usize,
}
