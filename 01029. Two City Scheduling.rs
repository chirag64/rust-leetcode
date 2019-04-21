use std::cmp;

impl Solution {
    // We're cloning the input vector in a new sorted vector. The new vector is sorted by differences between the two costs. Once we've the new array, we send everyone in first half to city A and everyone in second half to city B
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut i = 0;
        let mut size = costs.len();
        let mut total_cost = 0;
        let mut costs_sorted = Vec::new();
        loop {
            //println!("i is {}", i);
            if (i == size) {
                break;
            }
            costs_sorted = Solution::insertion_sort(costs_sorted, ([costs[i][0], costs[i][1], costs[i][0] - costs[i][1]]).to_vec());
            i += 1;
        }
        //println!("{:?}", costs_sorted);
        i = 0;
        
        // Now we only need to loop through the vector (size / 2) times
        size /= 2;
        loop {
            if (i == size) {
                break;
            } else {
                total_cost += costs_sorted[i][0];
                total_cost += costs_sorted[i + size][1];
            }
            i += 1;
        }
        return total_cost;
    }
    
    // Implementing insertion sort to our own vector which clones the existing vector with a 3rd element in inner vector that carries the difference
    pub fn insertion_sort(mut costs_sorted: Vec<Vec<i32>>, new_cost: Vec<i32>) -> Vec<Vec<i32>>{
        let mut i = costs_sorted.len();costs_sorted.push(new_cost);
        loop {
            //println!("inner i is {}", i);
            if (i == 0) {
                break;
            }
            if costs_sorted[i][2] < costs_sorted[i - 1][2] {
                //mem::swap(&costs_sorted[i], &costs_sorted[i - 1]);
                costs_sorted.swap(i, i - 1);
            } else {
                break;
            }
            i -= 1;
        }
        return costs_sorted;
    }    
}
