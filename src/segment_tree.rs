use std::collections::VecDeque;
use std::fmt::Debug;
use std::iter::FromIterator;
#[derive(Debug, Clone)]
pub struct SegmentTree<T: Clone + Debug, F>
where
    F: Fn(T, T) -> T,
{
    cells: Vec<Vec<T>>,
    cal: F,
}

impl<T: Clone + Debug + Copy, F> SegmentTree<T, F>
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
        cells.reverse();
        SegmentTree {
            cells: cells,
            cal: cal,
        }
    }
    pub fn update(&mut self, pos: usize, v: T) {}
    pub fn query(&self, left: usize, right: usize) -> T {
        // 対象範囲を列挙して全てにcal
        self.cells[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::{max, min};

    #[test]
    fn test_new_segment_tree() {
        let st = SegmentTree::new(vec![1, 2, 3, 4, 5, 6], |a, b| max(a, b));
        assert_eq!(
            st.cells,
            vec![vec![6], vec![4, 6], vec![2, 4, 6], vec![1, 2, 3, 4, 5, 6]]
        );
        let st = SegmentTree::new(vec![1, 2], |a, b| max(a, b));
        assert_eq!(st.cells, vec![vec![2], vec![1, 2]]);

        let st = SegmentTree::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 2], |a, b| max(a, b));
        assert_eq!(
            st.cells,
            vec![
                vec![8],
                vec![8, 2],
                vec![4, 8, 2],
                vec![2, 4, 6, 8, 2],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 2]
            ]
        );
    }

    #[test]
    fn test_update() {
        let mut st = SegmentTree::new(vec![1, 2, 3, 4, 5, 6], |a, b| max(a, b));
        st.update(0, 7);
        assert_eq!(
            st.cells,
            vec![vec![7], vec![7, 6], vec![7, 4, 6], vec![7, 2, 3, 4, 5, 6]]
        );
        st.update(0, 1);
        assert_eq!(
            st.cells,
            vec![vec![6], vec![2, 6], vec![2, 4, 6], vec![1, 2, 3, 4, 5, 6]]
        );
        st.update(6, 555);
        assert_eq!(
            st.cells,
            vec![vec![1], vec![1, 6], vec![1, 4, 6], vec![1, 2, 3, 4, 5, 6]]
        );
        st.update(3, 2);
        assert_eq!(
            st.cells,
            vec![vec![6], vec![3, 6], vec![1, 3, 6], vec![1, 2, 3, 2, 5, 6]]
        );
        let mut st = SegmentTree::new(vec![2, 1], |a, b| min(a, b));
        st.update(0, 1);
        assert_eq!(st.cells, vec![vec![1], vec![1, 1]]);
        st.update(1, 0);
        assert_eq!(st.cells, vec![vec![0], vec![1, 0]]);

        let mut st = SegmentTree::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 2], |a, b| min(a, b));
        assert_eq!(
            st.cells,
            vec![
                vec![8],
                vec![8, 2],
                vec![4, 8, 2],
                vec![2, 4, 6, 8, 2],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 2]
            ]
        );
    }
}
