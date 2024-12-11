use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input
        .split_ascii_whitespace()
        .filter_map(|number| number.parse::<u64>().ok())
        .collect();
    let mut seen = HashMap::<(u64, usize), u64>::new();

    let result: Vec<u64> = stones
        .into_iter()
        .map(|stone| blink(stone, 0, 25, &mut seen))
        .collect();

    Some(result.into_iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input
        .split_ascii_whitespace()
        .filter_map(|number| number.parse::<u64>().ok())
        .collect();
    let mut seen = HashMap::<(u64, usize), u64>::new();

    let result: Vec<u64> = stones
        .into_iter()
        .map(|stone| blink(stone, 0, 75, &mut seen))
        .collect();

    Some(result.into_iter().sum())
}

fn blink(stone: u64, count: usize, to: usize, seen: &mut HashMap<(u64, usize), u64>) -> u64 {
    if count == to {
        return 1;
    }
    if let Some(stone_count) = seen.get(&(stone, count)) {
        return *stone_count;
    }
    let stone_count = if stone == 0 {
        blink(1, count + 1, to, seen)
    } else {
        let len = stone.checked_ilog10().unwrap_or(0) + 1;
        if len % 2 == 0 {
            let divisor = 10u64.pow(len / 2);
            let first = stone / divisor;
            let second = stone % divisor;
            blink(first, count + 1, to, seen) + blink(second, count + 1, to, seen)
        } else {
            blink(stone * 2024, count + 1, to, seen)
        }
    };
    seen.insert((stone, count), stone_count);
    stone_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u64> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u64> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
