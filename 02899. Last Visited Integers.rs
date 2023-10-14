impl Solution {
    // Create an output vector and a num vector. Num vector will contain list of numbers and a marker variable will keep a track of which was the last 'prev' value that was added in output vector.      
    pub fn last_visited_integers(words: Vec<String>) -> Vec<i32> {
        let mut output = Vec::new();
        let mut nums: Vec<i32> = Vec::new();
        let mut marker = -1;
        for word in words {
            if word == "prev" {
                if marker > -1 {
                    let s = nums[marker as usize];
                    marker -= 1;
                    output.push(s);
                } else {
                    output.push(-1);
                    marker = -1;
                }
            } else {
                nums.push(word.parse::<i32>().unwrap());
                marker = (nums.len() as i32) - 1;
            }
        }
        return output;
    }
}