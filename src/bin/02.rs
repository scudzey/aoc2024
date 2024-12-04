advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let report_list: Vec<Vec<u32>> = input
                                            .lines()
                                            .filter(|line| !line.trim().is_empty())
                                            .map(|line| {
                                                line.split(' ')
                                                    .filter_map(|field| field.trim().parse::<u32>().ok())
                                                    .collect()
                                            }).collect();

    return Some(calc_safe(&report_list));
}

pub fn part_two(input: &str) -> Option<u32> {
    let report_list: Vec<Vec<u32>> = input
                                            .lines()
                                            .filter(|line| !line.trim().is_empty())
                                            .map(|line| {
                                                line.split(' ')
                                                    .filter_map(|field| field.trim().parse::<u32>().ok())
                                                    .collect()
                                            }).collect();

    return Some(calc_safe_buffer(&report_list));

}

pub fn calc_safe(reports: &Vec<Vec<u32>>) -> u32 {
    let mut safe_count = 0;
    for report in reports {
        if check_report(&report) {
            safe_count += 1;
        }
    }

    return safe_count
}

pub fn calc_safe_buffer(reports: &Vec<Vec<u32>>) -> u32 {
    let mut safe_count = 0;
    for report in reports {
        if check_report_safe(&report) {
            safe_count += 1;
        }
    }

    return safe_count
}

pub fn check_report(report: &Vec<u32>) -> bool{
    let mut is_increasing = false;
    if report[0] < report [1] {
        is_increasing = true;
    }

    if !is_valid(report[0], report[1], is_increasing) {//((report[0] - report[1]).abs() > 3) or ((report[0] - report[1]).abs() < 1) {
        return false
    }

    for index in 1..(report.len()-1) {
        if !is_valid(report[index], report[index+1], is_increasing) {
            return false
        }
    }

    return true;
}

pub fn is_valid(num1: u32, num2:u32, is_increasing: bool) -> bool {
    if (u32::abs_diff(num1, num2) > 3) || (u32::abs_diff(num1, num2) < 1) {
        return false;
    }

    if (is_increasing) && num1 > num2 {
        return false;
    } else if !is_increasing && num1 < num2 {
        return false;
    }

    return true;
}

pub fn check_report_safe(report: &Vec<u32>) -> bool{

    let mut is_increasing = false;
    let mut is_invalid_count = 0;
    if report[0] < report [1] {
        is_increasing = true;
    }

    if !is_valid(report[0], report[1], is_increasing) {
        is_invalid_count += 1
    }



    for index in 1..(report.len()-1) {
        if !is_valid(report[index], report[index+1], is_increasing) {
            is_invalid_count += 1
        }
    }

    if is_invalid_count > 1 {
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
