use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let pattern = r"mul\([0-9]+,[0-9]+\)";

    let re = Regex::new(pattern).unwrap();

    let matches: Vec<String> = re.find_iter(&input)
                                .map(|mat| {
                                        let match_str = mat.as_str();
                                        match_str.to_string()
                                }).collect();


    return Some(process_list(&matches));
}

pub fn part_two(input: &str) -> Option<u32> {
    let pattern = r"(mul\(\d+,\d+\)|do\(\)|don't\(\))";

    let re = Regex::new(pattern).unwrap();

    let matches: Vec<String> = re.find_iter(&input)
                                .map(|mat| {
                                        let match_str = mat.as_str();
                                        match_str.to_string()
                                }).collect();


    return Some(process_list(&matches));
}

fn process_list(commands: &Vec<String>) -> u32 {
    let mut result: u32 = 0;
    let mut execute = true;
    for item in commands{
        if item.contains("mul(") && execute {
            result += calculate_mul(item[4..item.len()-1].to_string());
        }

        if item.contains("do(") {
            execute = true;
        }

        if item.contains("don't(") {
            execute = false
        }

    }
    return result
}

fn calculate_mul(input_text: String) -> u32 {
    let nums: Vec<u32> = input_text.split(",").filter_map(|item| item.parse::<u32>().ok()).collect();
    return nums[0] * nums[1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
