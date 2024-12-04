use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut rdr = csv::ReaderBuilder::new()
                                    .delimiter(b';')
                                    .has_headers(false)
                                    .from_reader(input.as_bytes());

    let mut list1:Vec<u32> = Vec::new();
    let mut list2:Vec<u32> = Vec::new();

    let mut occurance_count: HashMap<u32, u32> = HashMap::new();

    for result in rdr.records() {
        match result {
            Ok(record) => {
                let value1 = record[0].parse::<u32>().unwrap();
                let value2 = record[1].parse::<u32>().unwrap();
                list1.push(value1);
                list2.push(value2);

                let count = occurance_count.entry(value2).or_insert(0);
                *count += 1;
            }
            Err(err) => {
                println!("error reading CSV: {}", err);
                return None
            }
        }
    }

    list1.sort();
    list2.sort();

    let distances = find_distance(&list1, &list2);
    assert_eq!(list1.len(), distances.len());
    let final_distance = get_sum_distances(&distances);

    return Some(final_distance);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rdr = csv::ReaderBuilder::new()
                                    .delimiter(b';')
                                    .has_headers(false)
                                    .from_reader(input.as_bytes());

    let mut list1:Vec<u32> = Vec::new();
    let mut list2:Vec<u32> = Vec::new();

    let mut occurance_count: HashMap<u32, u32> = HashMap::new();

    for result in rdr.records() {
        match result {
            Ok(record) => {
                let value1 = record[0].parse::<u32>().unwrap();
                let value2 = record[1].parse::<u32>().unwrap();
                list1.push(value1);
                list2.push(value2);

                let count = occurance_count.entry(value2).or_insert(0);
                *count += 1;
            }
            Err(err) => {
                println!("error reading CSV: {}", err);
                return None
            }
        }
    }

    list1.sort();
    list2.sort();

    let distances = find_distance(&list1, &list2);
    assert_eq!(list1.len(), distances.len());

    let similarity = calc_simularity(&list1, &occurance_count);


    return Some(similarity);
}

pub fn find_distance(list1: &Vec<u32>, list2: &Vec<u32>) -> Vec<u32> {
    let mut result = Vec::<u32>::new();

    for index in 0..list1.len() {
        result.push(u32::abs_diff(list1[index], list2[index]));
    }
    return result
}

pub fn get_sum_distances(distance_list: &Vec<u32>) -> u32 {
    let mut distance = 0;
    for item in distance_list {
        distance += item
    }
    return distance
}

pub fn calc_simularity(list1: &Vec<u32>, occurance_count: &HashMap<u32, u32>) -> u32{
    let mut result = 0;

    for item in list1 {
        result += item * occurance_count.get(item).unwrap_or(&0);
    }

    return result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
