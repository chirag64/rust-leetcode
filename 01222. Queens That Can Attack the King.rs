use std::cmp::max;
use std::cmp::min;

impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let mut queens_vec: Vec<Vec<bool>> = Vec::new();
        let mut attackers: Vec<Vec<i32>> = Vec::new();
        
        // Let all queens_vec be false
        for i in 0..8 {
            let mut temp_vec = Vec::new();
            for j in 0..8 {
                temp_vec.push(false);
            }
            queens_vec.push(temp_vec);
        }
        
        // Mark position of queens as true in queens_vec
        for queen in &queens {
            queens_vec[queen[0] as usize][queen[1] as usize] = true;
        }

        // Check if each queen is an attacker
        for queen in &queens {
            let mut is_queen_attacker = true;
            if queen[0] == king[0] {
                // Check if both are in same row
                for i in (min(queen[1], king[1]) + 1)..(max(queen[1], king[1])) {
                    if (is_queen_attacker) {
                        is_queen_attacker = !queens_vec[queen[0] as usize][i as usize];
                    }
                }
            } else if queen[1] == king[1] {
                // Check if both are in same column
                for i in (min(queen[0], king[0]) + 1)..(max(queen[0], king[0])) {
                    if (is_queen_attacker) {
                        is_queen_attacker = !queens_vec[i as usize][queen[1] as usize];
                    }
                }
            } else if (queen[0] - queen[1]) == (king[0] - king[1]) {
                // Check if both are in same diagonal from top-left to bottom-right
                let x_y_difference = queen[0] - queen[1];
                for i in (min(queen[0], king[0]) + 1)..(max(queen[0], king[0])) {
                    if (is_queen_attacker) {
                        is_queen_attacker = !queens_vec[i as usize][(i - x_y_difference) as usize];
                    }
                }
            } else if (queen[0] + queen[1] == (king[0] + king[1])) {
                // Check if both are in same diagonal from top-right to bottom-left
                let x_y_sum = queen[0] + queen[1];
                for i in (min(queen[0], king[0]) + 1)..(max(queen[0], king[0])) {
                    if (is_queen_attacker) {
                        is_queen_attacker = !queens_vec[i as usize][(x_y_sum - i) as usize];
                    }
                }
            } else {
                // Queen is certainly not an attacker as there's no clear line of sight
                is_queen_attacker = false;
            }
            
            if is_queen_attacker {
                attackers.push(queen.to_vec());
            }
        }
        // println!("{:?}", attackers);
        return attackers;
    }
}
