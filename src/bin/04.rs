use std::fmt;

advent_of_code::solution!(4);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    fn to_offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (-1, 1),
            Direction::DownLeft => (1, -1),
            Direction::DownRight => (1, 1),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let direction_str = match self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
            Direction::UpLeft => "UpLeft",
            Direction::UpRight => "UpRight",
            Direction::DownLeft => "DownLeft",
            Direction::DownRight => "DownRight",
        };
        write!(f, "{}", direction_str)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let board: Vec<Vec<u8>> = input
                                .lines()
                                .map(|row|
                                                row.bytes().collect()
                                )
                                .collect();

    let mut count: u32 = 0;
    let word = "XMAS";
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            count += backtrack_part_1( word, &board, i, j, None)
        }
    }
    return Some(count)
}

pub fn backtrack_part_1(
    word: &str,
    board : &[Vec<u8>],
    i : usize, j : usize,
    direction: Option<Direction>
) -> u32 {
    if word.len() == 0 {
        return 1
    }
    if i >= board.len() || j >= board[i].len() || board[i][j] != word.as_bytes()[0] {
        return 0
    }

    let next_guess: &str = &word[1..];

    match direction {
        Some(val) => {
            let next_offset = val.to_offset();
            return backtrack_part_1(next_guess, board, i.wrapping_add_signed(next_offset.0), j.wrapping_add_signed(next_offset.1), Some(val))
        }
        None => {
            return backtrack_part_1(next_guess, board, i+1, j, Some(Direction::Down)) + //down
                    backtrack_part_1(next_guess, board, i, j+1, Some(Direction::Right)) + //right
                    backtrack_part_1(next_guess, board, i.wrapping_sub(1), j, Some(Direction::Up)) + //up
                    backtrack_part_1(next_guess, board, i, j.wrapping_sub(1), Some(Direction::Left)) + //left
                    backtrack_part_1(next_guess, board, i+1, j+1, Some(Direction::DownRight)) + //down and right
                    backtrack_part_1(next_guess, board, i.wrapping_sub(1), j.wrapping_sub(1), Some(Direction::UpLeft)) + //up and left
                    backtrack_part_1(next_guess, board, i+1, j.wrapping_sub(1), Some(Direction::DownLeft)) + //down and left
                    backtrack_part_1(next_guess, board, i.wrapping_sub(1), j+1, Some(Direction::UpRight)) // up and right
        }
    }

}

pub fn part_two(input: &str) -> Option<u32> {
    let board: Vec<Vec<u8>> = input
                                .lines()
                                .map(|row|
                                                row.bytes().collect()
                                )
                                .collect();
    let combos = vec![vec![vec!['M','.','M'], vec!['.', 'A','.'], vec!['S', '.', 'S']],
                                           vec![vec!['M','.','S'], vec!['.', 'A','.'], vec!['M', '.', 'S']],
                                           vec![vec!['S','.','M'], vec!['.', 'A','.'], vec!['S', '.', 'M']],
                                           vec![vec!['S','.','S'], vec!['.', 'A','.'], vec!['M', '.', 'M']]];

    let mut count: u32 = 0;
    for i in  0..(board.len()-2) {
        for j in 0..(board[0].len()-2) {


            for combo in &combos {
                let mut found = true;
                for k in 0..combo.len() {
                    for l in 0..combo[0].len() {
                        if combo[k][l] == '.' {
                            continue;
                        }

                        if combo[k][l] != board[i+k][j+l] as char{
                            found = false
                        }

                    }
                }
                if found {
                    count += 1;
                }
            }


        }
    }

    return Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
