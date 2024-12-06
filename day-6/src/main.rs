use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let (starting_pos, grid) = parse_input(input);

    let result = part_one(starting_pos, grid);
    println!("Total visited: {}", result);
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_in_direction(&mut self, d: &Direction) {
        match d {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
        }
    }
}

fn parse_input(input: &str) -> (Point, Vec<Vec<char>>) {
    let grid = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut starting_pos: Option<Point> = None;

    // Look, i know.. but im tired and been doing too many puzzles to have a brain left for this
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '^' {
                starting_pos = Some(Point { x: j as i32, y: i as i32 });
            }
        }
    }

    (starting_pos.unwrap(), grid)
}

fn part_one(starting_pos: Point, grid: Vec<Vec<char>>) -> i32 {
    let mut visited = HashSet::new();

    let mut point = starting_pos;
    let mut direction = Direction::Up;

    point.move_in_direction(&direction);

    let mut current_char = get_grid_value(&grid, &point);

    while current_char.is_some() {
        visited.insert(point.clone());

        let mut next_point = point.clone();
        next_point.move_in_direction(&direction);
        let next_char = get_grid_value(&grid, &next_point);

        match next_char {
            Some(ch) => {
                if ch == '#' {
                    println!("Encountered object at next! {:?}", point);
                    direction = rotate_direction_right(&direction);
                    println!("Rotated direction! {:?}", &direction);
                    point.move_in_direction(&direction);
                    current_char = get_grid_value(&grid, &point);
                } else {
                    println!("Keeping same direction! {:?}", point);
                    // continue in current direction
                    point.move_in_direction(&direction);
                    current_char = next_char;
                }
            },
            None => {
                // Set next char to current and it'll bail next iter
                break;
            }
        }

    }

    visited.len() as i32
}

fn get_grid_value(grid: &Vec<Vec<char>>, point: &Point) -> Option<char> {
    if point.x < 0 || point.y < 0 {
        return None;
    }

    grid.get(point.y as usize)?.get(point.x as usize).copied()
}

fn rotate_direction_right(d: &Direction) -> Direction {
    match d {
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_one() {
        let (sp, grid) = parse_input(TEST_INPUT);
        assert_eq!(part_one(sp, grid), 41)
    }

    #[test]
    fn test_part_two() {}
}
