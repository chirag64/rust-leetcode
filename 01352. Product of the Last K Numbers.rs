struct ProductOfNumbers {
    val: Vec<i32>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {

    fn new() -> Self {
        return ProductOfNumbers {
            val: Vec::new()
        };
    }
    
    fn add(&mut self, num: i32) {
        self.val.push(num);
    }
    
    fn get_product(&self, k: i32) -> i32 {
        let length = self.val.len() as i32;
        let mut product = 1;
        
        for i in ((length - k)..length).rev() {
            product *= self.val[i as usize];
        }
        return product;
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */
