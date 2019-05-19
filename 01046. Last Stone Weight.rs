impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        
        // Convert to mutable vector
        let mut mut_stones = stones.to_vec();
        let mut size = mut_stones.len();
        loop {
            // If only one element is left, return it
            if (size == 1) {
                return mut_stones[0];
            }
            // If no elements are left, return 0
            if (size == 0) {
                return 0;
            }
            
            // Sort the vector
            mut_stones.sort();
            
            // If second last and last vector are same, remove both, else assign second last to difference of the two and remove last
            if (mut_stones[size - 1] == mut_stones[size - 2]) {
                mut_stones.pop();
                mut_stones.pop();
                size -= 2;
            } else {
                mut_stones[size - 2] = mut_stones[size - 1] - mut_stones[size - 2];
                mut_stones.pop();
                size -= 1;
            }
        }
    }
}
