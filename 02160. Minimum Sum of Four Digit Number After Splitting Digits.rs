impl Solution {
    // Split number into Vec<char> and then sort the characters. Then generate two 2-digit numbers, first number using the 1st and 3rd digit, second number using the 2nd and 4th digit. These will be the smallest two 2-digit numbers possible using these digits. Then return their sum
    pub fn minimum_sum(num: i32) -> i32 {
        let string = num.to_string();
        let mut char_vec: Vec<char> = string.chars().collect();
        char_vec.sort();
        let first_num: i32 = [char_vec[0], char_vec[2]].iter().collect::<String>().parse().unwrap();
        let second_num: i32 = [char_vec[1], char_vec[3]].iter().collect::<String>().parse().unwrap();
        return (first_num + second_num);
    }
}
