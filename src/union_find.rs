#[derive(Debug, Clone, Copy)]
enum UnionFindNode {
    Parent(usize),
    Size(usize),
}

#[derive(Debug, Clone)]
pub struct UnionFind {
    // ルートノードであるとき->グループのサイズ
    // ルートノードでないとき->親
    graph: Vec<UnionFindNode>,
    root_count: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        let new_union_find = Self {
            graph: vec![UnionFindNode::Size(1); size],
            root_count: size,
        };
        new_union_find
    }

    /// ルートの数を取得
    pub fn get_root_count(&self) -> usize {
        self.root_count
    }

    /// node_aとnode_bそれぞれを含むグループを結合する
    pub fn merge(&mut self, node_a: usize, node_b: usize) {
        let (root_a, size_a) = self.get_root_and_size(node_a);
        let (root_b, size_b) = self.get_root_and_size(node_b);
        if root_a != root_b {
            self.root_count -= 1;
            let (root_bigger, root_smaller) = if size_a >= size_b {
                (root_a, root_b)
            } else {
                (root_b, root_a)
            };
            self.graph[root_smaller] = UnionFindNode::Parent(root_bigger);
            self.graph[root_bigger] = UnionFindNode::Size(size_a + size_b);
        }
    }

    /// node_aとnode_bが同一のグループか判定
    pub fn same(&mut self, node_a: usize, node_b: usize) -> bool {
        self.get_root_and_size(node_a).0 == self.get_root_and_size(node_b).0
    }

    /// nodeのroot_nodeとnodeが属するグループのsizeを取得
    /// nodeの親をroot_nodeに再設定 (経路圧縮)
    pub fn get_root_and_size(&mut self, node: usize) -> (usize, usize) {
        let mut node_tmp: usize = node;
        loop {
            match self.graph[node_tmp] {
                UnionFindNode::Parent(parent) => node_tmp = parent,
                UnionFindNode::Size(size) => {
                    if node != node_tmp {
                        self.graph[node] = UnionFindNode::Parent(node_tmp);
                    }
                    return (node_tmp, size);
                }
            }
        }
    }
}
