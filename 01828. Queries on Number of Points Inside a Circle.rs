impl Solution {
    // Loop through the queries (circles). Then loop through the points. If the distance between each point and the circle's center is less than or equal to the radius of the circle, count the point to be within the circle
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        for query in queries {
            let mut points_in_current_circle = 0;
            for point in &points {
                let distance = (((i32::pow((query[0] - point[0]), 2) + i32::pow((query[1] - point[1]), 2)) as f64).sqrt());
                if distance <= (query[2] as f64) {
                    points_in_current_circle += 1;
                }
            }
            result.push(points_in_current_circle);
        }
        return result;
    }
}
