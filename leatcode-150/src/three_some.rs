use std::collections::HashSet;

pub fn test() {
    let output = three_sum(vec![-1, 0, 1, 2, -1, -4]);

    println!("{output:?}");
}

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut out: HashSet<Vec<i32>> = HashSet::new();

    //todo
    for (index, current) in nums.iter().enumerate() {
        //todo try to get the best fit for it

        let mut min = index + 1;
        let mut max = nums.len() - 1;

        while max > min {
            let sum = (nums[max] + nums[min]) * -1;

            if sum == *current {
                out.insert(vec![*current, nums[min], nums[max]]);
                min += 1;
            } else if sum > *current {
                min += 1;
            } else {
                max -= 1;
            }
        }
    }

    Vec::from_iter(out)
}
