advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let problems: Vec<(u64, Vec<u64>)> = input
        .lines()
        .filter_map(|row| {
            row.split_once(':').and_then(|(before, after)| {
                before.trim().parse::<u64>().ok().map(|predicate| {
                    let numbers: Vec<u64> = after
                        .split_whitespace()
                        .filter_map(|num| num.parse::<u64>().ok())
                        .collect();
                    (predicate, numbers)
                })
            })
        })
        .collect();

    let mut result = 0;
    for problem in problems {
        if can_reach_target(&problem.1, problem.0, 0) {
            result += problem.0;
        }
    }
    Some(result)
}

pub fn can_reach_target(nums: &[u64], target: u64, current_value: u64) -> bool {
    if nums.is_empty() {
        return current_value == target;
    }

    let next_num: u64 = nums[0];
    let rest = &nums[1..];

    can_reach_target(rest, target, current_value + next_num)
        || can_reach_target(rest, target, current_value * next_num)
}

pub fn part_two(input: &str) -> Option<u64> {
    let problems: Vec<(u64, Vec<u64>)> = input
        .lines()
        .filter_map(|row| {
            row.split_once(':').and_then(|(before, after)| {
                before.trim().parse::<u64>().ok().map(|predicate| {
                    let numbers: Vec<u64> = after
                        .split_whitespace()
                        .filter_map(|num| num.parse::<u64>().ok())
                        .collect();
                    (predicate, numbers)
                })
            })
        })
        .collect();

    let mut result = 0;
    for problem in problems {
        if can_reach_target_pt2(&problem.1[1..], problem.0, problem.1[0]) {
            result += problem.0;
        }
    }
    Some(result)
}

pub fn can_reach_target_pt2(nums: &[u64], target: u64, current_value: u64) -> bool {
    if nums.is_empty() {
        return current_value == target;
    }

    let next_num: u64 = nums[0];
    let rest = &nums[1..];

    can_reach_target_pt2(rest, target, current_value + next_num)
        || can_reach_target_pt2(rest, target, current_value * next_num)
        || can_reach_target_pt2(rest, target, concat_numbers(current_value, next_num))
}

pub fn concat_numbers(num1: u64, num2: u64) -> u64 {
    format!("{}{}", num1, num2).parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
