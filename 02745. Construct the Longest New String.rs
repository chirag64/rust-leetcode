use std::cmp::min;

impl Solution {
    // If x and y are same, use them alternatively and use all z ones on any end. If x and y are different, use them alternatively and wrap the highest number one with the lowest number one. Use all z ones on one of the sides wherever possible
    pub fn longest_string(x: i32, y: i32, z: i32) -> i32 {
        if x == y {
            return (x + y + z) * 2
        } else {            
            return (z + min(x, y) + (min(x, y) + 1)) * 2;
        }
    }
}
