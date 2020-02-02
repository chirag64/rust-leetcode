use std::collections::HashMap;

// Count soldiers in each row. Then sort the rows as per their soldier count. Return the first k row values
impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        
        // Count soldiers of each row
        let mut map = HashMap::new();
        let mut sum;
        for (index, row) in mat.iter().enumerate() {
            sum = 0;
            for cell in row {
                sum += cell;
            }
            let count = map.entry(index).or_insert(0);
            *count += sum;
        }
        
        // Sort the counts in ascending order, first by row number and then by count
        let mut count_vec: Vec<_> = map.iter().collect();
        count_vec.sort_by(|a, b| a.0.cmp(b.0));
        count_vec.sort_by(|a, b| a.1.cmp(b.1));
        
        // Return the first k rows with lowest number of soldiers
        let mut result = Vec::new();
        for i in 0..k {
            result.push(*(count_vec[i as usize].0) as i32);
        }
        return result;
    }
}
