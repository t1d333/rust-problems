#![forbid(unsafe_code)]

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    let mut res = vec![];
    combinations_rec(arr, 0, k, vec![], &mut res);
    res
}

pub fn combinations_rec(
    arr: &[i32],
    start: usize,
    ln: usize,
    comb: Vec<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    if ln == 0 {
        res.push(comb);
        return;
    }

    for i in start..arr.len() {
        let mut tmp = comb.clone();
        tmp.push(arr[i]);
        combinations_rec(arr, i + 1, ln - 1, tmp, res)
    }
}
