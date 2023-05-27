impl Solution {
    // Maintain 2 variables: lowest and second_lowest keeping track of the 2 lowest prices. After looping through prices, if sum of both lowest is not higher than money that we have, return the money left after subtracting both lowest from money. Else return money.
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut lowest = i32::MAX;
        let mut second_lowest = i32::MAX;
        
        for price in prices {
            if price < lowest {
                second_lowest = lowest;
                lowest = price;
            } else if price < second_lowest {
                second_lowest = price;
            }
        }
        
        if (lowest + second_lowest) <= money {
            return money - (lowest + second_lowest)
        } else {
            return money;
        }
    }
}
