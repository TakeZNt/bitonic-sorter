use crate::SortOrder;
use crate::SortOrder::*;
use std::cmp::Ordering;
use std::cmp::Ordering::*;

/// 配列をソートする。配列の要素が2^nではない場合、エラーを返す
/// # 引数
/// - array : 配列。ただし、要素数は2^nでなければならない。また、要素はOrdを実装しなければならない。
/// - order : ソート順
pub fn sort<T: Ord>(array: &mut [T], order: &SortOrder) -> Result<(), String> {
    match *order {
        Ascending => sort_by(array, &|a, b| a.cmp(&b)),
        Descending => sort_by(array, &|a, b| b.cmp(&a)),
    }
}

/// 配列をソートする。配列の要素が2^nではない場合、エラーを返す
/// # 引数
/// - array : 配列。ただし、要素数は2^nでなければならない。また、要素はOrdを実装しなければならない。
/// - comparator : 大小比較するためのクロージャ
/// # 戻り値
/// - 要素数が2^nの場合Okを、それ以外の場合Errを返す
pub fn sort_by<T, F>(array: &mut [T], comparator: &F) -> Result<(), String>
where
    F: Fn(&T, &T) -> Ordering,
{
    match array.len() {
        0 => Ok(()),
        n => {
            if n.is_power_of_two() {
                do_sort(array, comparator, true);
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
/// - comparator : 比較のためのクロージャ
/// - forward : 昇順の場合true, 降順の場合false
fn do_sort<T, F>(array: &mut [T], comparator: &F, forward: bool)
where
    F: Fn(&T, &T) -> Ordering,
{
    if array.len() <= 1 {
        return;
    }
    // バイトニック列を作る
    let mid = array.len() / 2;
    do_sort(&mut array[..mid], comparator, true); // 前半を昇順
    do_sort(&mut array[mid..], comparator, false); // 後半を降順でソート
    
    sub_sort(array, comparator, forward);
}

/// バイトニック列をソートする
/// #　引数
/// - bitonic_array : バイトニック列
/// - comparator : 比較のためのクロージャ
/// - forward : 昇順の場合true, 降順の場合false
fn sub_sort<T, F>(bitonic_array: &mut [T], comparator: &F, forward: bool)
where
    F: Fn(&T, &T) -> Ordering,
{
    if bitonic_array.len() <= 1 {
        return;
    }
    // 比較＆入れ替えによって並び順をascに近づける（ソートは不完全。半分にするとそれぞれがバイトニック列になる）
    compare_and_swap(bitonic_array, comparator, forward);

    let mid = bitonic_array.len() / 2;
    sub_sort(&mut bitonic_array[..mid], comparator, forward);
    sub_sort(&mut bitonic_array[mid..], comparator, forward);
}

/// 各要素を要素数n / 2だけ右の要素と比較し、昇順か降順かに応じて並べ替える
/// 並び替えの結果、配列を半分に分けるとそれぞれがバイトニック列になる
/// # 引数
/// - array : 並び替え対象の配列
/// - comparator : 比較のためのクロージャ
/// - forward : 昇順の場合true, 降順の場合false
fn compare_and_swap<T, F>(array: &mut [T], comparator: &F, forward: bool)
where
    F: Fn(&T, &T) -> Ordering,
{
    let mid = array.len() / 2;

    for i in 0..mid {
        if forward {
            //　昇順の場合
            if comparator(&array[i], &array[i + mid]) == Greater {
                array.swap(i, i + mid);
            }
        } else {
            // 降順の場合
            if comparator(&array[i], &array[i + mid]) == Less {
                array.swap(i, i + mid);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{sort, sort_by};
    use crate::utils::{is_sorted_ascending, is_sorted_descending, new_u32_vec};
    use crate::SortOrder::*;

    #[derive(Debug, PartialEq)] //これがないとassert_eq!ができない
    struct Student {
        first_name: String,
        last_name: String,
        age: u8,
    }

    impl Student {
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {
            Self {
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                age,
            }
        }
    }

    #[test]
    fn sort_students_by_age_ascending() {
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        let mut array = vec![&taro, &hanako, &kyoko, &ryosuke];

        let expected = vec![&hanako, &kyoko, &taro, &ryosuke];

        assert!(sort_by(&mut array, &|a, b| a.age.cmp(&b.age)).is_ok());

        assert_eq!(array, expected);
    }

    #[test]
    fn sort_students_by_name_ascending() {
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        let mut array = vec![&taro, &hanako, &kyoko, &ryosuke];

        let expected = vec![&ryosuke, &kyoko, &hanako, &taro];

        assert!(sort_by(&mut array, &|a, b| a
            .last_name
            .cmp(&b.last_name)
            .then_with(|| a.first_name.cmp(&b.first_name)))
        .is_ok());

        assert_eq!(array, expected);
    }

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

    #[test]
    fn sort_u32_large_ascending() {
        let mut x = new_u32_vec(2 << 16);

        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert!(is_sorted_ascending(&x));
    }

    #[test]
    fn sort_u32_large_descending() {
        let mut x = new_u32_vec(2 << 16);

        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert!(is_sorted_descending(&x));
    }
}
