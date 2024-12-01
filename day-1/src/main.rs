use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let (location_list_1, location_list_2) = parse_input(input).expect("failed to parse input");

    let result_1 = part_one(&location_list_1, &location_list_2);
    println!("Total difference: {result_1}");

    let result_2 = part_two(&location_list_1, &location_list_2);
    println!("Total similarity score: {result_2}");
}

fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<i32>), String> {
    let (mut list_1, mut list_2): (Vec<i32>, Vec<i32>) = input.lines().map(|line| {
        let nums = line.split_whitespace().into_iter().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
        (nums[0], nums[1])
    }).unzip();

    list_1.sort();
    list_2.sort();

    return Ok((list_1, list_2));
}


fn part_one(list_1: &Vec<i32>, list_2: &Vec<i32>) -> i32 {
    return list_1.iter().zip(list_2.iter()).fold(0, |acc, (x, y)| { (x - y).abs() + acc });
}

fn part_two(list_1: &Vec<i32>, list_2: &Vec<i32>) -> i32 {
    let mut freq_map = HashMap::with_capacity(list_1.len());

    for num in list_2 {
        let freq = freq_map.entry(num).or_insert(0);
        *freq += 1;
    }

    return list_1.iter().map(|x| (x * freq_map.get(&x).unwrap_or(&0))).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parse() {
        let test_input = "3   4
            4   3
            2   5
            1   3
            3   9
            3   3";
        let result = parse_input(test_input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (vec![1, 2, 3, 3, 3, 4], vec![3, 3, 3, 4, 5, 9]));
    }

    #[test]
    fn test_part_one() {
        let list_1 = vec![1, 2, 3, 3, 3, 4];
        let list_2 = vec![3, 3, 3, 4, 5, 9];
        let result = part_one(&list_1, &list_2);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_two() {
        let list_1 = vec![1, 2, 3, 3, 3, 4];
        let list_2 = vec![3, 3, 3, 4, 5, 9];

        let result = part_two(&list_1, &list_2);
        assert_eq!(result, 31);
    }
}

