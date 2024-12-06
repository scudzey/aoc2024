advent_of_code::solution!(6);

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
            Direction::Left => Direction::Up
        };
    }


}
pub fn part_one(input: &str) -> Option<u32> {

    let map: Vec<Vec<char>> = input
        .lines()
        .map(|row| row.chars().collect())
        .collect();

    let location = map.iter().enumerate()
                                                        .filter_map(|(row_idx, row)|
                                                            row.iter().position(|&value| value == '^')
                                                                .map(|col_idx| (row_idx, col_idx))
                                                        )
                                                        .next();
    match location {
        Some((row,col)) => println!("Found starting point at pos ({},{})", row, col),
        None => println!("not found"),
    }

    let mut current_direction = Direction::Up;
    let mut current_position: (usize, usize) = location.unwrap();
    let mut count_steps = 0;
    let mut traveled: Vec<Vec<bool>> = vec![vec![false;map[0].len()]; map.len()];

    loop {
        //Take Step
        match current_direction {
            Direction::Up => current_position.0 = current_position.0.saturating_sub(1),
            Direction::Right => current_position.1 =current_position.1.saturating_add(1),
            Direction::Down => current_position.0 = current_position.0.saturating_add(1),
            Direction::Left => current_position.1 = current_position.1.saturating_sub(1),
        };
        //Check to see if we exited
        if current_position.0 >= map.len() || current_position.0 == usize::MAX ||
          current_position.1 >= map[0].len() || current_position.1 == usize::MAX {
            break;
        }
        //We're still in the map, let's count our step
        if !traveled[current_position.0][current_position.1] {
            count_steps  += 1;
            traveled[current_position.0][current_position.1] = true;
        }

        //Check if we need to rotate
        match current_direction {
            Direction::Up => {
                if current_position.0 == 0 {
                    continue;
                }
                if map[current_position.0-1][current_position.1] == '#'{
                    current_direction.next_direction();
                }
            },
            Direction::Right => {
                if current_position.1 == (map[0].len() - 1){
                    continue;
                }
                if map[current_position.0][current_position.1+1] == '#'{
                    current_direction.next_direction();
                }
            }
            Direction::Down => {
                if current_position.0 == (map.len() -1) {
                    continue;
                }
                if map[current_position.0+1][current_position.1] == '#'{
                    current_direction.next_direction();
                }
            }
            Direction::Left => {
                if current_position.1 == 0 {
                    continue;
                }
                if map[current_position.0][current_position.1-1] == '#'{
                    current_direction.next_direction();
                }
            }
        }
    }
    Some(count_steps)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
