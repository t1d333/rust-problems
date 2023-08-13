#![forbid(unsafe_code)]

pub fn longest_common_prefix(strs: Vec<&str>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut prefix = String::from(strs[0]);
    let mut res = prefix.clone();

    for item in strs.iter().skip(1) {
        let mut str_it = item.char_indices();
        let mut pr_it = prefix.char_indices();
        loop {
            let p1 = str_it.next();
            let p2 = pr_it.next();

            if p1.is_none() && p2.is_some() {
                res = String::from(*item);
                break;
            }

            if p2.is_none() {
                break;
            }

            let (idx_s, ch_s) = p1.unwrap();
            let (idx_p, ch_p) = p2.unwrap();

            if idx_p != idx_s || ch_s != ch_p {
                res = String::from(&prefix[0..idx_p]);
                break;
            }
        }
        prefix = res.clone();
    }

    res
}
