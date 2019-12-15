// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use std::char;

// Create a recursive function to create string out of linked list. Then use OOTB method to convert binary string to decimal
impl Solution {
    pub fn string_from_list_node(head: Option<Box<ListNode>>, mut bin_string: String) -> String {
        match head {
            Some(node) => {
                // Convert i32 to char
                let c = char::from_digit(node.val as u32, 10).unwrap();
                bin_string.push(c);
                return Solution::string_from_list_node(node.next, bin_string)
            },
            None => return bin_string,
        }
    }
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut bin_string = String::new();
        bin_string = Solution::string_from_list_node(head, bin_string);

        // If linked list is empty, return 0
        if bin_string.chars().count() == 0 {
            return 0;
        }
        return i32::from_str_radix(&bin_string, 2).unwrap();
    }
}
