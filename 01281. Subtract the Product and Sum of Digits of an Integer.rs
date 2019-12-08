impl Solution {
    // Convert number into an array & then loop through it, multiplying and adding at the same time. Then return the difference
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let str = n.to_string();
        let mut str_arr = str.split("");
        let mut product = 1;
        let mut sum = 0;
        for item in str_arr {
            let item_i32 = (*item).parse::<i32>().unwrap_or(-1);
            if item_i32 != -1 {
                product *= item_i32;
                sum += item_i32;
            }
        }
        return (product - sum);
    }
}
