impl Solution {
    
    // Create a stack of indexes of opening brackets. When you encounter closing brackets, pop the stack and check if they match. The idea is that every closing bracket will be a complementary bracket of the last opening bracket. Always.
    pub fn is_valid(s: String) -> bool {
        
        // Create vectors to recognize opening and closing brackets
        let openers = ['(', '{', '['];
        let closers = [')', '}', ']'];
        
        // Create stack to keep track of last opening bracket
        let mut stack:Vec<i32> = vec![];
        
        for char in s.chars() {
            
            // If it is an opening bracket, add to stack
            if openers.contains(&char) {
                let opener_index = openers.iter().position(|&x| x == char).unwrap() as i32;
                stack.push(opener_index);
            } else {
                // If it is a closing bracket, check if it is the complementary of the last added opening bracket in the stack
                if stack.len() == 0 {
                    return false;
                }
                let closer_index = closers.iter().position(|&x| x == char).unwrap() as i32;
                let last_stack_element = stack.pop().unwrap();
                if closer_index != last_stack_element {
                    return false;
                }
            }
        }
        if stack.len() != 0 {
            return false;
        }
        
        return true;
    }
}
