impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        // Generate substring from query, check how many characters are needed to make it palindrome and then check if that is greater than k
        let mut i = 0;
        let mut j = 0;
        let size = queries.len();
        let mut output = vec!(false; size as usize);
        let mut sub_str: &str;
        let mut sub_str_size;
        let mut characters_to_make_palindrome;
        loop {
            if (i == size) {
                break;
            }
            // Generate substring
            sub_str = &s[(queries[i][0] as usize)..((queries[i][1] + 1) as usize)];
            sub_str_size = sub_str.len();
            // println!("{}", sub_str);
            characters_to_make_palindrome = 0;
            j = 0;
            loop {
                if (j >= (sub_str_size - 1 - j)) {
                    break;
                }
                // println!("{}, {}", sub_str.chars().nth(j).unwrap(), sub_str.chars().nth(sub_str_size - 1 - j).unwrap());
                // Check if the jth character from start and end is same, if it is not, add it to the count of characters needed to change to make it palindrome
                if (sub_str.chars().nth(j).unwrap() != sub_str.chars().nth(sub_str_size - 1 - j).unwrap()) {
                    characters_to_make_palindrome += 1;
                }
                j += 1;
            }
            // If the number of characters needed to change to make it palindrome is greater than k, we cannot make the string a palindrome, else we can
            if (characters_to_make_palindrome > queries[i][2]) {
                output[i] = false;
            } else {
                output[i] = true;
            }
            i += 1;
        }
        return output;
    }
}
