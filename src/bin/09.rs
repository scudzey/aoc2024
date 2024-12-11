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

    let mut available_spaces = Vec::new();
    let mut i = 0;
    while i < disk_map.len() {
        if disk_map[i] == u32::MAX {
            let start = i;
            while i < disk_map.len() && disk_map[i] == u32::MAX {
                i += 1;
            }
            available_spaces.push((start, i - start));
        } else {
            i += 1
        }
    }
    let mut free_spaces = Vec::new();
    let mut i = 0;

    // Precompute contiguous free spaces
    while i < disk_map.len() {
        if disk_map[i] == u32::MAX {
            let start = i;
            while i < disk_map.len() && disk_map[i] == u32::MAX {
                i += 1;
            }
            free_spaces.push((start, i - start));
        } else {
            i += 1;
        }
    }

    let mut block_end = disk_map.len();
    while block_end > 0 {
        block_end -= 1;

        if disk_map[block_end] == u32::MAX {
            continue;
        }

        let value = disk_map[block_end];
        let mut block_start = block_end;
        while block_start > 0 && disk_map[block_start - 1] == value {
            block_start -= 1;
        }
        let block_length = block_end - block_start + 1;

        let mut found_space = None;
        for (space_idx, &(space_start, space_length)) in free_spaces.iter().enumerate() {
            if space_length >= block_length && space_start + space_length <= block_start {
                found_space = Some((space_idx, space_start));
                break;
            }
        }

        if let Some((space_idx, space_start)) = found_space {
            for k in 0..block_length {
                disk_map[space_start + k] = value;
                disk_map[block_start + k] = u32::MAX;
            }

            let (old_start, old_length) = free_spaces[space_idx];
            if old_length == block_length {
                free_spaces.remove(space_idx);
            } else {
                free_spaces[space_idx] = (old_start + block_length, old_length - block_length);
            }
        }
        block_end = block_start;
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
