use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next_direction(&mut self) {
        *self = match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    let location = map
        .iter()
        .enumerate()
        .filter_map(|(row_idx, row)| {
            row.iter()
                .position(|&value| value == '^')
                .map(|col_idx| (row_idx, col_idx))
        })
        .next();

    let mut current_direction = Direction::Up;
    let mut current_position: (usize, usize) = location.unwrap();
    let mut count_steps = 0;
    let mut traveled: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];

    loop {
        //Take Step
        match current_direction {
            Direction::Up => current_position.0 = current_position.0.saturating_sub(1),
            Direction::Right => current_position.1 = current_position.1.saturating_add(1),
            Direction::Down => current_position.0 = current_position.0.saturating_add(1),
            Direction::Left => current_position.1 = current_position.1.saturating_sub(1),
        };
        //Check to see if we exited
        if current_position.0 >= map.len()
            || current_position.0 == usize::MAX
            || current_position.1 >= map[0].len()
            || current_position.1 == usize::MAX
        {
            break;
        }
        //We're still in the map, let's count our step
        if !traveled[current_position.0][current_position.1] {
            count_steps += 1;
            traveled[current_position.0][current_position.1] = true;
        }

        //Check if we need to rotate
        match current_direction {
            Direction::Up => {
                if current_position.0 == 0 {
                    continue;
                }
                if map[current_position.0 - 1][current_position.1] == '#' {
                    current_direction.next_direction();
                }
            }
            Direction::Right => {
                if current_position.1 == (map[0].len() - 1) {
                    continue;
                }
                if map[current_position.0][current_position.1 + 1] == '#' {
                    current_direction.next_direction();
                }
            }
            Direction::Down => {
                if current_position.0 == (map.len() - 1) {
                    continue;
                }
                if map[current_position.0 + 1][current_position.1] == '#' {
                    current_direction.next_direction();
                }
            }
            Direction::Left => {
                if current_position.1 == 0 {
                    continue;
                }
                if map[current_position.0][current_position.1 - 1] == '#' {
                    current_direction.next_direction();
                }
            }
        }
    }
    Some(count_steps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    let location: Option<(usize, usize)> = map
        .iter()
        .enumerate()
        .filter_map(|(row_idx, row)| {
            row.iter()
                .position(|&value| value == '^')
                .map(|col_idx| (row_idx, col_idx))
        })
        .next();

    let mut visited = HashSet::new();
    let mut queue = VecDeque::from(vec![(location.unwrap(), Direction::Up)]);

    while let Some((current_pos, current_dir)) = queue.pop_front() {
        visited.insert(current_pos);
        let (next_pos, next_dir) = execute_step(&map, current_pos, current_dir);

        if next_pos == current_pos && next_dir == current_dir {
            break;
        }

        queue.push_back((next_pos, next_dir));
    }

    let infinite_maps: Vec<Vec<Vec<char>>> = visited
        .into_iter()
        .filter_map(|position| {
            let mut new_map = map.clone();
            if new_map[position.0][position.1] == '.' {
                new_map[position.0][position.1] = '#';

                if is_infinite(&new_map, location.unwrap()) {
                    Some(new_map)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let return_count = infinite_maps.len() as u32;
    Some(return_count)
}

pub fn is_infinite(map: &[Vec<char>], (row, col): (usize, usize)) -> bool {
    let mut visited_states = HashSet::new();
    let mut current_pos = (row, col);
    let mut current_dir = Direction::Up;

    for _ in 0..8000 {
        let current_state: ((usize, usize), Direction) = (current_pos, current_dir);

        if !visited_states.insert(current_state) {
            return true; // Cycle detected
        }

        let (next_pos, next_dir) = execute_step(map, current_pos, current_dir);

        if next_pos == current_pos && next_dir == current_dir {
            return false;
        }

        current_pos = next_pos;
        current_dir = next_dir;
    }
    false
}

pub fn execute_step(
    map: &[Vec<char>],
    (row, col): (usize, usize),
    direction: Direction,
) -> ((usize, usize), Direction) {
    let (dx, dy) = match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    };

    let new_pos = ((row as isize + dx) as usize, (col as isize + dy) as usize);

    if new_pos.0 != usize::MAX
        && new_pos.0 < map.len()
        && new_pos.1 != usize::MAX
        && new_pos.1 < map[0].len()
    {
        if map[new_pos.0][new_pos.1] == '#' {
            let mut new_direction = direction;
            new_direction.next_direction();
            return ((row, col), new_direction);
        }

        return (new_pos, direction);
    }
    ((row, col), direction)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
