// Create a simple object of 3 ints that keep a track of remaining parking positions for big, medium & small cars
struct ParkingSystem {
    bigCars: i32,
    mediumCars: i32,
    smallCars: i32
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {

    fn new(big: i32, medium: i32, small: i32) -> Self {
        return ParkingSystem {
            bigCars: big,
            mediumCars: medium,
            smallCars: small
        }
    }
    
    fn add_car(&mut self, car_type: i32) -> bool {
        if car_type == 1 {
            if self.bigCars == 0 {
                return false;
            } else {
                self.bigCars -= 1;
                return true;
            }
        }
        if car_type == 2 {
            if self.mediumCars == 0 {
                return false;
            } else {
                self.mediumCars -= 1;
                return true;
            }
        }
        if car_type == 3 {
            if self.smallCars == 0 {
                return false;
            } else {
                self.smallCars -= 1;
                return true;
            }
        }
        return false;
    }
}

/**
 * Your ParkingSystem object will be instantiated and called as such:
 * let obj = ParkingSystem::new(big, medium, small);
 * let ret_1: bool = obj.add_car(carType);
 */
