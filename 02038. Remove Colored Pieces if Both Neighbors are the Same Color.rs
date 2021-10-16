use std::cmp::max;

impl Solution {
    // Loop through colors vector to find out how many total pieces A & B can remove. Every time we see 'n' continous pieces, the player can remove max(0, (n - 2)) pieces from it. After loop is complete, do a last check of the currently tracked piece since those last continous pieces should also be evaluated
    pub fn winner_of_game(colors: String) -> bool {
        let mut pieces_a_can_remove = 0;
        let mut pieces_b_can_remove = 0;
        let mut current_piece = 'C';
        let mut current_piece_count = 0;
        
        for c in colors.chars() {
            if c == current_piece {
                current_piece_count += 1;
            } else {
                if current_piece == 'A' {
                    pieces_a_can_remove += max(0, current_piece_count - 2);
                } else if current_piece == 'B' {
                    pieces_b_can_remove += max(0, current_piece_count - 2);
                }
                current_piece = c;
                current_piece_count = 1;
            }
        }
        
        if current_piece == 'A' {
            pieces_a_can_remove += max(0, current_piece_count - 2);
        } else if current_piece == 'B' {
            pieces_b_can_remove += max(0, current_piece_count - 2);
        }
        
        return pieces_a_can_remove > pieces_b_can_remove;
    }
}
