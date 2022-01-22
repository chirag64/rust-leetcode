impl Solution {
    // Sort the list in reverse order. Then add values of all elements in output variable, but skip every 3rd element since every 3rd element can be the free item with the 2 items before it
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        let mut output = 0;
        cost.sort();
        cost.reverse();
        for (index, c) in cost.iter().enumerate() {
            if (index + 1) % 3 != 0 {
                output += c;
            }
        }
        return output;
    }
}
