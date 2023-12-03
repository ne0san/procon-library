#[derive(Debug, Clone, Copy)]
enum WeightedUnionFindNode {
    ParentAndWeight(usize, isize), //親ノード番号と親への重み
    Size(usize),
}

#[derive(Debug, Clone)]
pub struct WeightedUnionFind {
    // ルートノードであるとき->グループのサイズ
    // ルートノードでないとき->親と親への重み
    graph: Vec<WeightedUnionFindNode>,
    root_count: usize,
}

impl WeightedUnionFind {
    pub fn new(size: usize) -> Self {
        let new_union_find = Self {
            graph: vec![WeightedUnionFindNode::Size(1); size],
            root_count: size,
        };
        new_union_find
    }

    /// ルートの数を取得
    pub fn get_root_count(&self) -> usize {
        self.root_count
    }

    /// node_aとnode_bそれぞれを含むグループを結合する
    /// true: 結合成功時、もしくは既結合かつweightに矛盾がない時
    /// false: 既結合かつweightに矛盾がある時
    pub fn merge(&mut self, node_a: usize, node_b: usize, weight: isize) -> bool {
        let (root_a, size_a, weight_a_to_root) = self.get_root_size_weight(node_a);
        let (root_b, size_b, weight_b_to_root) = self.get_root_size_weight(node_b);

        if root_a != root_b {
            self.root_count -= 1;
            let (root_bigger, root_smaller, set_weight) = if size_a >= size_b {
                (root_a, root_b, weight + weight_a_to_root - weight_b_to_root)
            } else {
                (root_b, root_a, weight_b_to_root - weight - weight_a_to_root)
            };
            self.graph[root_smaller] =
                WeightedUnionFindNode::ParentAndWeight(root_bigger, set_weight);
            self.graph[root_bigger] = WeightedUnionFindNode::Size(size_a + size_b);
            true
        } else {
            if (weight_b_to_root - weight_a_to_root) == weight {
                true
            } else {
                false
            }
        }
    }

    /// node_aとnode_bが同一のグループか判定
    pub fn same(&mut self, node_a: usize, node_b: usize) -> bool {
        self.get_root_size_weight(node_a).0 == self.get_root_size_weight(node_b).0
    }

    /// 以下を取得
    /// - nodeのroot_node
    /// - nodeが属するグループのsize
    /// - nodeからrootまでのweight
    /// nodeの親をroot_nodeに再設定 (経路圧縮)
    pub fn get_root_size_weight(&mut self, node: usize) -> (usize, usize, isize) {
        let mut node_tmp: usize = node;
        let mut weight_tmp: isize = 0;
        loop {
            match self.graph[node_tmp] {
                WeightedUnionFindNode::ParentAndWeight(parent, weight) => {
                    weight_tmp += weight;
                    node_tmp = parent;
                }
                WeightedUnionFindNode::Size(size) => {
                    if node != node_tmp {
                        self.graph[node] =
                            WeightedUnionFindNode::ParentAndWeight(node_tmp, weight_tmp);
                    }
                    return (node_tmp, size, weight_tmp);
                }
            }
        }
    }
}
// WeightedUnionFindのテスト
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let uf = WeightedUnionFind::new(5);
        assert_eq!(uf.get_root_count(), 5);
    }

    #[test]
    fn test_merge() {
        let mut uf = WeightedUnionFind::new(5);
        assert!(uf.merge(0, 1, 2));
        assert!(uf.merge(2, 3, 3));
        assert!(uf.merge(1, 3, 5));
        assert!(!uf.merge(1, 3, 3));
        assert_eq!(uf.get_root_count(), 2);
    }

    #[test]
    fn test_same() {
        let mut uf = WeightedUnionFind::new(5);
        uf.merge(0, 1, 2);
        uf.merge(2, 3, 3);
        assert!(uf.same(0, 1));
        assert!(!uf.same(1, 4));
    }

    #[test]
    fn test_get_root_size_weight() {
        let mut uf = WeightedUnionFind::new(5);
        uf.merge(0, 1, 2);
        uf.merge(2, 3, 3);
        let (root, size, weight) = uf.get_root_size_weight(3);
        assert_eq!(root, 2);
        assert_eq!(size, 2);
        assert_eq!(weight, 3);
    }
}
