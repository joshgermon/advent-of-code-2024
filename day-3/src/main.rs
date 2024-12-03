use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("Total multiplied: {:?}", part_1(input));
    println!("Total multiplied with start & stop instructions: {:?}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let total_multiplied = re.captures_iter(input).map(|x| { (x[1].to_string(), x[2].to_string()) }).fold(0, |acc, (x, y)| {
        (x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap()) + acc
    });

    return total_multiplied;
}

fn part_2(input: &str) -> i32 {
    // I'm not sure why /ms flags don't make this unnecessary but alas
    let mut clean_input = input.to_string();
    clean_input.retain(|c| c != '\r' && c != '\n');

    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let enabled_re = Regex::new(r"(?ms)^(.*?)don't\(\)|do\(\)(.*?)(?:don't\(\)|$)").unwrap();

    let enabled_chunks: Vec<String> = enabled_re.captures_iter(&clean_input).into_iter().map(|x| {
        x[0].to_string()
    }).collect();

    let enabled_input = &enabled_chunks.join("");
    let total_multiplied = mul_re.captures_iter(&enabled_input).map(|x| { (x[1].to_string(), x[2].to_string()) }).fold(0, |acc, (x, y)| {
        (x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap()) + acc
    });

    return total_multiplied;
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 161)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT_2), 48)
    }
}


