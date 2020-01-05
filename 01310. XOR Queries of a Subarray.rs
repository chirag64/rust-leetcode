// Simple approach which I know is very likely to time out
impl Solution {
    pub fn xor(arr: Vec<i32>) -> i32 {
        let mut i = 1;
        let mut result = arr[0];
        loop {
            if (i == arr.len()) {
                return result;
            }
            result = result ^ arr[i];
            i += 1;
        }
        return result;
    }
    // Run a for loop over the vector queries and calculate the XOR among the subvectors
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for i in queries {
            result.push(Solution::xor((&arr[(i[0] as usize)..((i[1] + 1) as usize)]).to_vec()));
        }
        return result;
    }
}
