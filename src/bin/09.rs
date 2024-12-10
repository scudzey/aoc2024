advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk_map: Vec<u32> = input
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .enumerate()
        .flat_map(|(i, value)| {
            if (i % 2) == 0 {
                std::iter::repeat((i / 2) as u32)
                    .take(value as usize)
                    .collect::<Vec<_>>()
            } else {
                std::iter::repeat(u32::MAX)
                    .take(value as usize)
                    .collect::<Vec<_>>()
            }
        })
        .collect();
    let mut left = 0;
    let mut right = disk_map.len() - 1;

    while left < right {
        while left < disk_map.len() && disk_map[left] != u32::MAX {
            left += 1;
        }

        while right > 0 && disk_map[right] == u32::MAX {
            right -= 1;
        }

        if left < right {
            disk_map.swap(left, right);
        }
    }
    Some(
        disk_map
            .iter()
            .filter_map(|&ch| {
                if ch == u32::MAX {
                    return None;
                }
                Some(ch as u64)
            })
            .enumerate()
            .map(|(i, ch)| ch * (i as u64))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk_map: Vec<u32> = input
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .enumerate()
        .flat_map(|(i, value)| {
            if (i % 2) == 0 {
                std::iter::repeat((i / 2) as u32)
                    .take(value as usize)
                    .collect::<Vec<_>>()
            } else {
                std::iter::repeat(u32::MAX)
                    .take(value as usize)
                    .collect::<Vec<_>>()
            }
        })
        .collect();

    let mut i = disk_map.len();
    while i > 0 {
        i -= 1;

        if disk_map[i] == u32::MAX {
            continue;
        }

        let value = disk_map[i];
        let mut start = i;

        while start > 0 && disk_map[start - 1] == value {
            start -= 1;
        }
        let block_length = i - start + 1;

        let mut found_space = None;
        for j in 0..=disk_map.len() - block_length {
            if j + block_length <= start
                && disk_map[j..j + block_length]
                    .iter()
                    .all(|&ele| ele == u32::MAX)
            {
                found_space = Some(j);
                break;
            }
        }

        if let Some(space_start) = found_space {
            for item in disk_map.iter_mut().take(i + 1).skip(start) {
                *item = u32::MAX;
            }
            // for k in start..=i {
            //     disk_map[k] = u32::MAX;
            // }

            for k in 0..block_length {
                disk_map[space_start + k] = value;
            }
        }

        i = start;
    }

    Some(
        disk_map
            .iter()
            .enumerate()
            .map(|(i, ch)| {
                if *ch == u32::MAX {
                    return 0;
                }
                *ch as u64 * (i as u64)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
