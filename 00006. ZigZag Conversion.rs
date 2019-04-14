impl Solution {
    
  
  pub fn convert(s: String, num_rows: i32) -> String {
    // Convert string into char vector for easy get of individual chars
  let v: Vec<char> = s.chars().collect();

  // Counter for string
  let mut i: i32;

  // Counter for row
  let mut curr_row: i32 = 0;
  let size = s.len() as i32;

  // Initialize resulting char vector with . character. We'll return this as a string in the end
  let mut result: Vec<char> = vec!['.'; size as usize];

  // Counter for result char vector
  let mut result_i: i32 = 0;

  // So we run through 2 nested loops. Trick is we go through the outer loop num_rows number of times. Every time we go through this loop, we pick only the characters found in that row of resulting zigzag. Formula is (i (+ / -) curr_row) should be divisible by ((num_rows - 1) * 2)
  // e.g.: With the inputs of s = "PAYPALISHIRING", numRows = 3, when curr_row will be 0, we will only pick P,A,H,N
  // 3
  //    0, 4, 8, 12
  //    1, 3, 5, 7, 9, 11, 13
  //    2, 6, 10

  // 4    
  //   0, 6, 12
  //   1, 5, 7, 11, 13
  //   2, 4, 8, 10
  //   3, 9
     
  // 5    
  //   0, 8
  //   1, 7, 9
  //   2, 6, 10
  //   3, 5, 11, 13
  //   4, 12

  // TODO: Solve in single loop:
  // f(3, i, 14) = result:
  //   i: 0, 1, 2,  3, 4, 5, 6,  7, 8, 9, 10, 11, 12, 13
  //   result: 0, 4, 11, 5, 1, 6, 12, 7, 2, 8, 13, 9,   3, 10
        
  //   i + 1: 1, 2,  3, 4, 5, 6,  7, 8, 9, 10, 11, 12, 13
  //   result + 1: 1, 5, 12, 6, 2, 7, 13, 8, 3,  9, 14, 10,  4, 11
  loop {
      if (curr_row == num_rows) {
        break;
      }
      i = 0;
          loop {
            if (i == size as i32) {
              break;
            }

            if ((num_rows == 1) || ((i - curr_row) % ((num_rows - 1) * 2) == 0) || ((i + curr_row) % ((num_rows - 1) * 2) == 0)) {
              result[result_i as usize] = v[i as usize];
              result_i += 1;
            }
            i += 1;
          }
        curr_row += 1;
      }
      // println!("{:?}", result);
      return result.into_iter().collect();
  }
}
