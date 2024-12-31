pub fn test() {
    println!("{}", min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]));
}

pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut min_len = 0;

    let mut i = 0;
    let mut j = 0;

    let mut current_sum = nums[i];

    loop {
        if current_sum >= target {
            let len = j - i + 1;

            if len == 1 {
                return 1;
            } else if len < min_len || min_len == 0 {
                min_len = len;
            }

            current_sum -= nums[i];
            i += 1;
        } else {
            // current sum < target

            j += 1;
            if j >= nums.len() {
                break;
            }
            current_sum += nums[j];
        }
    }

    min_len as i32
}
