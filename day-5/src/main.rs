use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("../input.txt");
    let result = part_1(input);
    let result_2 = part_2(input);
    println!("Total of middle ordered upgrades: {}", result.unwrap());
    println!("Total of middle re-ordered upgrades: {}", result_2.unwrap());
}

fn parse_input(input: &str) -> Result<(HashMap<i32, Vec<i32>>, Vec<Vec<i32>>), String> {
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let top = split_input.get(0).ok_or("Missing top value")?;
    let bottom = split_input.get(1).ok_or("Missing bottom value")?;

    let order_rules: HashMap<i32, Vec<i32>> = top.split('\n').fold(HashMap::new(), |mut map, l| {
        let pair: Vec<i32> = l.split('|').map(|x| x.parse().unwrap()).collect();
        map.entry(pair[1]).or_insert_with(Vec::new).push(pair[0]);
        map
    });
    let upgrades: Vec<Vec<i32>> = bottom
        .split('\n')
        .map(|l| {
            l.split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    Ok((order_rules, upgrades))
}

fn part_1(input: &str) -> Result<i32, String> {
    let mut total = 0;
    let (order_rules, upgrades) = parse_input(input)?;

    // Iterate through upgrades
    for upgrade in &upgrades {
        let upgrade_is_ordered = upgrade.iter().all(|&p| page_is_ordered(p, upgrade, &order_rules));
        if upgrade_is_ordered {
            total += upgrade[upgrade.len() / 2];
        }
    }

    Ok(total)
}

fn page_is_ordered(page: i32, upgrade: &[i32], order_rules: &HashMap<i32, Vec<i32>>) -> bool {
    if let Some(pages) = order_rules.get(&page) {
        if let Some(page_pos) = upgrade.iter().position(|&x| x == page) {
            for p in pages {
                if let Some(dep_page_pos) = upgrade.iter().position(|&y| y == *p) {
                    if dep_page_pos > page_pos {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn part_2(input: &str) -> Result<i32, String> {
    let mut total = 0;
    let (order_rules, upgrades) = parse_input(input)?;

    // Iterate through upgrades
    let unordered: Vec<Vec<i32>> = upgrades.into_iter().filter(|u| u.iter().any(|&p| !page_is_ordered(p, u, &order_rules))).collect();
    let sort_unordered: Vec<Vec<i32>>= unordered.into_iter().map(|mut upgrade| {
        upgrade.sort_by(|a, b| {
            if let Some(pages) = order_rules.get(a) {
                if pages.contains(b) {
                    return Ordering::Greater
                }
            }
            return Ordering::Less
        });
        upgrade
    }).collect();

    for upgrade in sort_unordered {
        let upgrade_is_ordered = upgrade.iter().all(|&p| page_is_ordered(p, &upgrade, &order_rules));
        if upgrade_is_ordered {
            total += upgrade[upgrade.len() / 2];
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), Ok(143))
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), Ok(123));
    }
}
