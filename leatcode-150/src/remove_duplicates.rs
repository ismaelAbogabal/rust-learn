pub mod remove_duplicate {
    pub fn test() {
        let mut nums = Vec::from([1, 1, 1, 2, 2, 3]);

        let out = remove_duplicates(&mut nums);

        println!("{:?}", &nums);
        println!("{:?}", out);
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();

        if len < 2 {
            return len as i32;
        }

        let mut write_index = 2;

        for index in 2..len {
            let current = nums[index];

            if current != nums[write_index - 2] {
                nums[write_index] = current;

                write_index += 1;
            }
        }

        write_index as i32
    }
}
