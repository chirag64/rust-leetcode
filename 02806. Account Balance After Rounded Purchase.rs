impl Solution {
    // Find last number of purchase amount. If last digit is less than 5, deduct it from the purchase amount to round down. If last digit is 5 or above, add a number to the purchase amount to round it up. Return 100 - the newly calculated purchase amount
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        let mut actual_purchase_amount;
        if purchase_amount % 10 < 5 {
            actual_purchase_amount = purchase_amount - (purchase_amount % 10);
        } else {
            actual_purchase_amount = purchase_amount + (10 - (purchase_amount % 10));
        }
        return 100 - actual_purchase_amount;
    }
}
