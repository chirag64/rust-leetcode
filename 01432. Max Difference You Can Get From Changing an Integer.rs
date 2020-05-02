impl Solution {
    // Convert num to string. Then calculate a by replacing all instances of the first non-9 number from left to a 9. Then calculate b by: checking if first digit is not 1, then replace all instances of that digit with a 1. If it is a one, replace all instances of the first non-0 digit with a 0. Return difference between a and b
    pub fn max_diff(num: i32) -> i32 {
        let str = num.to_string();
        let len = str.len();
        let mut a = -1;
        let mut b = -1;
        for i in 0..len {
            if (str.chars().nth(i).unwrap() != '9') {
                a = str.replace(str.chars().nth(i).unwrap(), "9").parse::<i32>().unwrap();
                break;
            }
        }
        if a == -1 {
            a = num;
        }
        
        for i in 0..len {
            if (i == 0 && str.chars().nth(i).unwrap() != '1') {
                b = str.replace(str.chars().nth(i).unwrap(), "1").parse::<i32>().unwrap();
                break;
            } else if (i != 0 && str.chars().nth(i).unwrap() != '0' && str.chars().nth(i).unwrap() != str.chars().nth(0).unwrap()) {
                b = str.replace(str.chars().nth(i).unwrap(), "0").parse::<i32>().unwrap();
                break;
            }
        }
        if b == -1 {
            b = num;
        }
        return a - b;
    }
}
