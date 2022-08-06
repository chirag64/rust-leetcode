use std::collections::HashMap;

impl Solution {
    // Create a hashmap which contains the list of values and their total weights by looping through both the items vector. Then create a sorted vector of tuples (sorted by keys) from the hashmap. Finally, convert the vector of tuples to vector of vectors and return it
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        
        for item in &items1 {
            let count = map.entry(item[0]).or_insert(0);
            *count += item[1];
        }
        
        for item in &items2 {
            let count = map.entry(item[0]).or_insert(0);
            *count += item[1];
        }        
        
        let mut count_vec: Vec<_> = map.iter().collect();
        count_vec.sort_by(|a, b| a.0.cmp(b.0));
        
        let mut result = Vec::new();
        for tuple in count_vec {
            result.push(vec![*tuple.0, *tuple.1]);
        }
        
        return result;
    }
}
