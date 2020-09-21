use std::collections::HashMap;

impl Solution {
    // Create a hashmap with mapping of the chars of s string to t string. Iterate through the first string's characters. For every character, if it exists in hashmap, check if it's mapping equals the corresponding character in t string. If mapping check fails, return false. If the character doesn't exist in hashmap, add the character in the hashmap with the corresponding t string's character as its value. Repeat same process by iterating through t string and maintaining its mapping in another hashmap
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let size = s.len();
        let mut s_to_t_map: HashMap<char, char> = HashMap::new();
        let mut t_to_s_map: HashMap<char, char> = HashMap::new();

        for i in 0..size {
            let s_char = s.chars().nth(i).unwrap();
            let t_char = t.chars().nth(i).unwrap();
            
            if s_to_t_map.contains_key(&s_char) {
                if *(s_to_t_map.get(&s_char).unwrap()) != t_char {
                    return false;
                }
            } else {
                s_to_t_map.insert(s_char, t_char);
            }
            
            if t_to_s_map.contains_key(&t_char) {
                if *(t_to_s_map.get(&t_char).unwrap()) != s_char {
                    return false;
                }
            } else {
                t_to_s_map.insert(t_char, s_char);
            }
        }
        return true;
    }
}
