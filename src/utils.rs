use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::hash::Hash;

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
/// * `left` - 左端インデックス
/// * `right` - 右端インデックス
/// * `judge` - midを元に判定するクロージャ okの場合true
pub fn bin_sch<F>(left: usize, right: usize, mut judge: F) -> (isize, isize)
where
    F: FnMut(usize) -> bool,
{
    let mut ok: isize = left as isize - 1isize;
    let mut ng: isize = (right + 1) as isize;
    while ng - ok > 1 {
        let mid: isize = ((ok + ng) / 2) as isize;
        if judge(mid as usize) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    (ok, ng)
}

/// 反復マージソート関数
/// ソート済みベクタを返却
///
/// # Arguments
/// * `arr` - ソートを行いたい任意の型のベクタ
/// * `cmp` - 比較用クロージャ |a, b| でaを先頭側に置きたい時trueを返す
pub fn marge_sort<T: Clone, F>(arr: &Vec<T>, mut cmp: F) -> Option<Vec<T>>
where
    F: FnMut(&T, &T) -> bool,
{
    if arr.is_empty() {
        // 空であるとき、Noneを返却
        None
    } else {
        // 「0usizeと「対象Vecの各要素それぞれを単一の要素として持つVecDequeue」」のタプルのVecDequeuを作成
        // マージ済みVecDequeueのVecDequeue
        let mut q: VecDeque<(bool, VecDeque<T>)> = (*arr)
            .clone()
            .into_iter()
            .map(|x| (false, vec![x].into()))
            .collect();

        loop {
            let mut left: (bool, VecDeque<T>) = q.pop_front().unwrap();
            if !q.is_empty() {
                // マージ済みVecDequeueが複数の時
                let next_loop: bool = !left.0;
                if left.0 == q[0].0 {
                    // 先頭二つのマージ済みVecDequeueが同一ループであるとき、それらをマージし、後端にpush
                    let mut right: (bool, VecDeque<T>) = q.pop_front().unwrap();
                    let mut marged: (bool, VecDeque<T>) = (next_loop, VecDeque::new());
                    while !left.1.is_empty() && !right.1.is_empty() {
                        marged.1.push_back(if cmp(&left.1[0], &right.1[0]) {
                            left.1.pop_front().unwrap()
                        } else {
                            right.1.pop_front().unwrap()
                        });
                    }
                    marged.1.append(&mut left.1);
                    marged.1.append(&mut right.1);
                    q.push_back(marged);
                } else {
                    // 先頭二つのマージ済みVecDequeueが同一ループでない時、ループカウントをインクリメントし、後端にpush
                    q.push_back((next_loop, left.1));
                }
            } else {
                // マージ済みVecDequeueが単一であるとき、それをVecに変換して返却
                return Some(left.1.into());
            }
        }
    }
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
    let arr = vec![1, 3, 5, 7, 9];
    let result = bin_sch(0, arr.len() - 1, |mid| arr[mid] <= 5);
    assert!(matches!(result, (2, 2) | (2, 3)));
}

// marge_sort 関数のテスト
#[test]
fn test_marge_sort_function() {
    let arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let sorted = marge_sort(&arr, |a, b| a <= b);
    assert_eq!(sorted, Some(vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]));
}
