impl Solution {
    // Maintain 2 markers (i & j) that mark start and end of substring and are of size 3. If substring's last character is #, use first 2 chars as index to find corresponding letter & move both markers by 3. If substring's last character is not #, use only first char as index to find corresponding letter & move both markers by 1.
    pub fn freq_alphabets(s: String) -> String {
        let mut result = String::new();
        let len = s.len();
        let mut i = 0;
        let mut j = (i + 3);
        let num_to_alphabet = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
        let mut last_char: char;
        loop {
            last_char = s.chars().nth(j - 1).unwrap();
            if (last_char == '#') {
                result.push(num_to_alphabet[(&s[i..(j - 1)].parse::<i32>().unwrap() - 1) as usize]);
                i += 3;
                j += 3;
            } else {
                result.push(num_to_alphabet[(&s[i..(i + 1)].parse::<i32>().unwrap() - 1) as usize]);
                i += 1;
                j += 1;
            }
            if (j >= len) {
                if (i >= len) {
                    break;
                }
                j = len;
            }
        }
        return result;
    }
}
