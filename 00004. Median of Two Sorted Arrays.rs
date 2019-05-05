impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // Calculate size of both vectors and combined vector
        let size1 = nums1.len();
        let size2 = nums2.len();
        let size = size1 + size2;
        
        // Create new vector that we'll populate combining the original two
        let mut vector = Vec::new();
        
        // Check if combined vector size will be odd or even
        let is_size_even = (size % 2 == 0);
        
        // Add 1 to half size for odd / even problem
        let half_size = (size / 2) + 1;
        
        let mut i = 0;
        let mut j = 0;
        
        loop {
            
            // If we've reached half size of the combined vector, stop since we only need median
            if ((i + j) == half_size) {
                let vec_size = vector.len();
                
                // If complete combined vector would be even size, divide the middle two elements and return the result
                // If complete combined vector would be odd size, simply return its last element (which would be the middle of the complete vector)
                if (is_size_even) {
                    return (vector[vec_size - 1] + vector[vec_size - 2]) as f64 / 2.0;
                } else {
                   return vector[vec_size - 1] as f64; 
                }
            }
            
            // If none of the 2 original vectors are out of bounds, push the lowest number of the current two in the new vector and move that vectors loop
            if (i < size1 && j < size2) {
                if (nums1[i] <= nums2[j]) {
                    vector.push(nums1[i]);
                    i += 1;
                } else {
                    vector.push(nums2[j]);
                    j += 1;
                }
            } else if (i < size1) {
                vector.push(nums1[i]);
                i += 1;
            } else {
                vector.push(nums2[j]);
                j += 1;
            }
        }
    }
}
