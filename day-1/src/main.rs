use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");

    let result_1 = part_one(input);
    println!("Total difference: {result_1}");

    let result_2 = part_two(input);
    println!("Total similarity score: {result_2}");
}

fn part_one(input: &str) -> i32 {
    let mut location_list_1 = Vec::new();
    let mut location_list_2 = Vec::new();

    input.lines().for_each(|x| {
        let loc_id_entry: Vec<&str> = x.split_whitespace().collect();
        let [first, second] = [loc_id_entry[0], loc_id_entry[1]];

        let first_loc_id: i32 = first.parse().expect("failed to parse");
        let second_loc_id: i32 = second.parse().expect("failed to parse");

        location_list_1.push(first_loc_id);
        location_list_2.push(second_loc_id);
    });

    location_list_1.sort();
    location_list_2.sort();

    assert!(location_list_1.len() == location_list_2.len());

    let list_len = location_list_1.len();

    let mut total_diff: i32 = 0;

    for i in 0..list_len {
        let id_difference = location_list_1[i] - location_list_2[i];
        total_diff += id_difference.abs();
    }

    return total_diff;
}

fn part_two(input: &str) -> i32 {
    let mut location_list_1 = Vec::new();

    let mut list_2_num_freq = HashMap::<i32, i32>::new();

    input.lines().for_each(|x| {
        let loc_id_entry: Vec<&str> = x.split_whitespace().collect();
        let [first, second] = [loc_id_entry[0], loc_id_entry[1]];

        let first_loc_id: i32 = first.parse().expect("failed to parse");
        let second_loc_id: i32 = second.parse().expect("failed to parse");

        location_list_1.push(first_loc_id);

        if list_2_num_freq.contains_key(&second_loc_id) {
            list_2_num_freq.entry(second_loc_id).and_modify(|v| *v += 1);
        } else {
            list_2_num_freq.insert(second_loc_id, 1);
        }
    });

    let mut total_diff = 0;
    location_list_1.iter().for_each(|x| {
        if list_2_num_freq.contains_key(x) {
           total_diff += list_2_num_freq.get(&x).unwrap() * x;
        }
    });

    return total_diff;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = "3   4
            4   3
            2   5
            1   3
            3   9
            3   3";
        let result = part_one(test_input);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_two() {
        let test_input = "3   4
            4   3
            2   5
            1   3
            3   9
            3   3";

        let result = part_two(test_input);
        assert_eq!(result, 31);
    }
}
