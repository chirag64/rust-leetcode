use std::collections::HashMap;

impl Solution {
    // Create a hashmap with key as group size and value as vector indexes with that group size. Then loop through the hashmap and keep adding them 'n' number at a time in the output vector, n being the key of the hashmap
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut groups:HashMap<i32, Vec<i32>> = HashMap::new();
        let mut output: Vec<Vec<i32>> = [].to_vec();
        
        for (index, item) in group_sizes.iter().enumerate() {
            let index_i32 = index as i32;
            if !groups.contains_key(item) {
                let vec = [index_i32].to_vec();
                groups.insert(*item, vec);
            } else {
                groups.get_mut(item).unwrap().push(index_i32);
            }
        }
        for (key, value) in &groups {
            let mut counter = 1;
            let mut temp_arr: Vec<i32> = [].to_vec();
            for i in 0..value.len() {
                temp_arr.push(value[i]);
                if counter % key == 0 {
                    output.push(temp_arr);
                    temp_arr = [].to_vec();
                }
                counter += 1;
            }
        }
        return output;
    }
}
