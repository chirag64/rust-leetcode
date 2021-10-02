impl Solution {
    // First calculate if original no of items can be fit in an m x n grid. If they can, loop through the outer vector and generating the rows and then adding them in the output vector
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if original.len() != ((m * n) as usize) {
            return [].to_vec();
        }
        
        let mut output = Vec::new();
        for i in 0..m {
            let mut inner_vec = Vec::new();
        
            for j in 0..n {
                inner_vec.push(original[((i * n) + j) as usize]);
            }
            
            output.push(inner_vec.to_vec());
        }
        return output;
    }
}
