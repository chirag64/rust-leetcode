impl Solution {
    // Sort vectors and loop through them in reverse order. If trainer is better or as good as player, count them and move to next (or previous) trainer/player. If trainer isn't as good as player, skip the player and move to the next (or previous)
    pub fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
        players.sort();
        trainers.sort();
        
        let mut result = 0;
        let mut i = (trainers.len() as i32) - 1;
        let mut j = (players.len() as i32) - 1;
        
        loop {
            if i == -1 || j == -1 {
                break;
            } else if trainers[i as usize] >= players[j as usize] {
                result += 1;
                i -= 1;
                j -= 1;
            } else {
                j -= 1;
            }
        }
        return result;
    }
}
