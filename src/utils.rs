use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::hash::Hash;
use std::mem::swap;

/// 複数の引数のうち最小値を返す
#[macro_export]
macro_rules! min_value {
    ($f:expr, $( $x:expr ),* ) => {
        {
            let mut temp_min = $f;
            $(
                temp_min = temp_min.min($x);
            )*
            temp_min
        }
    };
}

/// 複数の引数のうち最大値を返す
#[macro_export]
macro_rules! max_value {
    ($f:expr, $( $x:expr ),* ) => {
        {
            let mut temp_max = $f;
            $(
                temp_max = temp_max.max($x);
            )*
            temp_max
        }
    };
}

/// 第一引数を、第一引数と以降を含む引数のうち最小の値で上書きする
#[macro_export]
macro_rules! replace_min {
    ($t:expr; $( $x:expr ),* ) => {
        {
            $(
                $t = $t.min($x);
            )*
        }
    };
}

/// 第一引数を、第一引数と以降を含む引数のうち最大の値で上書きする
#[macro_export]
macro_rules! replace_max {
    ($t:expr; $( $x:expr ),* ) => {
        {
            $(
                $t = $t.max($x);
            )*
        }
    };
}

#[macro_export]
macro_rules! debug_print {
    ($($v:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!("| ", $(stringify!($v), ": {:?} | "),*), $(&$v),*);
    };
}

/// ベクタの各値をデリミタで区切って標準出力する
pub fn print_vector<T: Display>(vector: &Vec<T>, delimiter: &str) {
    let len = vector.len();
    for i in 0..len - 1 {
        print!("{}{}", vector[i], delimiter);
    }
    println!("{}", vector[len - 1]);
}

/// 二次元ベクタの各値をデリミタで区切って標準出力する
pub fn print_2d_vector<T: Display>(grid: &Vec<Vec<T>>, delimiter: &str) {
    for vector in grid {
        let v_len: usize = vector.len();
        for i in 0..v_len - 1 {
            print!("{}{}", vector[i], delimiter);
        }
        println!("{}", vector[v_len - 1]);
    }
}

/// HashMapの、keyのキーの値とcandidate_vをcmpメソッドに基づいて比較
/// cmpがtrueもしくはkeyのキーの値がない場合、insert
///
/// # Arguments
/// * `map` - 操作対象のHashMap
/// * `key` - mapの操作対象のkey
/// * `candidate_v` - 当該keyに対する新規value候補
/// * `cmp` - candidate_vと既存valueを比較するメソッド trueか当該キーのvalueがない時、insert
pub fn cmp_and_replace_value_in_hashmap<T, U, F>(
    map: &mut HashMap<T, U>,
    key: &T,
    candidate_v: U,
    mut cmp: F,
) where
    T: Eq + PartialEq + Hash + Copy,
    F: FnMut(&U, &U) -> bool,
{
    if let Some(existing_v) = map.get(key) {
        if cmp(&candidate_v, &existing_v) {
            map.insert(*key, candidate_v);
        }
    } else {
        // 既存valueがない時
        map.insert(*key, candidate_v);
    }
}

/// 二分探索関数
///
/// # Arguments
/// * `edge_l` - 橋インデックス片方
/// * `edge_r` - 橋インデックスもう片方
/// * `judge` - midを元に判定するクロージャ okの場合true
pub fn bin_sch<F>(edge_l: usize, edge_r: usize, mut judge: F) -> isize
where
    F: FnMut(usize) -> bool,
{
    let mut ok: isize = edge_l as isize - 1isize;
    let mut ng: isize = (edge_r + 1) as isize;
    while ng - ok > 1 {
        let mid: isize = ((ok + ng) / 2) as isize;
        if judge(mid as usize) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

/// 反復マージソート関数
/// ソート済みベクタを返却
///
/// # Arguments
/// * `v` - ソートを行いたい任意の型のベクタ
/// * `cmp` - 比較用クロージャ |a, b| でaを先頭側に置きたい時trueを返す
pub fn merge_sort<T: Clone, F>(v: &VecDeque<T>, mut cmp: F) -> Option<VecDeque<T>>
where
    F: FnMut(&T, &T) -> bool,
{
    // 「対象VecDequeの各要素それぞれを単一の要素として持つVecDequeue」のVecDequeを作成
    // マージ済みVecDequeのVecDeque
    let mut q: VecDeque<VecDeque<T>> = (*v).clone().into_iter().map(|x| vec![x].into()).collect();
    let mut next_q: VecDeque<VecDeque<T>> = VecDeque::new();
    while q.len() > 1 || !next_q.is_empty() {
        match (q.pop_front(), q.pop_front()) {
            (Some(mut first), Some(mut second)) => {
                let mut merged: VecDeque<T> = VecDeque::new();
                while !first.is_empty() && !second.is_empty() {
                    merged.push_back(if cmp(&first[0], &second[0]) {
                        first.pop_front().unwrap()
                    } else {
                        second.pop_front().unwrap()
                    });
                }
                merged.append(&mut first);
                merged.append(&mut second);
                next_q.push_back(merged);
            }
            (Some(first), None) => {
                next_q.push_back(first);
                swap(&mut q, &mut next_q);
            }
            (None, None) => {
                swap(&mut q, &mut next_q);
            }
            _ => unreachable!(),
        }
    }
    q.pop_front()
}

/// 不安定反復マージソート関数
/// ソート済みベクタを返却
///
/// # Arguments
/// * `v` - ソートを行いたい任意の型のベクタ
/// * `cmp` - 比較用クロージャ |a, b| でaを先頭側に置きたい時trueを返す
pub fn merge_sort_unstable<T: Clone, F>(v: &VecDeque<T>, mut cmp: F) -> Option<VecDeque<T>>
where
    F: FnMut(&T, &T) -> bool,
{
    let mut q: VecDeque<VecDeque<T>> = (*v).clone().into_iter().map(|x| vec![x].into()).collect();
    while q.len() > 1 {
        match (q.pop_front(), q.pop_front()) {
            (Some(mut first), Some(mut second)) => {
                let mut merged: VecDeque<T> = VecDeque::new();
                while !first.is_empty() && !second.is_empty() {
                    merged.push_back(if cmp(&first[0], &second[0]) {
                        first.pop_front().unwrap()
                    } else {
                        second.pop_front().unwrap()
                    });
                }
                merged.append(&mut first);
                merged.append(&mut second);
                q.push_back(merged);
            }
            _ => unreachable!(),
        }
    }
    q.pop_front()
}

/// 10進数で各桁の和を算出
pub fn sum_digits(mut v: usize) -> usize {
    let mut sum = 0;
    sum += v % 10;
    v /= 10;
    while v > 0 {
        sum += v % 10;
        v /= 10;
    }
    sum
}

// min_value マクロのテスト
#[test]
fn test_min_value_macro() {
    assert_eq!(min_value!(3, 5, 2, 8), 2);
    assert_eq!(min_value!(10, 5, 20, 8), 5);
}

// max_value マクロのテスト
#[test]
fn test_max_value_macro() {
    assert_eq!(max_value!(3, 5, 2, 8), 8);
    assert_eq!(max_value!(10, 5, 20, 8), 20);
}

// replace_min マクロのテスト
#[test]
fn test_replace_min_macro() {
    let mut value = 10;
    replace_min!(value; 5, 8, 12);
    assert_eq!(value, 5);
}

// replace_max マクロのテスト
#[test]
fn test_replace_max_macro() {
    let mut value = 10;
    replace_max!(value; 5, 8, 12);
    assert_eq!(value, 12);
}

// debug_print マクロのテスト
#[test]
fn test_debug_print_macro() {
    // デバッグモードでのみ出力されるため、テストは省略
}

// print_vector 関数のテスト
#[test]
fn test_print_vector_function() {
    let vec = vec![1, 2, 3, 4, 5];
    let delimiter = ",";
    print_vector(&vec, &delimiter);
    // 標準出力を確認する必要があるため、テストは省略
}

// print_2d_vector 関数のテスト
#[test]
fn test_print_2d_vector_function() {
    let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let delimiter = ",";
    print_2d_vector(&grid, &delimiter);
    // 標準出力を確認する必要があるため、テストは省略
}

// cmp_and_replace_value_in_hashmap 関数のテスト
#[test]
fn test_cmp_and_replace_value_in_hashmap_function() {
    let mut map = HashMap::new();
    map.insert(1, 10);
    cmp_and_replace_value_in_hashmap(&mut map, &1, 5, |a, b| a < b);
    assert_eq!(map.get(&1), Some(&5));
}

// bin_sch 関数のテスト
#[test]
fn test_bin_sch_function() {
    let v = vec![1, 3, 5, 7, 9];
    let result = bin_sch(0, v.len() - 1, |mid| v[mid] <= 5);
    assert!(matches!(result, 2));
}

// merge_sort 関数のテスト
#[test]
fn test_merge_sort_function() {
    // 奇数個
    let v: VecDeque<i32> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5].into();
    let sorted = merge_sort(&v, |a, b| a <= b);
    assert_eq!(sorted, Some(vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9].into()));

    // 偶数個
    let v: VecDeque<i32> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 7].into();
    let sorted = merge_sort(&v, |a, b| a <= b);
    assert_eq!(
        sorted,
        Some(vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 7, 9].into())
    );

    // 空
    let v: VecDeque<i32> = VecDeque::new();
    let sorted = merge_sort(&v, |a, b| a <= b);
    assert_eq!(sorted, None);

    // 安定ソートの確認 偶数個
    let v_with_index: VecDeque<(i32, usize)> = vec![
        (3, 0),
        (1, 1),
        (4, 2),
        (1, 3),
        (5, 4),
        (2, 6),
        (6, 7),
        (5, 8),
        (3, 9),
        (5, 10),
    ]
    .into();
    let sorted_with_index = merge_sort(&v_with_index, |&(a, _), &(b, _)| a <= b);
    assert_eq!(
        sorted_with_index,
        Some(
            vec![
                (1, 1),
                (1, 3),
                (2, 6),
                (3, 0),
                (3, 9),
                (4, 2),
                (5, 4),
                (5, 8),
                (5, 10),
                (6, 7),
            ]
            .into()
        )
    );
    let sorted_with_index = merge_sort_unstable(&v_with_index, |&(a, _), &(b, _)| a <= b);
    assert_ne!(
        sorted_with_index,
        Some(
            vec![
                (1, 1),
                (1, 3),
                (2, 6),
                (3, 0),
                (3, 9),
                (4, 2),
                (5, 4),
                (5, 8),
                (5, 10),
                (6, 7),
            ]
            .into()
        )
    );

    // 安定ソートの確認 奇数個
    let v_with_index: VecDeque<(i32, usize)> = vec![
        (3, 0),
        (1, 1),
        (4, 2),
        (1, 3),
        (5, 4),
        (9, 5),
        (2, 6),
        (6, 7),
        (5, 8),
        (3, 9),
        (5, 10),
    ]
    .into();
    let sorted_with_index = merge_sort(&v_with_index, |&(a, _), &(b, _)| a <= b);
    assert_eq!(
        sorted_with_index,
        Some(
            vec![
                (1, 1),
                (1, 3),
                (2, 6),
                (3, 0),
                (3, 9),
                (4, 2),
                (5, 4),
                (5, 8),
                (5, 10),
                (6, 7),
                (9, 5)
            ]
            .into()
        )
    );
    let sorted_with_index = merge_sort_unstable(&v_with_index, |&(a, _), &(b, _)| a <= b);
    assert_ne!(
        sorted_with_index,
        Some(
            vec![
                (1, 1),
                (1, 3),
                (2, 6),
                (3, 0),
                (3, 9),
                (4, 2),
                (5, 4),
                (5, 8),
                (5, 10),
                (6, 7),
                (9, 5)
            ]
            .into()
        )
    );
}

// merge_sort_unstable 関数のテスト
#[test]
fn test_merge_sort_unstable_function() {
    // 奇数個
    let v: VecDeque<i32> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5].into();
    let sorted = merge_sort_unstable(&v, |a, b| a <= b);
    assert_eq!(sorted, Some(vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9].into()));

    // 偶数個
    let v: VecDeque<i32> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 7].into();
    let sorted = merge_sort_unstable(&v, |a, b| a <= b);
    assert_eq!(
        sorted,
        Some(vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 7, 9].into())
    );

    // 空
    let v: VecDeque<i32> = VecDeque::new();
    let sorted = merge_sort_unstable(&v, |a, b| a <= b);
    assert_eq!(sorted, None);
}

#[test]
fn test_sum_digits() {
    let res = sum_digits(5);
    assert_eq!(5, res);

    let res = sum_digits(10);
    assert_eq!(1, res);

    let res = sum_digits(18);
    assert_eq!(9, res);

    let res = sum_digits(457);
    assert_eq!(16, res);
}
