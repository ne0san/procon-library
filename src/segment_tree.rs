use std::collections::VecDeque;
use std::fmt::Debug;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct SegmentTree<T: Clone + Debug + Copy + PartialEq, F>
where
    F: Fn(T, T) -> T,
{
    cells: Vec<Vec<T>>,
    cal: F,
}

impl<T: Clone + Debug + Copy + PartialEq, F> SegmentTree<T, F>
where
    F: Fn(T, T) -> T,
{
    pub fn new(init: Vec<T>, cal: F) -> SegmentTree<T, F> {
        let mut cells = vec![init.clone()];
        let mut que: VecDeque<T> = init.clone().into();
        let mut next = vec![];
        loop {
            match (que.pop_front(), que.pop_front()) {
                (Some(v1), Some(v2)) => next.push(cal(v1, v2)),
                (Some(v), None) => {
                    if next.is_empty() {
                        break;
                    } else {
                        next.push(v);
                        que = next.clone().into();
                        cells.push(next);
                        next = vec![];
                    }
                }
                _ => {
                    if next.is_empty() {
                        break;
                    } else {
                        que = next.clone().into();
                        cells.push(next);
                        next = vec![];
                    }
                }
            }
        }
        SegmentTree {
            cells: cells,
            cal: cal,
        }
    }
    pub fn update(&mut self, pos: usize, v: T) {
        self.cells[0][pos] = v;
        for i in 1..self.cells.len() {
            let b_pos = pos / (1 << i);
            let before = self.cells[i][b_pos];
            if b_pos * 2 + 1 >= self.cells[i - 1].len() {
                self.cells[i][b_pos] = self.cells[i - 1][b_pos * 2]
            } else {
                self.cells[i][b_pos] = (self.cal)(
                    self.cells[i - 1][b_pos * 2],
                    self.cells[i - 1][b_pos * 2 + 1],
                );
            }
            if self.cells[i][b_pos] == before {
                break;
            }
        }
        // 末端から更新
        // 更新できなくなるまで更新
    }
    pub fn query(&self, left: usize, right: usize) -> T {
        // 対象範囲を列挙して全てにcal
        // 検査対象の(row,col)
        let mut vals = vec![];
        let mut que = VecDeque::from_iter(vec![(self.cells.len() - 1, 0)]);
        while let Some((c_row, c_col)) = que.pop_front() {
            let (c_l, c_r) = self.cell_range(c_row, c_col);
            if c_l > right || c_r < left {
                // チェック対象セルが、確認範囲に完全に含まれない場合
                continue;
            } else if c_l >= left && c_r <= right {
                // チェック対象セルが、完全に含まれる場合
                vals.push(self.cells[c_row][c_col]);
                continue;
            } else if c_row > 0 {
                //半端に含まれている場合、より細かいセルをチェック対象に
                que.push_back((c_row - 1, c_col * 2));
                que.push_back((c_row - 1, c_col * 2 + 1));
            }
        }
        match vals.split_first() {
            Some((&first, rest)) => rest.iter().fold(first, |res, &v| (self.cal)(res, v)),
            None => unreachable!(),
        }
    }
    // セルが含まれる区間を算出
    fn cell_range(&self, row: usize, column: usize) -> (usize, usize) {
        let base = 1 << row;
        (
            base * column,
            self.cells[0].len().min(base * (column + 1)) - 1,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::{max, min};

    #[test]
    fn test_new() {
        let st = SegmentTree::new(vec![1, 2, 3, 4, 5, 6], |a, b| max(a, b));
        assert_eq!(
            st.cells,
            vec![vec![1, 2, 3, 4, 5, 6], vec![2, 4, 6], vec![4, 6], vec![6]]
        );
        let st = SegmentTree::new(vec![1, 2], |a, b| max(a, b));
        assert_eq!(st.cells, vec![vec![1, 2], vec![2]]);

        let st = SegmentTree::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 2], |a, b| max(a, b));
        assert_eq!(
            st.cells,
            vec![
                vec![1, 2, 3, 4, 5, 6, 7, 8, 2],
                vec![2, 4, 6, 8, 2],
                vec![4, 8, 2],
                vec![8, 2],
                vec![8]
            ]
        );
    }

    #[test]
    fn test_update() {
        let mut st = SegmentTree::new(vec![1, 2, 3, 4, 5, 6], |a, b| max(a, b));
        st.update(0, 7);
        assert_eq!(
            st.cells,
            vec![vec![7, 2, 3, 4, 5, 6], vec![7, 4, 6], vec![7, 6], vec![7]]
        );
        st.update(0, 1);
        assert_eq!(
            st.cells,
            vec![vec![1, 2, 3, 4, 5, 6], vec![2, 4, 6], vec![4, 6], vec![6]]
        );
        st.update(3, 2);
        assert_eq!(
            st.cells,
            vec![vec![1, 2, 3, 2, 5, 6], vec![2, 3, 6], vec![3, 6], vec![6]]
        );
        let mut st = SegmentTree::new(vec![2, 1], |a, b| min(a, b));
        st.update(0, 1);
        assert_eq!(st.cells, vec![vec![1, 1], vec![1]]);
        st.update(1, 0);
        assert_eq!(st.cells, vec![vec![1, 0], vec![0]]);

        let mut st = SegmentTree::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 2], |a, b| min(a, b));
        assert_eq!(
            st.cells,
            vec![
                vec![1, 2, 3, 4, 5, 6, 7, 8, 2],
                vec![1, 3, 5, 7, 2],
                vec![1, 5, 2],
                vec![1, 2],
                vec![1]
            ]
        );
        st.update(0, 2555);
        assert_eq!(
            st.cells,
            vec![
                vec![2555, 2, 3, 4, 5, 6, 7, 8, 2],
                vec![2, 3, 5, 7, 2],
                vec![2, 5, 2],
                vec![2, 2],
                vec![2]
            ]
        );
        st.update(8, 0);
        assert_eq!(
            st.cells,
            vec![
                vec![2555, 2, 3, 4, 5, 6, 7, 8, 0],
                vec![2, 3, 5, 7, 0],
                vec![2, 5, 0],
                vec![2, 0],
                vec![0]
            ]
        );
    }

    #[test]
    fn test_query() {
        let st = SegmentTree::new(vec![1, 2, 3, 4, 5, 6], |a, b| max(a, b));
        assert_eq!(st.query(0, 5), 6);
        assert_eq!(st.query(0, 0), 1);
        assert_eq!(st.query(0, 1), 2);
        assert_eq!(st.query(3, 5), 6);
        assert_eq!(st.query(0, 4), 5);
        let st = SegmentTree::new(vec![1, 2], |a, b| max(a, b));
        assert_eq!(st.query(0, 0), 1);
        assert_eq!(st.query(1, 1), 2);

        let st = SegmentTree::new(vec![1, 2, 5, 1, 9, 6, 7, 8, 2], |a, b| max(a, b));
        assert_eq!(st.query(0, 0), 1);
        assert_eq!(st.query(1, 1), 2);
        assert_eq!(st.query(0, 2), 5);
        assert_eq!(st.query(3, 5), 9);
        assert_eq!(st.query(4, 8), 9);
        assert_eq!(st.query(5, 8), 8);
    }
    #[test]
    fn test_cell_range() {
        let st = SegmentTree::new(vec![1, 2, 3, 4, 5, 6, 7], |a, b| max(a, b));
        assert_eq!(
            st.cells,
            vec![
                vec![1, 2, 3, 4, 5, 6, 7],
                vec![2, 4, 6, 7],
                vec![4, 7],
                vec![7]
            ]
        );
        assert_eq!(st.cell_range(0, 5), (5, 5));
        assert_eq!(st.cell_range(0, 0), (0, 0));
        assert_eq!(st.cell_range(0, 1), (1, 1));
        assert_eq!(st.cell_range(1, 1), (2, 3));
        assert_eq!(st.cell_range(1, 3), (6, 6));
        assert_eq!(st.cell_range(2, 0), (0, 3));
        assert_eq!(st.cell_range(2, 1), (4, 6));
        assert_eq!(st.cell_range(3, 0), (0, 6));
    }
}
