use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let result = part_one(input);
    println!("Total visited: {}", result);

    let result_2 = part_two(input);
    println!("Total obstacle options: {}", result_2);
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
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

fn part_one(input: &str) -> i32 {
    let (starting_pos, grid) = parse_input(input);
    simulate_guard_path(starting_pos, &grid).unwrap().len() as i32
}

fn part_two(input: &str) -> i32 {
    let (starting_pos, mut grid) = parse_input(input);
    // Do first iteration to get visted points
    let visited = simulate_guard_path(starting_pos.clone(), &grid).unwrap();
    let mut obstacle_points = HashSet::new();

    for point in visited {
        // Add obstacle to grid at point
        let prev_val = grid[point.y as usize][point.x as usize];
        grid[point.y as usize][point.x as usize] = '#';
        let visited = simulate_guard_path(starting_pos.clone(), &grid);

        // Change back
        grid[point.y as usize][point.x as usize] = prev_val;

        match visited {
            Ok(_) => continue,
            Err(_) => { obstacle_points.insert(point.clone()); },
        };

    }

    obstacle_points.len() as i32
}

fn simulate_guard_path(starting_pos: Point, grid: &Vec<Vec<char>>) -> Result<HashSet<Point>, String>  {
    let mut visited = HashSet::new();
    let mut visited_with_directions = HashSet::new();

    let mut point = starting_pos.clone();
    let starting_direction = Direction::Up;

    let mut direction = starting_direction.clone();

    point.move_in_direction(&direction);

    let mut current_char = get_grid_value(&grid, &point);

    while current_char.is_some() {
        if visited_with_directions.contains(&(point.clone(), direction.clone())) {
            return Err(String::from("LOOP"));
        }

        visited_with_directions.insert((point.clone(), direction.clone()));
        visited.insert(point.clone());

        let mut next_point = point.clone();
        next_point.move_in_direction(&direction);
        let next_char = get_grid_value(&grid, &next_point);

        match next_char {
            Some(ch) => {
                if ch == '#' {
                    // Rotate direction right & move in that direction
                    direction = rotate_direction_right(&direction);

                    // This was a problem! Don't turn and move, turn OR move to let the loop
                    // check if the turn gets stuck too
                    //
                    // point.move_in_direction(&direction);
                    // current_char = get_grid_value(&grid, &point);
                    continue;
                }
                // Continue in current direction
                point.move_in_direction(&direction);
                current_char = next_char;
            },
            None => break
        }

    }

    Ok(visited)
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

    const TEST_INPUT_2: &str =".#..
#..#
....
^...
#...
.#..";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 41)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 6);
        assert_eq!(part_two(TEST_INPUT_2), 1);
    }
}
