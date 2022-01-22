impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        // Loop through corridor and calculate number of seats. If there are only two seats, return 1, if the total seats are not even-numbered or no seats exist, there is no possible solution because they can't be divided into groups of 2. If seats are even, maintain a vector which keeps track of differences of every odd and even numbered seat (excluding first one). At the end, multiply all these numbers in the vector, because each of these numbers represent the combinations possible of placing a corridor between those pairs of seats, and the final solution would be a product of all these combinations
        let mut seats_count = 0;
        let mut second_seat_position = 0;
        let mut third_seat_position = 0;
        let mut vec = vec![];
        let mut output: i64 = 1;
        for (index, item) in corridor.chars().enumerate() {
            if item == 'S' {
                seats_count += 1;
                if seats_count % 2 == 0 {
                    second_seat_position = index;
                }
                else if (second_seat_position != 0) && (seats_count % 2 == 1) {
                    third_seat_position = index;
                    vec.push(third_seat_position - second_seat_position);                    
                }
            }
        }
        
        if seats_count == 2 {
            return 1;
        }
        if (seats_count % 2) != 0 || seats_count == 0 {
            return 0;
        }
        
        for i in vec {
            output *= (i as i64);
            output %= 1000000007;
        }
        
        return output as i32;
    }
}
