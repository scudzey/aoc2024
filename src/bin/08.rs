use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut antenna_map: HashMap<char, Vec<(u32, u32)>> = HashMap::new();

    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars().enumerate().for_each(|(col, item)| {
                if item != '.' {
                    let antenna = antenna_map.entry(item).or_default();
                    antenna.push((row as u32, col as u32));
                }
            });
            line.chars().collect()
        })
        .collect();

    let mut location_set: HashSet<(u32, u32)> = HashSet::new();

    for (_, locations) in antenna_map {
        for (idx, position1) in locations.iter().enumerate() {
            for position2 in locations.iter().skip(idx + 1) {
                let distance: (i32, i32) = (
                    (position1.0 as i32 - position2.0 as i32),
                    (position1.1 as i32 - position2.1 as i32),
                );

                let antipol1 = (
                    (position1.0 as i32 + distance.0),
                    (position1.1 as i32 + distance.1),
                );
                let antipol2 = (
                    (position2.0 as i32 - distance.0),
                    (position2.1 as i32 - distance.1),
                );

                let row_size = &grid.len();
                let col_size = &grid[0].len();

                if antipol1.0 >= 0
                    && antipol1.0 < *row_size as i32
                    && antipol1.1 >= 0
                    && antipol1.1 < *col_size as i32
                {
                    location_set.insert((antipol1.0 as u32, antipol1.1 as u32));
                }

                if antipol2.0 >= 0
                    && antipol2.0 < *row_size as i32
                    && antipol2.1 >= 0
                    && antipol2.1 < *col_size as i32
                {
                    location_set.insert((antipol2.0 as u32, antipol2.1 as u32));
                }
            }
        }
    }

    Some(location_set.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut antenna_map: HashMap<char, Vec<(u32, u32)>> = HashMap::new();

    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars().enumerate().for_each(|(col, item)| {
                if item != '.' {
                    let antenna = antenna_map.entry(item).or_default();
                    antenna.push((row as u32, col as u32));
                }
            });
            line.chars().collect()
        })
        .collect();

    let mut location_set: HashSet<(u32, u32)> = HashSet::new();

    for (_, locations) in antenna_map {
        for (idx, position1) in locations.iter().enumerate() {
            location_set.insert(*position1);
            for position2 in locations.iter().skip(idx + 1) {
                location_set.insert(*position2);
                let distance: (i32, i32) = (
                    (position1.0 as i32 - position2.0 as i32),
                    (position1.1 as i32 - position2.1 as i32),
                );

                let mut antipol1 = (
                    (position1.0 as i32 + distance.0),
                    (position1.1 as i32 + distance.1),
                );
                let mut antipol2 = (
                    (position2.0 as i32 - distance.0),
                    (position2.1 as i32 - distance.1),
                );

                let row_size = &grid.len();
                let col_size = &grid[0].len();

                while antipol1.0 >= 0
                    && antipol1.0 < *row_size as i32
                    && antipol1.1 >= 0
                    && antipol1.1 < *col_size as i32
                {
                    location_set.insert((antipol1.0 as u32, antipol1.1 as u32));
                    antipol1 = ((antipol1.0 + distance.0), (antipol1.1 + distance.1));
                }

                while antipol2.0 >= 0
                    && antipol2.0 < *row_size as i32
                    && antipol2.1 >= 0
                    && antipol2.1 < *col_size as i32
                {
                    location_set.insert((antipol2.0 as u32, antipol2.1 as u32));
                    antipol2 = ((antipol2.0 - distance.0), (antipol2.1 - distance.1));
                }
            }
        }
    }

    Some(location_set.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
