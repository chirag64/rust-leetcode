impl Solution {
    // Run a loop where in each second memory gets reduced from each memory stick. If both memory sticks don't have enough memory left, return the result with the number of second and the memory left on both the sticks
    pub fn mem_leak(mut memory1: i32, mut memory2: i32) -> Vec<i32> {
        let mut second = 1;
        while (second <= memory1) || (second <= memory2) {
            if memory1 >= memory2 {
                memory1 -= second;
            } else {
                memory2 -= second;
            }
            second += 1;
        }
        return vec![second, memory1, memory2];
    }
}
