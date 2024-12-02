use std::cmp::Ordering;

fn main() {
    let input = include_str!("../input.txt");
    let parsed_input = parse_input(input);

    let safe_reports = part_1(&parsed_input);
    println!("Safe reports: {:?}", safe_reports);

    let safe_reports_with_dampener = part_2(&parsed_input);
    println!(
        "Safe reports with Problem Dampener: {:?}",
        safe_reports_with_dampener
    );
}

#[derive(Debug)]
struct ReadingComparison {
    within_range: bool,
    direction: Ordering,
}

fn cmp_readings(x: i32, y: i32) -> ReadingComparison {
    let abs_diff = (x - y).abs();
    let direction = x.cmp(&y);

    return ReadingComparison {
        within_range: (1..=3).contains(&abs_diff),
        direction,
    };
}

fn is_unsafe(report: &Vec<i32>) -> bool {
    assert!(report.len() > 2);
    let report_direction = report[0].cmp(&report[1]);
    return report.windows(2).any(|window| {
        let (prev, curr) = (window[0], window[1]);
        let comparison = cmp_readings(prev, curr);
        match report_direction {
            std::cmp::Ordering::Equal => return true, // This is unsafe reading
            _ => return report_direction != comparison.direction || !comparison.within_range,
        }
    });
}

fn run_reports(reports: &Vec<Vec<i32>>, problem_dampener_enabled: bool) -> i32 {
    let mut total_safe_reports = 0;
    for report in reports {
        let mut report_is_unsafe = is_unsafe(&report);

        if report_is_unsafe && problem_dampener_enabled {
            for i in 0..report.len() {
                let mut report_clone = report.clone();
                report_clone.remove(i);

                report_is_unsafe = is_unsafe(&report_clone);
                if !report_is_unsafe {
                    break;
                }
            }
        }

        if !report_is_unsafe {
            total_safe_reports += 1;
        }
    }
    return total_safe_reports;
}

fn part_1(reports: &Vec<Vec<i32>>) -> i32 {
    run_reports(reports, false)
}

fn part_2(reports: &Vec<Vec<i32>>) -> i32 {
    run_reports(reports, true)
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
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

    const EDGE_CASE_INPUT: &str = "48 46 47 49 51 54 56
1 1 2 3 4 5
1 2 3 4 5 5
5 1 2 3 4 5
1 4 3 2 1
1 6 7 8 9
1 2 3 4 3
9 8 7 6 7";

    #[test]
    fn parse_input_test() {
        assert_eq!(
            parse_input(TEST_INPUT),
            [
                [7, 6, 4, 2, 1],
                [1, 2, 7, 8, 9],
                [9, 7, 6, 2, 1],
                [1, 3, 2, 4, 5],
                [8, 6, 4, 4, 1],
                [1, 3, 6, 7, 9],
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let parsed_input = parse_input(TEST_INPUT);
        assert_eq!(part_1(&parsed_input), 2);
    }

    #[test]
    fn test_part_2() {
        let parsed_input = parse_input(TEST_INPUT);
        assert_eq!(part_2(&parsed_input), 4);

        let edge = parse_input(EDGE_CASE_INPUT);
        assert_eq!(part_2(&edge), 8);
    }
}
