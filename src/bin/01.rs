advent_of_code::solution!(1);

fn get_two_columns(input: &str) -> (Vec<u64>, Vec<u64>) {
    
    let (a, b): (Vec<_>, Vec<_>) = input
        .trim()
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .unzip();

    let mut left: Vec<u64> = a.into_iter().map(|s| s.parse().unwrap()).collect();
    let mut right: Vec<u64> = b.into_iter().map(|s| s.parse().unwrap()).collect();

    left.sort();
    right.sort();

    (left, right)
}

pub fn part_one(input: &str) -> Option<u64> {
    
    let (left, right) = &get_two_columns(input);
    
    let sum: u64 = left
        .clone()
        .iter()
        .zip(right)
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    
    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
