fn main() {
    let input = include_str!("../input.txt");
    let parsed_input = parse_input(input);

    let safe_reports = part_1(parsed_input);
    println!("Safe reports: {:?}", safe_reports);
}

#[derive(PartialEq)]
enum Direction {
    Ascending,
    Descending
}

fn part_1(reports: Vec<Vec<i32>>) -> i32 {
    let mut total_safe_reports = 0;

    for report in reports {
        let mut is_safe = true;
        let mut report_direction: Option<Direction> = None;

        for (i, r) in report.clone().into_iter().enumerate() {
            let next = report.get(i + 1);
            if next.is_none() { break; };

            let next_direction = if (next.unwrap() - r).is_positive() { Direction::Ascending } else { Direction::Descending };

            match report_direction {
                Some(ref d) => if *d != next_direction {
                    is_safe = false;
                    break;
                },
                None => report_direction = Some(next_direction),
            }

            let difference =  (next.unwrap() - r).abs();
            if difference > 3 || difference < 1 {
                is_safe = false;
                break;
            }
        }

        if is_safe { total_safe_reports += 1 };
    }

    return total_safe_reports;
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect()).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn parse_input_test() {
        assert_eq!(parse_input(TEST_INPUT), [
            [7,6,4,2,1],
            [1,2,7,8,9],
            [9,7,6,2,1],
            [1,3,2,4,5],
            [8,6,4,4,1],
            [1,3,6,7,9],
        ]);
    }

    #[test]
    fn test_part_1() {
        let parsed_input = parse_input(TEST_INPUT);
        assert_eq!(part_1(parsed_input), 2);
    }
}
