mod solution;

fn main() {
  let mut actual_output;
  let mut expected_output;
  
  actual_output = solution::Solution::distance_between_bus_stops(vec![1,2,3,4], 0, 1);
  expected_output = 2;
  assert!(actual_output == expected_output, "First test case failed!\nactual: {}, expected: {}", actual_output, expected_output);

  actual_output = solution::Solution::distance_between_bus_stops(vec![1,2,3,4], 0, 2);
  expected_output = 3;
  assert!(actual_output == expected_output, "Second test case failed!\nactual: {}, expected: {}", actual_output, expected_output);

  actual_output = solution::Solution::distance_between_bus_stops(vec![1,2,3,4], 0, 3);
  expected_output = 5;
  assert!(actual_output == expected_output, "Third test case failed!\nactual: {}, expected: {}", actual_output, expected_output);
}