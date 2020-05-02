impl Solution {
    // Convert both strings into two vectors of sorted characters. Then compare both vectors. Compare first with 2nd to check if any character in vec1 is less than vec2. If it is, s1 does not break s2. Then compare second with first to check if any character in vec2 is less than vec1. If it is, s2 does not break s1. If s1 does break s2, no need to do 2nd comparison, just return true.
    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut s1_vec: Vec<char> = s1.chars().collect();
        s1_vec.sort_by(|a, b| a.cmp(b));
        
        let mut s2_vec: Vec<char> = s2.chars().collect();
        s2_vec.sort_by(|a, b| a.cmp(b));
        
        let mut result = true;
        for (index, i) in s1_vec.iter().enumerate() {
            if (s1_vec[index] < s2_vec[index]) {
                result = false;
            }
        }
        
        if !result {
            result = true;
            for (index, i) in s1_vec.iter().enumerate() {
                if (s2_vec[index] < s1_vec[index]) {
                    result = false;
                }    
            }
        }
        
        return result;
    }
}
