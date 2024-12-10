use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let board: Vec<Vec<u32>> = input
        .lines()
        .map(|row| row.chars().filter_map(|ch| ch.to_digit(10)).collect())
        .collect();

    let mut sum = 0;
    let mut visited = HashSet::new();
    for row in 0..board.len() {
        for col in 0..board[0].len() {
            if board[row][col] == 0 {
                visited.clear();
                let paths = find_paths(&board, row, col, u32::MAX, &mut visited);
                sum += paths;
            }
        }
    }
    Some(sum)
}

pub fn find_paths(
    board: &Vec<Vec<u32>>,
    row: usize,
    col: usize,
    previous_val: u32,
    visited: &mut HashSet<(usize, usize)>,
) -> u32 {
    if row >= board.len() || col >= board[0].len() {
        return 0;
    }

    if previous_val != u32::MAX && previous_val + 1 != board[row][col] {
        return 0;
    }

    if board[row][col] == 9 {
        if visited.insert((row, col)) {
            return 1;
        } else {
            return 0;
        }
    }

    find_paths(board, row + 1, col, board[row][col], visited)
        + find_paths(board, row.wrapping_sub(1), col, board[row][col], visited)
        + find_paths(board, row, col + 1, board[row][col], visited)
        + find_paths(board, row, col.wrapping_sub(1), board[row][col], visited)
}

pub fn part_two(input: &str) -> Option<u32> {
    let board: Vec<Vec<u32>> = input
        .lines()
        .map(|row| row.chars().filter_map(|ch| ch.to_digit(10)).collect())
        .collect();

    let mut sum = 0;
    for row in 0..board.len() {
        for col in 0..board[0].len() {
            if board[row][col] == 0 {
                let paths = find_paths_pt2(&board, row, col, u32::MAX);
                sum += paths;
            }
        }
    }
    Some(sum)
}

pub fn find_paths_pt2(board: &Vec<Vec<u32>>, row: usize, col: usize, previous_val: u32) -> u32 {
    if row >= board.len() || col >= board[0].len() {
        return 0;
    }

    if previous_val != u32::MAX && previous_val + 1 != board[row][col] {
        return 0;
    }

    if board[row][col] == 9 {
        return 1;
    }

    find_paths_pt2(board, row + 1, col, board[row][col])
        + find_paths_pt2(board, row.wrapping_sub(1), col, board[row][col])
        + find_paths_pt2(board, row, col + 1, board[row][col])
        + find_paths_pt2(board, row, col.wrapping_sub(1), board[row][col])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
