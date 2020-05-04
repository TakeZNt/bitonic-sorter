use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

/// ランダムな整数を要素に持つ指定サイズの配列を返す
/// # 引数
/// n : サイズ
///
/// # 戻り値
/// ランダムな整数を要素に持つ指定サイズの配列
///
pub fn new_u32_vec(n: usize) -> Vec<u32> {
    let rng = Pcg64Mcg::from_seed([0; 16]);
    rng.sample_iter(&Standard).take(n).collect()
}

/// 昇順にソートされているかどうかを返す
pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|pair| pair[0] <= pair[1])
}

/// 降順にソートされているかどうかを返す
pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|pair| pair[0] >= pair[1])
}
