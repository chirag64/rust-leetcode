use std::cmp::max;

impl Solution {
    // Loop through candies to figure out the highest count. Then loop through it again to check if each number in vector, when added with extra candies is >= highest. If it is then add true, else add false
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {        
        let mut highest = std::i32::MIN;
        let mut output = Vec::new();
        for &i in &candies {
            highest = max(highest, i);
        }
        for &i in &candies {
            output.push((i + extra_candies) >= highest);
        }
        return output;
    }
}
