impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        let size = text.len();
        let mut i = 1;
        let mut j = size - 1;
        let mut firstStringStart = 0;
        let mut secondStringEnd = j + 1;
        let mut count = 0;
        
        // Maintain 2 markers from both sides & keep forming strings from both sides and check if they match. If they do, move the pointers and add 2 to the count. At the end, add a count of 1 if there is some string left
        loop {
            if (i > j) {
                break;
            }
            let firstString = &text[firstStringStart..i];
            let secondString = &text[j..secondStringEnd];
            println!("firstStringStart: {}, i: {}, First String: {}, second string: {}", firstStringStart, i, firstString, secondString);
            if (firstString == secondString) {
                println!("String match found: {}", firstString);
                count += 2;
                firstStringStart = i;
                secondStringEnd = j;
            }
            i += 1;
            j -= 1;
        }
        println!("firstStringStart: {}, i: {}, j: {}, substring: {}", firstStringStart, i, j, &text[firstStringStart..i]);
        if ((i - 1) == j || (i - firstStringStart) > 1) {
            count += 1;
        }
        return count;
    }
}
