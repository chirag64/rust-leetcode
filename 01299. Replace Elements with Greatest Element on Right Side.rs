impl Solution {
    
    // Iterate through the array from right to left. Keep track of highest number till now and replace current number with the highest till now
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut temp1 = -1;
        let mut temp2 = -1;
        for i in (0..arr.len()).rev() {
            if (i == arr.len() - 1) {
                temp1 = arr[arr.len() - 1];
                arr[i] = -1;
            } else {
                temp2 = arr[i];
                arr[i] = temp1;
                temp1 = std::cmp::max(temp1, temp2);                
            }
        }
        return arr;
    }
}
