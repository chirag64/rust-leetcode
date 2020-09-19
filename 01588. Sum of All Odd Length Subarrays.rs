impl Solution {
    // Brute-force: Create subarrays from small to big and add them up to calculate the final result
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let size = arr.len();
        let mut result = 0;
        let mut vec_size = 1;
        let mut subarray_start;
        while vec_size <= size {
            subarray_start = 0;
            while (subarray_start + vec_size) <= size {
                for i in subarray_start..=(subarray_start + vec_size - 1) {
                    result += arr[i];
                }
                subarray_start += 1;
            }
            vec_size += 2;
        }
        return result;
    }
}
