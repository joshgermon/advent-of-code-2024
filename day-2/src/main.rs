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

// fn calculate_safe_reports(reports: &Vec<Vec<i32>>, problem_dampener: bool) -> i32 {
//     let mut total_safe_reports = 0;
//
//     for report in reports {
//         let mut is_safe = true;
//         let mut report_direction: Option<Direction> = None;
//
//         for (i, r) in report.clone().into_iter().enumerate() {
//             let mut unsafe_reading = false;
//
//             if let Some(next) = report.get(i + 1) {
//                 let comparison = compare_subsequent_readings(r, *next);
//                 if let Some(ref direction) = report_direction {
//                     if *direction != comparison.direction {
//                         unsafe_reading = true;
//                     }
//                 } else {
//                     report_direction = Some(comparison.direction)
//                 }
//
//                 if !comparison.within_range {
//                     unsafe_reading = true;
//                 }
//
//                 if unsafe_reading {
//                     if !problem_dampener || i == 0 {
//                         is_safe = false;
//                         break;
//                     }
//
//                     if let Some(prev) = report.get(i - 1) {
//                         let prev_comparison = compare_subsequent_readings(*prev, *next);
//                         if !prev_comparison.within_range
//                             || report_direction.clone().unwrap() != prev_comparison.direction
//                         {
//                             is_safe = false;
//                             break;
//                         }
//                     }
//                 }
//             }
//         }
//
//         if is_safe {
//             total_safe_reports += 1
//         };
//     }
//
//     return total_safe_reports;
// }

fn compare_subsequent_readings(x: i32, y: i32) -> ReadingComparison {
    let abs_diff = (x - y).abs();
    let direction = x.cmp(&y);

    return ReadingComparison {
        within_range: (1..=3).contains(&abs_diff),
        direction,
    };
}

fn calc_reports(reports: &Vec<Vec<i32>>, problem_dampener: bool) -> i32 {
    let mut total_safe_reports = 0;
    for report in reports {
        let report_direction = report[0].cmp(&report[1]);
        let has_unsafe_reading = report.windows(2).any(|window| {
            let (prev, curr) = (window[0], window[1]);
            let comparison = compare_subsequent_readings(prev, curr);
            match report_direction {
                std::cmp::Ordering::Equal => return true, // This is unsafe reading
                _ => return report_direction != comparison.direction || !comparison.within_range,
            }
        });

        if has_unsafe_reading && problem_dampener {
            for i in 0..report.len() {
                let mut brute_force = report.clone();
                brute_force.remove(i);

                let brute_force_direction = brute_force[0].cmp(&brute_force[1]);
                let pd_unsafe_reading = brute_force.windows(2).any(|window| {
                    let (prev, curr) = (window[0], window[1]);
                    let comparison = compare_subsequent_readings(prev, curr);
                    match brute_force_direction {
                        std::cmp::Ordering::Equal => return true, // This is unsafe reading
                        _ => {
                            return brute_force_direction != comparison.direction
                                || !comparison.within_range
                        }
                    }
                });

                if !pd_unsafe_reading {
                    total_safe_reports += 1;
                    break;
                }
            }
        }

        if !has_unsafe_reading {
            total_safe_reports += 1;
        }
    }
    return total_safe_reports;
}

fn part_1(reports: &Vec<Vec<i32>>) -> i32 {
    calc_reports(reports, false)
}

fn part_2(reports: &Vec<Vec<i32>>) -> i32 {
    calc_reports(reports, true)
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
