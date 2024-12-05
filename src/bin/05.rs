use std::collections::{HashMap, HashSet, VecDeque};


advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut print_jobs: Vec<Vec<u32>> = vec![];

    let _lines : Vec<&str> = input
        .lines()
        .map(|line| {
            if line.contains('|') {
                let page_rule: Vec<u32>= line.split("|").filter_map(|ele| {
                    ele.parse::<u32>().ok()
                }).collect();

                if page_rule.len() > 1 {
                    let rule: &mut Vec<u32> = rules.entry(page_rule[0]).or_insert(vec![]);
                    rule.push(page_rule[1]);
                }
                return line;
            }
            if line.contains(',') {
                print_jobs.push(line
                    .split(',')
                    .filter_map(|ele| ele.parse::<u32>().ok())
                    .collect());
                return line;
            }
            line
        }).collect();

    let mut final_sum: u32 = 0;

    let _ = print_jobs.into_iter().for_each(|job| {
        let mut pages_processed: HashSet<u32> = HashSet::new();
        let mut is_valid = true;
        job.clone().into_iter().for_each(|ele| {
            pages_processed.insert(ele);
            if let Some(rule) = rules.get(&ele) {
                let exists = rule.iter().any(|page| pages_processed.contains(page));
                if exists {
                    is_valid = false;
                }
            }
        });

        if is_valid {
            final_sum += job[job.len() / 2];
        }

    });
    Some(final_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rules: Vec<(u32,u32)> = vec![];
    let mut print_jobs: Vec<Vec<u32>> = vec![];

    let _lines : Vec<&str> = input
        .lines()
        .map(|line| {
            if line.contains('|') {
                let page_rule: Vec<u32>= line.split("|").filter_map(|ele| {
                    ele.parse::<u32>().ok()
                }).collect();

                if page_rule.len() > 1 {
                    rules.push((page_rule[0], page_rule[1]));
                }
                return line;
            }
            if line.contains(',') {
                print_jobs.push(line
                    .split(',')
                    .filter_map(|ele: &str| ele.parse::<u32>().ok())
                    .collect());
                return line;
            }
            line
        }).collect();

    let mut final_sum: u32 = 0;



    let _ = print_jobs.into_iter().for_each(|job| {



        let original_job = job.clone();


        let mut filtered_rules: Vec<(u32, u32)> = rules.clone()
                                                    .into_iter()
                                                    .filter(|(x, y)| {
                                                        let this_job = job.clone();
                                                        this_job.contains(x) && this_job.contains(y)
                                                    })
                                                    .collect();

        let current_job = order_by_rules(job, &filtered_rules).unwrap_or_else(|| original_job.clone());

        if original_job != current_job {
            final_sum += current_job[current_job.len() / 2];
        }


    });
    Some(final_sum)
}

pub fn order_by_rules(mut items: Vec<u32>, rules: &Vec<(u32,u32)>) -> Option<Vec<u32>> {
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, u32> = HashMap::new();

    for &(x, y) in rules {
        graph.entry(x).or_default().push(y);
        *in_degree.entry(y).or_insert(0) += 1;
        in_degree.entry(x).or_insert(0);
    }

    let unrelated: Vec<u32> = items.iter().filter(|&&x| !in_degree.contains_key(&x)).cloned().collect();

    let mut queue: VecDeque<u32> = in_degree.iter().filter(|&(_, &deg)| deg == 0).map(|(&k, _)| k).collect();
    let mut result = Vec::new();

    while let Some(node) = queue.pop_front() {
        result.push(node);
        if let Some(neighbors) = graph.remove(&node) {
            for &neighbor in &neighbors {
                if let Some(deg) = in_degree.get_mut(&neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    // Check if we used all nodes
    if result.len() != in_degree.len() {
        return None; // Cycle detected
    }

    // Add unordered elements at the end, maintaining their original order
    result.extend(unrelated.into_iter());

    // Reorder the original list based on the computed order
    items.sort_by_key(|x| result.iter().position(|y| y == x).unwrap_or(usize::MAX));
    Some(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
