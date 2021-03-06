use crate::SortOrder;
use crate::SortOrder::*;

/// 配列をソートする。配列の要素が2^nではない場合、エラーを返す
/// # 引数
/// - array : 配列。ただし、要素数は2^nでなければならない。また、要素はOrdを実装しなければならない。
/// - order : ソート順
/// # 戻り値
/// - 要素数が2^nの場合Okを、それ以外の場合Errを返す
pub fn sort<T: Ord>(array: &mut [T], order: &SortOrder) -> Result<(), String> {
    match array.len() {
        0 => Ok(()),
        n => {
            if n.is_power_of_two() {
                do_sort(array, order);
                Ok(())
            } else {
                Err(format!(
                    "The length of array is not a power of two. (array.len(): {})",
                    array.len()
                ))
            }
        }
    }
}

/// 配列をソートする
/// # 引数
/// - array : 配列。ただし、要素数は2^nでなければならない
/// - order : ソート順
fn do_sort<T: Ord>(array: &mut [T], order: &SortOrder) {
    if array.len() <= 1 {
        return;
    }
    // バイトニック列を作る
    let mid = array.len() / 2;
    do_sort(&mut array[..mid], &Ascending); // 前半を昇順
    do_sort(&mut array[mid..], &Descending); // 後半を降順でソート
    sub_sort(array, order);
}

/// バイトニック列をソートする
/// #　引数
/// - bitonic_array : バイトニック列
/// - order : ソート順
fn sub_sort<T: Ord>(array: &mut [T], order: &SortOrder) {
    if array.len() <= 1 {
        return;
    }
    // 比較＆入れ替えによって並び順をascに近づける（ソートは不完全。半分にするとそれぞれがバイトニック列になる）
    compare_and_swap(array, order);
    let mid = array.len() / 2;
    sub_sort(&mut array[..mid], order);
    sub_sort(&mut array[mid..], order);
}

/// 各要素を要素数n / 2だけ右の要素と比較し、昇順か降順かに応じて並べ替える
/// 並び替えの結果、配列を半分に分けるとそれぞれがバイトニック列になる
/// # 引数
/// - array : 並び替え対象の配列
/// - order : ソート順
fn compare_and_swap<T: Ord>(array: &mut [T], order: &SortOrder) {
    let mid = array.len() / 2;

    for i in 0..mid {
        match *order {
            Ascending => {
                //　昇順の場合
                if array[i] > array[i + mid] {
                    array.swap(i, i + mid);
                }
            }
            Descending => {
                // 降順の場合
                if array[i] < array[i + mid] {
                    array.swap(i, i + mid);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    use crate::SortOrder::*;

    // 昇順ソートのテスト
    #[test]
    fn sort_u32_ascending() {
        let mut array: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

        assert!(sort(&mut array, &Ascending).is_ok());

        assert_eq!(array, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    //　降順ソートのテスト
    #[test]
    fn sort_u32_descending() {
        let mut array: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

        assert!(sort(&mut array, &Descending).is_ok());

        assert_eq!(array, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    // 要素が0個（昇順）
    #[test]
    fn sort_ascending_zero() {
        let mut array: Vec<u32> = vec![];

        assert!(sort(&mut array, &Ascending).is_ok());

        assert_eq!(array, vec![]);
    }

    // 要素が0個（降順）
    #[test]
    fn sort_descending_zero() {
        let mut array: Vec<u32> = vec![];

        assert!(sort(&mut array, &Descending).is_ok());

        assert_eq!(array, vec![]);
    }

    // 要素が1個（昇順）
    #[test]
    fn sort_ascending_one() {
        let mut array: Vec<u32> = vec![10];

        assert!(sort(&mut array, &Ascending).is_ok());

        assert_eq!(array, vec![10]);
    }

    // 要素が1個（降順）
    #[test]
    fn sort_descending_one() {
        let mut array: Vec<u32> = vec![45];

        assert!(sort(&mut array, &Descending).is_ok());

        assert_eq!(array, vec![45]);
    }

    // 昇順ソートのテスト（文字列）
    #[test]
    fn sort_str_ascending() {
        let mut array = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];

        assert!(sort(&mut array, &Ascending).is_ok());

        assert_eq!(
            array,
            vec![
                "GC",
                "Rust",
                "and",
                "fast",
                "is",
                "memory-efficient",
                "no",
                "with"
            ]
        );
    }

    //　降順ソートのテスト（文字列）
    #[test]
    fn sort_str_descending() {
        let mut array = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];

        assert!(sort(&mut array, &Descending).is_ok());

        assert_eq!(
            array,
            vec![
                "with",
                "no",
                "memory-efficient",
                "is",
                "fast",
                "and",
                "Rust",
                "GC"
            ]
        );
    }

    // 要素が2のべき乗個ではない場合
    #[test]
    fn sort_elemtns_not_power_of_two() {
        let mut array: Vec<u32> = vec![45, 21, 11];

        assert!(sort(&mut array, &Descending).is_err());
    }
}
