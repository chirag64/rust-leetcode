pub struct Solution;

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let total_stops = distance.len() as i32;
        let mut i = start;
        let mut distance_one: i32 = 0;
        let mut distance_two: i32 = 0;
        
        // Calculate distance from start to destination and store it in distance_one
        loop {
            if (i == destination) {
                break;
            }
            distance_one += distance[i as usize];
            i = (i + 1) % total_stops;
        }
        
        // Calculate distance from destination to start and store it in distance_two
        loop {
            if (i == start) {
                break;
            }
            distance_two += distance[i as usize];
            i = (i + 1) % total_stops;
        }
        
        // Return the value that is smallest among the two
        return std::cmp::min(distance_one, distance_two);
    }
}
