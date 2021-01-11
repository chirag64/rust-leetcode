impl Solution {
    // To find the 2nd digit from an XOR operation, XOR the first digit with the output. In this case, push 'first' in the 'arr' and then XOR it with the first encoded digit. Then keep XORing every answer with the next encoded digit
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let len = encoded.len();
        let mut arr = Vec::new();
        let mut i = 1;
        
        arr.push(first);
        arr.push(first ^ encoded[0]);
        
        while i != len {
            arr.push(arr[i] ^ encoded[i]);
            i += 1;
        }
        
        return arr;
    }
}
