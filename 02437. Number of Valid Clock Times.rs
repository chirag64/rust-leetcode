impl Solution {
    // Create 4 vars: h1, h2, m1, m2 & initialize 'output' by 1. Then based on every possible combination of whether h1 & h2 are present, multiply the 'output' accordingly. Repeat the same with m1 & m2
    pub fn count_time(time: String) -> i32 {
        let mut output = 1;
        let h1 = time.chars().nth(0).unwrap();
        let h2 = time.chars().nth(1).unwrap();
        let m1 = time.chars().nth(3).unwrap();
        let m2 = time.chars().nth(4).unwrap();
        
        // ??:xx
        if h1 == '?' && h2 == '?' {
            output *= 24;
        // ?3:xx
        } else if (h1 == '?' && (h2 as i32 - '0' as i32) < 4) {
            output *= 3;
        // ?9:xx
        } else if h1 == '?' {
            output *= 2;
        // 1?:xx
        } else if ((h1 as i32 - '0' as i32) < 2) && (h2 == '?') {
            output *= 10;
        // 2?:xx
        } else if ((h1 as i32 - '0' as i32) == 2) && (h2 == '?') {
            output *= 4;
        }
        
        // xx:??
        if m1 == '?' && m2 == '?' {
            output *= 60;
        // xx:?9
        } else if m1 == '?' {
            output *= 6;
        // xx:5?
        } else if m2 == '?' {
            output *= 10;
        }
        
        return output;
    }
}
