struct DataStream {
    integers: Vec<i32>,
    value: i32,
    k: usize
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DataStream {
    // Create a struct with a vector for storing the stream of integers and other properties for storing value and k
    fn new(value: i32, k: i32) -> Self {
        return DataStream {
            integers: Vec::new(),
            value: value,
            k: k as usize
        }
    }
    
    // Add num to the vector of stream of integers. If vector size is smaller than k, return false. Run a loop to check the integers of last k integers in the stream of integers. If any of them is not 'value', return false. Else return true which would mean all integers in the loop were same as 'value'.
    fn consec(&mut self, num: i32) -> bool {
        self.integers.push(num);
        if self.integers.len() < self.k {
            return false;
        }
        
        for i in (self.integers.len() - self.k)..self.integers.len() {
            if self.integers[i] != self.value {
                return false;
            }
        }
        return true;
    }
}

/**
 * Your DataStream object will be instantiated and called as such:
 * let obj = DataStream::new(value, k);
 * let ret_1: bool = obj.consec(num);
 */
