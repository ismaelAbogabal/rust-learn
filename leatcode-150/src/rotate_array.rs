pub mod rotate_array {
    pub fn test1() {
        let mut nums = Vec::from([1, 2, 3, 4, 5, 6, 7]);

        rotate(&mut nums, 3);

        println!("{:?}", nums);
    }

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        //nums.reverse();
        //
        //nums[0..k].reverse();
        //nums[k..].reverse();
        //
    }
}
