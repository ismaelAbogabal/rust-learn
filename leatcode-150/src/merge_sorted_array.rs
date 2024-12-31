pub mod merge_sorted_array {

    pub fn test() {
        let mut nums1 = Vec::from([1, 2, 3, 0, 0, 0]);
        let mut nums2 = Vec::from([2, 5, 6]);
        merge(&mut nums1, 3, &mut nums2, 3);

        print!("{:?}", nums1);
    }

    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m = m as usize;
        let n = n as usize;

        //
        let mut insertion_index = 0;
        for i in 0..n {
            let current = nums2[i];
            nums1.pop();

            // we got to the end of the array
            loop {
                if insertion_index >= m + i {
                    //we are on the end of the array
                    nums1.insert(insertion_index, current);
                    insertion_index += 1;

                    break;
                }
                let insertion_num = nums1[insertion_index];

                if current <= insertion_num {
                    nums1.insert(insertion_index, current);
                    insertion_index += 1;
                    break;
                }

                insertion_index += 1;
            }
        }
    }
}
