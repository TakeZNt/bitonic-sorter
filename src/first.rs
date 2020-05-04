/// 配列をソートする
/// # 引数
/// - array : 配列。ただし、要素数は2^nでなければならない
/// - asc : 昇順の場合true、降順の場合false
pub fn sort(array: &mut [u32], asc: bool) {
    if array.len() <= 1 {
        return;
    }

    // バイトニック列を作成する
    let mid = array.len() / 2;
    sort(&mut array[..mid], true); // 前半を昇順でソート
    sort(&mut array[mid..], false); // 後半を降順でソート
    sub_sort(array, asc);
}

/// バイトニック列をソートする
/// #　引数
/// - bitonic_array : バイトニック列
/// - asc : 昇順の場合true、降順の場合false
fn sub_sort(bitonic_array: &mut [u32], asc: bool) {
    if bitonic_array.len() <= 1 {
        return;
    }
    // 比較＆入れ替えによって並び順をascに近づける（ソートは不完全。半分にするとそれぞれがバイトニック列になる）
    compare_and_swap(bitonic_array, asc);

    let mid = bitonic_array.len() / 2;
    sub_sort(&mut bitonic_array[..mid], asc);
    sub_sort(&mut bitonic_array[mid..], asc);
}

/// 各要素を要素数n / 2だけ右の要素と比較し、昇順か降順かに応じて並べ替える
/// 並び替えの結果、配列を半分に分けるとそれぞれがバイトニック列になる
/// # 引数
/// - array : 並び替え対象の配列
/// - asc : 昇順の場合true、降順の場合false
fn compare_and_swap(array: &mut [u32], asc: bool) {
    let mid = array.len() / 2;

    for i in 0..mid {
        if asc {
            //　昇順の場合
            if array[i] > array[i + mid] {
                array.swap(i, i + mid);
            }
        } else {
            // 降順の場合
            if array[i] < array[i + mid] {
                array.swap(i, i + mid);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;

    // 昇順ソートのテスト
    #[test]
    fn sort_u32_ascending() {
        let mut array = vec![10, 30, 11, 20, 4, 330, 21, 110];

        sort(&mut array, true);

        assert_eq!(array, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    //　降順ソートのテスト
    #[test]
    fn sort_u32_descending() {
        let mut array = vec![10, 30, 11, 20, 4, 330, 21, 110];

        sort(&mut array, false);

        assert_eq!(array, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    // 要素が0個（昇順）
    #[test]
    fn sort_ascending_zero() {
        let mut array = vec![];

        sort(&mut array, true);

        assert_eq!(array, vec![]);
    }

    // 要素が0個（降順）
    #[test]
    fn sort_descending_zero() {
        let mut array = vec![];

        sort(&mut array, false);

        assert_eq!(array, vec![]);
    }

    // 要素が1個（昇順）
    #[test]
    fn sort_ascending_one() {
        let mut array = vec![10];

        sort(&mut array, true);

        assert_eq!(array, vec![10]);
    }

    // 要素が1個（降順）
    #[test]
    fn sort_descending_one() {
        let mut array = vec![45];

        sort(&mut array, false);

        assert_eq!(array, vec![45]);
    }
}
