impl Solution {
    // Create concatenated string with n, 2n and 3n. Then loop through a list from 1 to 9 and if any of those digits aren't present exactly once, return false. If it has 0, return false. Else return true.
    pub fn is_fascinating(n: i32) -> bool {
        let mut concatenated_string = n.to_string() + &((n * 2).to_string()) + &((n * 3).to_string());

        let zeroes:Vec<_> = concatenated_string.match_indices("0").collect();
        if (zeroes.len() == 1) {
            return false;
        }
        
        for i in 1..10 {
            let digit_occurrences:Vec<_> = concatenated_string.match_indices(&(i).to_string()).collect();
            if digit_occurrences.len() != 1 {
                return false;
            }
        }
        
        return true;
    }
}
