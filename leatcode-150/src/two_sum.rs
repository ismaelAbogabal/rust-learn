pub fn test() {
    println!("{:?}", two_sum(vec![2, 7, 11, 15], 9));
}

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = numbers.len() - 1;

    while j > i {
        let sum = numbers[i] + numbers[j];

        if sum > target {
            j -= 1;
        } else if sum < target {
            i += 1;
        } else {
            return vec![i as i32 + 1, j as i32 + 1];
        }
    }

    panic!("No solution")
}
