use std::{fmt::format, vec};

pub fn test() {
    let input = [0, 2, 3, 4, 6, 8, 9];

    let val = input.into_iter().collect();

    let output = summary_ranges(val);
    println!("{output:?}");
}

pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let len = nums.len();
    if len == 0 {
        return vec![];
    }

    let mut out = vec![];

    let mut rs = nums[0];
    for i in 1..len {
        let current = nums[i];
        let last = nums[i - 1];

        if current != last + 1 {
            if last == rs {
                out.push(format!("{last}"));
            } else {
                out.push(format!("{rs}->{last}"));
            }
            rs = current;
        }
    }

    if let Some(last) = nums.last() {
        if *last == rs {
            out.push(format!("{rs}"));
        } else {
            out.push(format!("{rs}->{}", last));
        }
    }

    out
}
