// 配列をソートする
// array : 配列
// asc : 昇順の場合true、降順の場合false
pub fn sort<T: Ord>(array: &mut[T], asc: bool) {
    if array.len() <= 1 {
        return;
    }

    let mid = array.len() / 2;
    sort(&mut array[..mid], true); // 前半を昇順
    sort(&mut array[mid..], false); // 後半を降順でソート
    sub_sort(array, asc);
}

// バイトニック数列をソートする
// array : バイトニック配列
// asc : 昇順の場合true、降順の場合false
fn sub_sort<T: Ord>(array: &mut[T], asc: bool) {
    if array.len() <= 1 {
        return;
    }
    // 比較＆入れ替えによって並び順をascに近づける（ソートは不完全）
    compare_and_swap(array, asc);
    let mid = array.len() / 2;
    sub_sort(&mut array[..mid], asc);
    sub_sort(&mut array[mid..], asc);
}

// 昇順か降順かに応じて前半と後半がそれぞれバイトニック数列になるように要素を並べ替える
fn compare_and_swap<T: Ord>(array: &mut[T], asc: bool){
    let mid = array.len() / 2;

    for i in 0..mid {
        if asc {
            //　昇順の場合
            if array[i] > array[i + mid] {
                array.swap(i, i + mid);
            }
        }
        else {
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
        let mut array: Vec<u32> = vec![10, 30 , 11, 20, 4, 330, 21, 110];
        
        sort(&mut array, true);

        assert_eq!(array, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    //　降順ソートのテスト
    #[test]
    fn sort_u32_descending() {
        let mut array: Vec<u32> = vec![10, 30 , 11, 20, 4, 330, 21, 110];
        
        sort(&mut array, false);

        assert_eq!(array, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    // 要素が0個（昇順）
    #[test]
    fn sort_ascending_zero() {
        let mut array: Vec<u32> = vec![];
        
        sort(&mut array, true);

        assert_eq!(array, vec![]);
    }

    // 要素が0個（降順）
    #[test]
    fn sort_descending_zero() {
        let mut array: Vec<u32> = vec![];
        
        sort(&mut array, false);

        assert_eq!(array, vec![]);
    }

    // 要素が1個（昇順）
    #[test]
    fn sort_ascending_one() {
        let mut array: Vec<u32> = vec![10];
        
        sort(&mut array, true);

        assert_eq!(array, vec![10]);
    }

    // 要素が1個（降順）
    #[test]
    fn sort_descending_one() {
        let mut array: Vec<u32> = vec![45];
        
        sort(&mut array, false);

        assert_eq!(array, vec![45]);
    }

    // 昇順ソートのテスト（文字列）
    #[test]
    fn sort_str_ascending() {
        let mut array = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        
        sort(&mut array, true);

        assert_eq!(array, vec!["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"]);
    }

    //　降順ソートのテスト（文字列）
    #[test]
    fn sort_str_descending() {
        let mut array = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        
        sort(&mut array, false);

        assert_eq!(array, vec!["with", "no", "memory-efficient", "is", "fast", "and", "Rust", "GC"]);
    }
}