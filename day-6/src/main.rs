use std::{collections::HashSet, time::Instant};

fn main() {
    let input = include_str!("../input.txt");

    let start = Instant::now();
    let result = part_one(input);
    let duration = start.elapsed();
    println!("Total visited: {}", result);
    println!("Time taken part one: {:?}", duration);

    let start = Instant::now();
    let result_2 = part_two(input);
    let duration_2 = start.elapsed();
    println!("Total obstacle options: {}", result_2);
    println!("Time taken part one: {:?}", duration_2);
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Grid<T> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn new(data: Vec<Vec<T>>) -> Self {
        let width = data.len();
        let height = data.get(0).unwrap().len();
        Self {
            data,
            width,
            height,
        }
    }

    fn get(&self, point: &Point) -> Option<T>
    where
        T: Clone,
    {
        if point.x < 0 || point.y < 0 {
            return None;
        }

        self.data
            .get(point.y as usize)?
            .get(point.x as usize)
            .cloned()
    }

    fn set(&mut self, point: &Point, val: T) -> Result<(), String> {
        if point.x < 0 || point.y < 0 {
            return Err(String::from("Point is out of bounds"));
        }
        let row = self.data.get_mut(point.y as usize).ok_or("No valid row");
        let col_val = row?.get_mut(point.x as usize).ok_or("No valid column")?;

        *col_val = val;
        Ok(())
    }

    fn find_first(&self, val: T) -> Option<Point>
    where
        T: PartialEq,
    {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.data[i][j] == val {
                    return Some(Point {
                        x: j as i32,
                        y: i as i32,
                    });
                }
            }
        }
        None
    }
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

fn parse_input(input: &str) -> (Grid<char>, Point) {
    let grid_data = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let grid = Grid::new(grid_data);
    let starting_pos: Point = grid
        .find_first('^')
        .expect("No starting position found in input");

    (grid, starting_pos)
}

fn part_one(input: &str) -> i32 {
    let (grid, starting_pos) = parse_input(input);
    simulate_guard_path(&grid, &starting_pos).unwrap().len() as i32
}

fn part_two(input: &str) -> i32 {
    let (mut grid, starting_pos) = parse_input(input);
    // Do first iteration to get visted points
    let visited = simulate_guard_path(&grid, &starting_pos).unwrap();
    let mut obstacle_points = HashSet::new();

    for point in visited {
        // Add obstacle to grid at point
        let prev_val = grid.get(&point).expect("Previous value did not exist");
        let _ = grid.set(&point, '#');

        let visited = simulate_guard_path(&grid, &starting_pos);

        // Change back
        let _ = grid.set(&point, prev_val);

        match visited {
            Ok(_) => continue,
            Err(_) => {
                obstacle_points.insert(point.clone());
            }
        };
    }

    obstacle_points.len() as i32
}

fn simulate_guard_path(grid: &Grid<char>, starting_pos: &Point) -> Result<HashSet<Point>, String> {
    let mut visited = HashSet::new();
    let mut visited_with_directions = HashSet::new();

    let mut point = starting_pos.clone();
    let starting_direction = Direction::Up;

    let mut direction = starting_direction.clone();

    point.move_in_direction(&direction);

    let mut current_char = grid.get(&point);

    while current_char.is_some() {
        if visited_with_directions.contains(&(point.clone(), direction.clone())) {
            return Err(String::from("Found a loop"));
        }

        visited_with_directions.insert((point.clone(), direction.clone()));
        visited.insert(point.clone());

        let mut next_point = point.clone();
        next_point.move_in_direction(&direction);
        let next_char = grid.get(&next_point);

        match next_char {
            Some(ch) => {
                if ch == '#' {
                    direction = rotate_direction_right(&direction);
                    continue;
                }
                point.move_in_direction(&direction);
                current_char = next_char;
            }
            None => break,
        }
    }

    Ok(visited)
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

    const TEST_INPUT_2: &str = ".#..
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
