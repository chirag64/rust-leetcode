// Create a simple struct object with a single property: seats which is a vector of bools. During init, all values are false since no reservations are made. When a reservation is made, loop through the vector, find the first false value and change it to true to reserve it. When a seat is unreserved, directly go to that vector index and change its value to false
struct SeatManager {
    seats: Vec<bool>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {

    fn new(n: i32) -> Self {
        return SeatManager {
            seats: vec![false; n as usize]
        }        
    }
    
    fn reserve(&mut self) -> i32 {
        for (idx, i) in self.seats.iter_mut().enumerate() {
            if *i == false {
                *i = true;
                return (idx + 1) as i32;
            }
        }
        return 0;
    }
    
    fn unreserve(&mut self, seat_number: i32) {
        self.seats[(seat_number - 1) as usize] = false;
    }
}

/**
 * Your SeatManager object will be instantiated and called as such:
 * let obj = SeatManager::new(n);
 * let ret_1: i32 = obj.reserve();
 * obj.unreserve(seatNumber);
 */
