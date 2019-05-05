impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        // Check slope of (point1, point2) & (point1, point3). If they match, they're in a straight line and it is not a valid boomerang.
        // Rust surprisingly won't complain if denominator is 0 while dividing
        /*if (points[1][0] == points[0][0] || points[2][0] == points[0][0]) {
            println!("Same denominators");
        }*/
        
        // If any 2 points are the same, it is not a valid boomerang
        if (points[0][0] == points[1][0] && points[0][1] == points[1][1]) ||
           (points[1][0] == points[2][0] && points[1][1] == points[2][1]) ||
           (points[2][0] == points[0][0] && points[2][1] == points[0][1]) {
               return false;
        }
        
        let slope1 = (points[1][1] as f64 - points[0][1] as f64) / (points[1][0] as f64 - points[0][0] as f64);
        let slope2 = (points[2][1] as f64 - points[0][1] as f64) / (points[2][0] as f64 - points[0][0] as f64);
        
        //println!("{}, {}", slope1, slope2);
        return slope1 != slope2;
    }
}
